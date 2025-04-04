import subprocess
import os
import shutil
import anthropic
import argparse
from generate_utils import GenerateUtils, VerificationUtils

generate_utils = GenerateUtils()
verification_utils = VerificationUtils()
anthropic_client = anthropic.Client(os.environ["ANTHROPIC_API_KEY"])


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


def clean_generated_code(generated_code):
    generated_code = generated_code.replace("<|endoftext|>", "").replace("pub fn", "fn")
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
    file_path = f"{rust_dir}/{package_name}/src/{file_name}.rs"
    rust_input = (
        source_code
        + f"\nRust refactoring of above C{language} code, with code only, no comments. Use the same function name, same argument and return types. Make sure it includes all imports, uses safe rust, and compiles. Give only code, and no main function. Convert i32 types to f32 if necessary. Use mut variables if necessary"
    )

    compiles = False
    tries = 0
    while not compiles:
        tries += 1
        if tries > number_tries:
            break
        print(f"LLM attempt # {tries}")
        rust_output = claude_gen(rust_input)
        rust_output = clean_generated_code(rust_output)

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


if __name__ == "__main__":
    ap = argparse.ArgumentParser(description="Evaluation for VERT")
    ap.add_argument(
        "language",
        choices=["c", "cpp", "go"],
        help="Choose source language to compile to Rust: c, cpp, or go",
    )
    args = ap.parse_args()
    language = args.language
    file_dir = f"/home/program_strings/{language}_transcoder"
    rust_dir = f"/home/program_strings/rust_{language}_transcoder"

    ###################################### Controls which portion to run ######################################
    entry_point = 1  # Compiles rwasm folders and locates entry point
    use_claude = 1  # Generate LLM transpilation
    bolero = 1  # Bolero verification
    bounded_kani = 1  # Bounded Kani verification
    full_kani = 1  # Full Kani verification
    number_tries = 20  # Number of tries for LLM
    testing_on_one = False
    test_project = "REPLACE_CHARACTER_C1_C2_C2_C1_STRING_S"
    ####################################################################################################

    valid_project_count = 0
    for subdir, dirs, files in os.walk(f"{file_dir}/"):
        for file in files:
            file_path = os.path.join(subdir, file)
            package_name = subdir.split("/")[-1]
            if "out-rwasm" in subdir:
                continue
            if (
                "test" in file
                or "onefunc" in file
                or "processed" in file
                or "mutated" in file
                or "main" in file
                or "_diff" in file
                or "toml" in file
                or ".rs.c" in file
                or "_towasm" in file
                or "Cargo" in file
                or "CACHEDIR" in file
            ):
                continue
            if "src" in file or "src" in package_name or "out-rwasm" in package_name:
                continue
            if testing_on_one and test_project != package_name:
                continue
            project_done = 0
            with open(f"{language}_results.csv", "r") as result_file:
                existing_results = result_file.readlines()
            for result_idx, existing_result in enumerate(existing_results):
                existing_project_name = existing_result.split(",")[0]
                if existing_project_name == package_name:
                    project_done = 1

            if not testing_on_one and project_done:
                continue
            print(package_name)

            rust_compiles = True
            bolero_successful = True
            kani_successful = True

            wasm_bolero_main = f"{file_dir}/{package_name}/out-rwasm-bolero/src/main.rs"
            wasm_kani_main = f"{file_dir}/{package_name}/out-rwasm-mutated/src/main.rs"

            result_string = (
                f"{package_name}, compile=0, bolero=0, bounded_kani=0, full_kani=0"
            )
            if "cpp" in file_dir:
                file_ext = ".cpp"
            else:
                file_ext = ".c"

            file_name = file.replace(".go", "").replace(".cpp", "").replace(".c", "")

            f_filled = ""
            c_filepath = f"{file_dir}/{package_name}/{file_name}{file_ext}"
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

            source_output, original = generate_utils.c_code_process(
                file_dir, package_name, file_name, f_filled, args_types
            )

            benchmark_lines = len(original.splitlines())
            pointer_variables = original.count(" *")
            struct_variables = original.count("struct ")

            ###################################### 2. set up wasm file #########################################
            cwasm_path = file_path.replace(file_ext, f"_towasm{file_ext}")
            if entry_point:
                try:
                    rwasm_arg_types = verification_utils.mutate_test(
                        file_dir,
                        package_name,
                        cwasm_path,
                        fn_name,
                        args_types,
                        file_ext,
                        fn_out_type,
                    )

                except:
                    print("Source file failed to compile")
                    rust_compiles = False
                    with open(f"{language}_results.csv", "a") as result_file:
                        result_file.write("\n" + result_string)
                    continue
            ####################################################################################################

            leetcode_name = "_".join(package_name.split("_")[1:])
            if "transcoder" in file_dir:
                leetcode_name = package_name

            ###################################### 3. LLM ######################################################
            if use_claude:
                generate_utils.build_rust_folder(rust_dir, leetcode_name)

                compiled_rust, rust_compiles = claude(
                    source_output,
                    leetcode_name,
                    rust_dir,
                    file_name,
                    number_tries,
                    file_ext,
                )
                if not rust_compiles:
                    result_string = f"{package_name}, compile=0, bolero=0, bounded_kani=0, full_kani=0"
                else:
                    result_string = f"{package_name}, compile=1, bolero=0, bounded_kani=0, full_kani=0"

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
                wasm_function = f"\n\nfn {wasm_fn_name}() -> {rust_fn_out_type} {{\n\tlet mut wasm_module = WasmModule::new();\n\twasm_module._start().unwrap();\n\tunsafe {{ RESULT }}\n}}\n\n"
                rwasm_arg_declaration = ""
                rwasm_harness_args = ""
                arg_string = ""
                bolero_argstring = ""
                bolero_arg_unsafe = "unsafe {\n"
                kani_arg_string = ""
                string_bolero_harness = ""
                string_ending_bracket = ""
                for i, arg_type in enumerate(args_types):
                    if "[]" in arg_type:
                        arg_string += (
                            f"[unsafe{{PARAM{i+1}}}[0], unsafe{{PARAM{i+1}}}[1]],"
                        )
                        kani_arg_string += (
                            f"[unsafe{{PARAM{i+1}}}[0], unsafe{{PARAM{i+1}}}[1]],"
                        )
                        bolero_argstring += f"PARAM_{i+1},"
                        bolero_arg_unsafe += f"\t\tPARAM{i+1} = PARAM_{i+1};\n"
                    elif "string" in arg_type:
                        string_bolero_harness = f"\t\tif let Some(param{i+1}_0) = PARAM_{i+1}.chars().nth(0){{\n"
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
                bolero_argstring = "(" + bolero_argstring[:-1] + ")"
                bolero_arg_unsafe += "\n\t\t}"
                ##########################################
                ########## 4.2 Bolero Harness ############

                bolero_import = "\nuse bolero::check;\n#[test]"
                bolero_func_decl = f"\nfn bolero_wasm_eq(){{\n\tbolero::check!().with_type::<({rust_args_types})>().cloned().for_each(|{bolero_argstring}|{{ \n{string_bolero_harness}".replace(
                    "'", ""
                )
                bolero_func_body = f"\t\t{bolero_arg_unsafe}\n\t\tlet result = {fn_name}{arg_string};\n\t\tlet result_prime = {wasm_fn_name}();\n\t\tassert_eq!(result, result_prime);\n\t{string_ending_bracket}}});\n}}"
                final_bolero_harness = (
                    "\n" + bolero_import + bolero_func_decl + bolero_func_body
                )
                ########################################
                ########## 4.3 Kani Harness ############

                kani_declare = "\n#[cfg(kani)]\n#[kani::proof]\n#[kani::unwind(10)]"
                kani_func_decl = f"\nfn kani_wasm_eq(){{ \n"
                kani_func_body = f"\t\tlet result = {fn_name}{kani_arg_string};\n\t\tlet result_prime = {wasm_fn_name}();\n\t\tassert_eq!(result, result_prime);\n}}"
                final_kani_harness = (
                    "\n" + kani_declare + kani_func_decl + kani_func_body
                )
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

            wasm_bolero_path = f"{file_dir}/{package_name}/out-rwasm-bolero/src"
            wasm_kani_path = f"{file_dir}/{package_name}/out-rwasm-mutated/src"

            bolero_target_path = wasm_bolero_path + "/target"
            kani_target_path = wasm_kani_path.replace("/src", "/target")
            ##########################################################################################
            ###################################### BOLERO ############################################
            if bolero and rust_compiles:
                print("Running bolero")
                command = "cargo bolero reduce bolero_wasm_eq"
                verification_output, timeout = verification_utils.verify(
                    wasm_bolero_path, command
                )
                if not timeout:
                    err_message = verification_output.stderr
                    stdout_message = verification_output.stdout

                    if "could not compile" in err_message:
                        print("Compilation problem")
                        print(err_message)
                        bolero_successful = False
                    elif (
                        "Test Failure" in err_message
                        or "Test Failure" in stdout_message
                    ):
                        print(f"Bolero failed")
                        result_string = f"{package_name}, compile=1, bolero=0, bounded_kani=0, full_kani=0"
                        bolero_successful = False
                    else:
                        print(f"Bolero pass")
                        result_string = f"{package_name}, compile=1, bolero=1, bounded_kani=0, full_kani=0"
                else:
                    print("Bolero timeout")
                    result_string = f"{package_name}, compile=1, bolero=1, bounded_kani=0, full_kani=0"
                    bolero_successful = False

                if not testing_on_one:
                    shutil.rmtree(bolero_target_path)

            ##########################################################################################
            ###################################### Bounded KANI ######################################
            if bounded_kani and rust_compiles and bolero_successful:
                print("Running Kani")
                command = "cargo kani --no-unwinding-checks --default-unwind 10"
                verification_output, timeout = verification_utils.verify(
                    wasm_kani_path, command
                )
                if not timeout:
                    err_message = verification_output.stderr
                    stdout_message = verification_output.stdout
                    if (
                        "VERIFICATION:- FAILED" in err_message
                        or "VERIFICATION:- FAILED" in stdout_message
                    ):
                        print("Kani failed")
                        kani_successful = False
                        result_string = f"{package_name}, compile=1, bolero=1, bounded_kani=0, full_kani=0"
                    else:
                        print("Kani succesful")
                        result_string = f"{package_name}, compile=1, bolero=1, bounded_kani=1, full_kani=0"
                else:
                    print("Kani timeout")
                    kani_successful = False
                    result_string = f"{package_name}, compile=1, bolero=1, bounded_kani=0, full_kani=0"

            ##########################################################################################
            ###################################### Full KANI #########################################
            if full_kani and rust_compiles and bolero_successful and kani_successful:
                with open(wasm_kani_main, "r") as wasmfile:
                    wasm_kani = wasmfile.read()
                wasm_fullkani = wasm_kani.replace("#[kani::unwind(10)]", "")
                with open(wasm_kani_main, "w") as wasmfile:
                    wasmfile.write(wasm_fullkani)

                print("Running Full Kani")
                command = "cargo kani"
                verification_output, timeout = verification_utils.verify(
                    wasm_kani_path, command
                )
                if not timeout:
                    err_message = verification_output.stderr
                    stdout_message = verification_output.stdout
                    if (
                        "VERIFICATION:- FAILED" in err_message
                        or "VERIFICATION:- FAILED" in stdout_message
                    ):
                        print("Full Kani failed")
                        result_string = f"{package_name}, compile=1, bolero=1, bounded_kani=1, full_kani=0"
                    else:
                        # print(err_message)
                        # print(stdout_message)
                        print("Full Kani succesful")
                        result_string = f"{package_name}, compile=1, bolero=1, bounded_kani=1, full_kani=1"
                else:
                    print("Full Kani timeout")
                    result_string = f"{package_name}, compile=1, bolero=1, bounded_kani=1, full_kani=0"

                with open(wasm_kani_main, "w") as wasmfile:
                    wasmfile.write(wasm_kani)
            if not testing_on_one and bolero_successful and rust_compiles:
                shutil.rmtree(kani_target_path)
            ##########################################################################################
            with open(f"{language}_results.csv", "a") as result_file:
                result_file.write("\n" + result_string)

    subprocess.run(
        f"chmod -R a+rw /home",
        shell=True,
        cwd="/home",
        capture_output=True,
    )
