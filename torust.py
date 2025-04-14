import json
import subprocess
import os
import shutil
import traceback

import anthropic
import argparse

from config import bolero_timeout, all_timeout
from generate_utils import GenerateUtils, VerificationUtils
from llm import LLMQuery, LLMQueryInput, Conversation, make_cached_llm

generate_utils = GenerateUtils()
verification_utils = VerificationUtils()
# anthropic_client = anthropic.Client(os.environ["ANTHROPIC_API_KEY"])


helper_funcs = """
fn min(x: i32, y: i32) -> i32 {if x < y { x } else { y } }
fn max(x: i32, y: i32) -> i32 {if x > y { x } else { y }}
fn cmpfunc(a: &i32, b: &i32) -> std::cmp::Ordering {a.cmp(b)}
fn len(arr: &[i32]) -> usize {arr.len()}
fn sort(arr: &mut [i32]) {arr.sort_by(cmpfunc);}
"""


def generate(input_text, tokenizer, model, device):
    inputs = tokenizer.encode(input_text, return_tensors="pt").to(device)
    outputs = model.generate(
        inputs,
        do_sample=True,
        top_k=0,
        top_p=0.9,
        temperature=0.2,
        max_new_tokens=500,
        pad_token_id=tokenizer.eos_token_id,
    )
    generated_code = tokenizer.decode(outputs[0])
    return generated_code


def parse_tag_content(content: str, tag: str) -> str:
    tag_open = lambda _tag: f"<{_tag}>"
    tag_close = lambda _tag: f"</{_tag}>"
    return content.split(tag_open(tag), 1)[1].split(tag_close(tag))[0]


def clean_generated_code(generated_code):
    generated_code = parse_tag_content(generated_code, "rust-code")
    generated_code = generated_code.replace("pub fn", "fn")
    # 1. only keep first generated function
    # generated_code = generate_utils.keep_first_func(generated_code)
    # 2. remove comments
    generated_code = generate_utils.remove_comments(generated_code)
    # 3. close unclosed brackets
    cleaned_code = generate_utils.bracket_adder(generated_code)
    return cleaned_code


def claude_gen(input, max_tokens_to_sample: int = 2000):
    resp = anthropic_client.completion(
        prompt=f"""{anthropic.HUMAN_PROMPT} {input}  
        {anthropic.AI_PROMPT}""",
        stop_sequences=[anthropic.HUMAN_PROMPT],
        model="claude-v1",
        max_tokens_to_sample=max_tokens_to_sample,
    )
    answer = resp["completion"]
    if answer.startswith(" "):
        answer = answer[1:]
    return answer


def claude(
    llm: LLMQuery,
    source_code,
    package_name,
    rust_dir,
    file_name,
    number_tries,
    file_ext,
):
    language = ""
    if "cpp" in file_ext:
        language = "++"
    file_path = f"{rust_dir}/src/translation.rs"
    system = "You are an expert programmer who helps rewrite code to Rust."
    lang = file_ext[1:].upper()
    prompt = f"""
<source-code>
{source_code}
</source-code>

<instructions>
In the <source-code> tags you are given {lang} code with a function f_gold. Please provide a C-like translation to Rust.
You are given the following rules, which should be _strictly_ followed:
- Name the Rust function f_gold
- The Rust f_gold must have the same number of input arguments as the {lang} f_gold
- Always map int types in f_gold to i32 in Rust
- Include all necessary imports
- Use safe Rust
- Do not give a main function
- Do not comment the code
- Enclose the code in <rust-code> tags

Before writing the translation do the following:
- copy the function signature of f_gold in the <source-code> tags
- state the number of input arguments to f_gold
- If the output of the original program is bool, return 0 or 1 int (-> i32) output.
- state how you will map the input and return types in f_gold to Rust
- state the Rust signature of the translated f_gold
- For GO lang and str inputs, use str: &str instead of str: &[u8]
</instructions>
    """
    # rust_input = (
    #     source_code
    #     + f"\nRust refactoring of above C{language} code, with code only, no comments. Use the same function name, "
    #       f"same argument and return types. Make sure it includes all imports, uses safe rust, and compiles. Give "
    #       f"only code, and no main function. Convert i32 types to f32 if necessary. Use mut variables if necessary. "
    #       f""
    # )

    compiles = False
    tries = 0
    while not compiles:
        tries += 1
        if tries > number_tries:
            break
        print(f"LLM attempt # {tries}")
        response = llm.query(
            LLMQueryInput.construct_llm_query_input(
                Conversation.new_convo(system, prompt), 0.2, ["</rust-code>"]
            )
        )
        # rust_output = claude_gen(rust_input)
        rust_output = clean_generated_code(response.Conversation.last_message())

        if not "fn main()" in rust_output:
            rust_output += "\nfn main(){}"
        if "fn max(" not in rust_output:
            rust_output = helper_funcs + "\n" + rust_output

        with open(file_path, "w") as file:
            file.write(rust_output)
        subprocess.run(f"chmod -R a+rw {rust_dir}", shell=True)
        rust_output, compiles = generate_utils.error_msg_repair(
            rust_output, package_name, rust_dir, file_name
        )

    rust_output = (
        rust_output.replace("i64", "i32")
        .replace("i8", "i32")
        .replace("f64", "f32")
        .replace("bool", "i32")
        .replace("true", "1")
        .replace("false", "0")
    )

    rust_output_split = rust_output.split("\n")
    for i, line in enumerate(rust_output_split):
        if " f_gold(" in line:
            rust_output_split[i] = (
                line.replace("u64", "i32")
                .replace("i64", "i32")
                .replace("i8", "i32")
                .replace("f64", "f32")
                .replace("bool", "i32")
                .replace("Vec<f32>", "[f32;2]")
                .replace("Vec<i32>", "[i32;2]")
                .replace("&[i32]", "[i32;2]")
                .replace("&[f32]", "[f32;2]")
                .replace("&mut", "i32")
                .replace("usize", "u32")
                .replace("[i32;10]", "[i32;2]")
                .replace("&[i32", "[i32")
            )
    rust_output = "\n".join(rust_output_split)

    rust_output = "////// LLM Output //////" + rust_output + "////// LLM Output //////"
    return (
        rust_output.replace("\nfn main(){}", "").replace("\nfn main(){\n}", ""),
        compiles,
    )


def dump_result(result_file, result):
    with open(result_file, "w") as fd:
        fd.write(json.dumps(result, indent="\t"))

def extract_param_bounds(code_string, arg_types):
    # Find the line with param0 declaration and determine type
    param_type = None
    for line in code_string.split('\n'):
        if 'param0[]' in line:
            # Determine the type from the declaration
            if 'double param0[]' in line :
                param_type = float
            elif 'int param0[]' in line or 'long param0[]' in line:
                param_type = int
            elif 'char param0[]' in line :
                param_type = str
                return param_type.__name__, None, None
            else:
                return None
            # Extract the values between curly braces
            values_str = line[line.find('{')+1:line.find('}')]
            # Convert string of values to list of appropriate type
            values = [param_type(x.strip()) for x in values_str.split(',')]
            # Sort the values
            sorted_values = sorted(values)
            n = len(sorted_values)
            q1_pos = n // 10
            q3_pos = n - (n // 5)
            middle_values = sorted_values[q1_pos:q3_pos]
            min_val = min(abs(x) for x in middle_values)
            max_val = max(middle_values)

            if param_type == float:
                if 'int' in arg_types:
                    min_val = int(min_val)
                    max_val = int(max_val)
                else:
                    min_val = round(min_val, 3)
                    max_val = round(max_val, 3)
            if param_type == int:
                min_val = int(min_val)
                max_val = int(max_val)

            return param_type.__name__, min_val, max_val
    return None  # Return None if param0[] is not found

def clean_main_rs(wasm_path: str):
    """Remove previous LLM output from main.rs file"""
    with open(wasm_path, 'r') as file:
        content = file.read()
    
    # Remove everything between and including the LLM Output markers
    while "////// LLM Output //////" in content:
        start = content.find("////// LLM Output //////")
        end = content.find("////// LLM Output //////", start + 1) + len("////// LLM Output //////")
        content = content[:start] + content[end:]
        
    while "////// bolero harness //////" in content:
        start = content.find("////// bolero harness //////")
        end = content.find("////// bolero harness //////", start + 1) + len("////// bolero harness //////")
        content = content[:start] + content[end:]
    while "////// kani harness //////" in content:
        start = content.find("////// kani harness //////")
        end = content.find("////// kani harness //////", start + 1) + len("////// kani harness //////")
        content = content[:start] + content[end:]
    while "////// wasm function //////" in content:
        start = content.find("////// wasm function //////")
        end = content.find("////// wasm function //////", start + 1) + len("////// wasm function //////")
        content = content[:start] + content[end:]
    while "static mut FETCH: bool = false;\n" in content:
        content = content.replace("static mut FETCH: bool = false;\n", "")
    
    with open(wasm_path, 'w') as file:
        file.write(content)

def main():
    ap = argparse.ArgumentParser(description="Evaluation for VERT")
    ap.add_argument(
        "--language",
        # required=True,
        choices=["c", "cpp", "go"],
        default="c",
        help="Choose source language to compile to Rust: c, cpp, or go",
    )
    ap.add_argument("--benchmark-dir", default="benchmark/c_transcoder/ADD_1_TO_A_GIVEN_NUMBER", help="Path to benchmark")
    ap.add_argument(
        "--aws-profile",
        default="default",
        help="AWS profile to use for credentials",
    )
    ap.add_argument(
        "--replay-cache",
        action="store_true",
        help="Replay cached llm responses. Assumes a directory prompt_log/ exists under the specified --benchmark-dir"
    )
    ap.add_argument(
        "--llm-attempts",
        default=2,
        help="Number of LLM attempts when when pbt or verification fails first try"
    )
    

    args = ap.parse_args()
    language = args.language
    # home_dir = os.getcwd()
    # file_dir = f"{home_dir}/benchmark/{language}_transcoder"
    rust_dir = f"{args.benchmark_dir}/translation"

    ###################################### Controls which portion to run ######################################
    use_claude = 1  # Generate LLM transpilation
    bolero = 1  # Bolero verification
    bounded_kani = 1  # Bounded Kani verification
    number_tries = 5  # Number of tries for LLM
    ####################################################################################################

    subdir = args.benchmark_dir
    package_name = subdir.split("/")[-1]
    file = package_name + "." + language

    file_path = os.path.join(subdir, file)

    rust_compiles = True
    bolero_successful = True
    kani_successful = True

    wasm_bolero_main = f"{args.benchmark_dir}/out-rwasm-bolero/src/main.rs"
    wasm_kani_main = f"{args.benchmark_dir}/out-rwasm-mutated/src/main.rs"

    result = {
        "project": package_name,
        "compile": False,
        "bolero": False,
        "bounded_kani": False,
    }
    result_file = f"{args.benchmark_dir}/result.json"
    if os.path.exists(result_file):
        os.remove(result_file)
    file_ext = f".{language}"

    file_name = file.replace(".go", "").replace(".cpp", "").replace(".c", "")

    f_filled = ""
    if ".go" in file_ext:
        c_ext = ".c"
        c_benchmark_dir = args.benchmark_dir.replace("go_transcoder", "c_transcoder")
    else:
        c_ext = file_ext
        c_benchmark_dir = args.benchmark_dir
    c_filepath = f"benchmark/c_transcoder/{file_name}/{file_name}.c"
    try:
        with open(c_filepath, "r") as cfile:
            c_output = cfile.read()
    except:
        c_filepath = f"benchmark/cpp_transcoder/{file_name}/{file_name}.cpp"
        with open(c_filepath, "r") as cfile:
            c_output = cfile.read()
    ###################################### 1. function identifiers ####################################
    (
        fn_name,
        args_types,
        args_names,
        fn_out_type,
        fn_line,
    ) = generate_utils.get_fn_args(c_output)
    f_filled = fn_line.replace("{", "{}").replace("f_gold", "f_filled")
    ####################################################################################################

    source_output, original_code = generate_utils.c_code_process(
        file_ext, args.benchmark_dir, file_name, f_filled, args_types
    )
    
    constraints = extract_param_bounds(original_code, args_types)
    if constraints:
        type_name, min_bound, max_bound = constraints
    
    

    ###################################### 2. set up wasm file #########################################
    cwasm_path = file_path.replace(file_ext, f"_towasm{file_ext}").replace("cpp", "c")
    try:
        rwasm_arg_types = verification_utils.mutate_test(
            args.benchmark_dir,
            package_name,
            cwasm_path,
            fn_name,
            args_types,
            file_ext,
            fn_out_type,
        )

    except Exception as e:
        print("Source file failed to compile:", e)
        traceback.print_exc()
        dump_result(result_file, result)
        return
    ####################################################################################################

    leetcode_name = "_".join(package_name.split("_")[1:])
    if "transcoder" in args.benchmark_dir:
        leetcode_name = package_name

    ###################################### 3. LLM ######################################################


    if use_claude:
        llm = make_cached_llm(f"{subdir}/prompt_log", args.replay_cache, args.aws_profile)
        llm_attempts = 0
        max_llm_attempts = int(args.llm_attempts)
        bolero_success = False

        while not bolero_success and llm_attempts <= max_llm_attempts:
            
            print(f"\nLLM+Bolero attempt #{llm_attempts}")
            llm_attempts += 1
            
            # Clean up previous LLM output from both main.rs files
            clean_main_rs(wasm_bolero_main)
            clean_main_rs(wasm_kani_main)
            generate_utils.build_rust_folder(rust_dir, leetcode_name)
            compiled_rust, rust_compiles = claude(
                llm,
                source_output,
                leetcode_name,
                rust_dir,
                file_name,
                number_tries,
                file_ext,
            )
            if not rust_compiles:
                print("LLM failed to produce compiling Rust")
                dump_result(result_file, result)
                return
            else:
                result["compile"] = True

            compiled_rust_fn_line = [l for l in compiled_rust.split("\n") if " f_gold" in l][0]
            ################################################################################################
            ###################################### 4. Harness ##############################################
            ########## 4.1 RWasm Init ############
            rust_args_types = str(args_types)[1:-1]
            rust_args_types = (
                rust_args_types.replace("unsigned int", "u32")
                .replace("int", "i32")
                .replace("float", "f32")
                .replace("i32 []", "[i32;2]")
                .replace("f32 []", "[f32;2]")
                .replace("char []", "[u8;2]")
                .replace("char", "u8")
                .replace("double", "f32")
                .replace("float", "f32")
                .replace("long", "i32")
                .replace("i32 [i32]", "[i32;2]")
                .replace("string", "String")
                .replace("&[f32;2]", "[i32;2]")
                .replace("[f32;2]", "[i32;2]")
            )
            rust_fn_out_type = (
                fn_out_type.replace("unsigned int", "u32")
                .replace("unsigned", "u32")
                .replace("int", "i32")
                .replace("i32 []", "Vec<i32>")
                .replace("double", "f32")
                .replace("float", "f32")
                .replace("long", "i32")
                .replace("string", "String")
            )
            if rust_fn_out_type == "i32" and "-> f32" in compiled_rust:
                compiled_rust = compiled_rust.replace("-> f32", "-> i32")
            if rust_fn_out_type == "i32" and "-> u32" in compiled_rust:
                compiled_rust = compiled_rust.replace("-> u32", "-> i32")

            wasm_fn_name = f"{fn_name}_wasm_thread_unsafe"
            wasm_function = f"\n////// wasm function //////\nfn {wasm_fn_name}() -> {rust_fn_out_type} {{\n\tlet mut wasm_module = WasmModule::new();\n\twasm_module._start();\n\tunsafe {{ RESULT }}\n}}\n////// wasm function //////\n\n"
            arg_string = ""
            bolero_argstring = ""
            bolero_arg_unsafe = "unsafe {\n"
            kani_arg_string = ""
            string_bolero_harness = ""
            string_ending_bracket = ""
            for i, arg_type in enumerate(args_types):
                if "[]" in arg_type:
                    general_arg_string = f"[unsafe{{PARAM{i+1}}}[0] as _, unsafe{{PARAM{i+1}}}[1] as _],"
                    if "&str" in compiled_rust_fn_line or "String" in compiled_rust_fn_line:
                        general_arg_string = general_arg_string[:-1] + ".iter().collect::<String>()," 
                        general_arg_string = general_arg_string.replace(" as _", " as char")
                        if "&str" in compiled_rust_fn_line:
                            general_arg_string = "&" + general_arg_string
                    arg_string +=  general_arg_string
                    kani_arg_string += general_arg_string
                    bolero_argstring += f"PARAM_{i+1},"
                    bolero_arg_unsafe += f"\t\tPARAM{i+1} = PARAM_{i+1};\n"
                elif "string" in arg_type:
                    string_bolero_harness = (
                        f"\t\tif let Some(param{i+1}_0) = PARAM_{i+1}.chars().nth(0){{\n"
                    )
                    string_ending_bracket = "}"
                    arg_string += f"unsafe{{PARAM{i+1}}}.into(),"
                    bolero_argstring += f"PARAM_{i+1},"
                    
                    bolero_arg_unsafe += f"\t\tPARAM{i+1} = param{i+1}_0;\n"
                    kani_arg_string += f"PARAM_{i+1}[0],"

                else:
                    arg_string += f"unsafe{{PARAM{i+1}}}.into(),"
                    kani_arg_string += f"unsafe{{PARAM{i+1}}}.into(),"
                    bolero_argstring += f"PARAM_{i+1},"
                    bolero_arg_unsafe += f"\t\tPARAM{i+1} = PARAM_{i+1};\n"

            arg_string = "(" + arg_string[:-1] + ")"
            kani_arg_string = "(" + kani_arg_string[:-1] + ")"
            bolero_argstring = "(" + bolero_argstring + ")"
            bolero_arg_unsafe += "\n\t\t}"
            ##########################################
            ########## 4.2 Bolero Harness ############

            bolero_import = "\nfn assert_eq(a: f64, b: f64) { assert!((a - b).abs() < 0.01); }\n#[test]"
            
            # Bolero input generator
            if constraints and min_bound and max_bound:
                inner_generator = ""
                for i, arg_type in enumerate(args_types):
                    subgen = f"{min_bound}..{max_bound}"
                    if "[]" in arg_type:
                        subgen = f"[{min_bound}..{max_bound}, {min_bound}..{max_bound}]"
                        if  "&str" in compiled_rust_fn_line or "String" in compiled_rust_fn_line:
                            subgen = "[('A' as u8)..('Z' as u8 + 1), ('A' as u8)..('Z' as u8 + 1)]"
                            if "to_digit(10)" in compiled_rust:
                                subgen = "[('0' as u8)..('9' as u8 + 1), ('0' as u8)..('9' as u8 + 1)]"
                    inner_generator += subgen + ","
                        
                generator = f"with_generator(({inner_generator}))"
            else:
                generator = f"with_type::<({rust_args_types},)>()"
                
            bolero_func_decl = f"\nfn bolero_wasm_eq(){{\n\tbolero::check!().{generator}.cloned().for_each(|{bolero_argstring}|{{ \n{string_bolero_harness}".replace(
                "'", ""
            )
            ## Convert to string if char [] type
            if constraints and type_name == str:
                bolero_func_body = f"\t\t{bolero_arg_unsafe}\n\t\tlet input_str = String::from_utf8_lossy(unsafe {{ &PARAM1 }}).into_owned();\n\t\tlet result = {fn_name}(&input_str);\n\t\tlet result_prime = {wasm_fn_name}();\n\t\tassert_eq(result as f64, result_prime as f64);\n\t{string_ending_bracket}}});\n}}"
            ###
            else:
                bolero_func_body = f"\t\t{bolero_arg_unsafe}\n\t\tlet result = {fn_name}{arg_string};\n\t\tlet result_prime = {wasm_fn_name}();\n\t\tassert_eq(result as f64, result_prime as f64);\n\t{string_ending_bracket}}});\n}}"
            final_bolero_harness = (
                "\n////// bolero harness //////" + "\n" + bolero_import + bolero_func_decl + bolero_func_body + "\n////// bolero harness //////\n"
            )
            ########################################
            ########## 4.3 Kani Harness ############

            kani_declare = "\nfn assert_eq(a: f64, b: f64) { assert!((a - b).abs() < 0.01); }#[cfg(kani)]\n#[kani::proof]\n#[kani::unwind(10)]"
            kani_func_decl = f"\nfn kani_wasm_eq(){{ \n"
            kani_func_body = f"\t\tlet result = {fn_name}{kani_arg_string};\n\t\tlet result_prime = {wasm_fn_name}();\n\t\tassert_eq(result as f64, result_prime as f64);\n}}"
            final_kani_harness = "\n////// kani harness //////" + "\n" + kani_declare + kani_func_decl + kani_func_body + "\n////// kani harness //////\n"
            #######################################
            
            bolero_output = wasm_function + compiled_rust + final_bolero_harness
            kani_output = wasm_function + compiled_rust + final_kani_harness


            if "String" in rust_fn_out_type:
                bolero_output = bolero_output.replace(
                    "unsafe { RESULT }", "unsafe { RESULT.to_string() }"
                )
                kani_output = kani_output.replace(
                    "unsafe { RESULT }", "unsafe { RESULT.to_string() }"
                )

            with open(wasm_bolero_main, "a") as wasmfile:
                wasmfile.write(bolero_output)
            with open(wasm_kani_main, "a") as wasmfile:
                wasmfile.write(kani_output)

            ##############################################################################################
            ###################################### 5. Verification ######################################

            wasm_bolero_path = f"{args.benchmark_dir}/out-rwasm-bolero/src"
            wasm_kani_path = f"{args.benchmark_dir}/out-rwasm-mutated/src"

            bolero_target_path = wasm_bolero_path + "/target"
            kani_target_path = wasm_kani_path.replace("/src", "/target")
            ##########################################################################################
            ###################################### BOLERO ############################################
            if bolero and rust_compiles:
                print("Running bolero")
                command = f'RUSTFLAGS="-C overflow-checks=false" cargo bolero test -T {bolero_timeout}s -S 0 bolero_wasm_eq'
                verification_output, timeout = verification_utils.verify(
                    wasm_bolero_path,
                    command,
                    f"{subdir}/bolero_out.txt",
                    f"{subdir}/bolero_err.txt",
                    500,
                )
                if not timeout:
                    err_message = verification_output.stderr
                    stdout_message = verification_output.stdout

                    if "could not compile" in err_message:
                        print("Bolero compilation problem")
                        print(err_message)
                        # dump_result(result_file, result)
                        # return
                    elif (
                        "Test Failure" in err_message
                        or "Test Failure" in stdout_message
                        or verification_output.returncode != 0
                    ):
                        print(f"Bolero failed")
                        # dump_result(result_file, result)
                        # return
                    else:
                        print(f"Bolero pass")
                        result["bolero"] = True
                        bolero_success = True
                else:
                    raise Exception("Command timeout")
        if not bolero_success:
            print(f"Failed to generate valid code after {max_llm_attempts} attempts")
            dump_result(result_file, result)
            return

        dump_result(result_file, result)
        exit(0)
    ##########################################################################################
    ###################################### Bounded KANI ######################################
    if bounded_kani and rust_compiles and bolero_successful:
        print("Running Kani")
        command = "cargo kani --no-unwinding-checks --default-unwind 10"
        verification_output, timeout = verification_utils.verify(
            wasm_kani_path,
            command,
            f"{subdir}/kani_out.txt",
            f"{subdir}/kani_err.txt",
            all_timeout,
        )
        if not timeout:
            err_message = verification_output.stderr
            stdout_message = verification_output.stdout
            if (
                "VERIFICATION:- FAILED" in err_message
                or "VERIFICATION:- FAILED" in stdout_message
                or verification_output.returncode != 0
            ):
                print("Kani failed")
                dump_result(result_file, result)
            else:
                print("Kani succesful")
                result["bounded_kani"] = True
                dump_result(result_file, result)
        else:
            print("Kani timeout")
            dump_result(result_file, result)


if __name__ == "__main__":
    main()
