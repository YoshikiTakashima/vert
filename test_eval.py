import subprocess
import os
from typing import List
import sys
from datetime import datetime
from concurrent.futures import ProcessPoolExecutor, as_completed
import threading
from queue import Queue
from rich.console import Console
from rich.panel import Panel
from rich.table import Table
from rich.progress import (
    Progress,
    TextColumn,
    BarColumn,
    TaskProgressColumn,
    TimeRemainingColumn,
)
import argparse


def parse_args():
    parser = argparse.ArgumentParser(description='Run benchmarks with customizable parameters')
    # Arguments for run_single_benchmark
    parser.add_argument('--aws-profile', type=str, default="default",
                      help='AWS profile to use')
    parser.add_argument('--language', type=str, default="go", choices=["c", "cpp", "go"],
                      help='Programming language')
    parser.add_argument('--llm-attempts', type=int, default=5,
                      help='Number of LLM attempts')
    
    # Arguments for parallel execution
    parser.add_argument('--max-workers', type=int, default=5,
                      help='Maximum number of parallel workers')
    parser.add_argument('--run-all', action="store_true", help='If passed, run all the scripts.')
    
    return parser.parse_args()

console = Console()

class StreamingProcess:
    def __init__(self, benchmark: str, total_benchmarks: int):
        self.benchmark = benchmark
        self.total_benchmarks = total_benchmarks
        self.output_queue = Queue()
        self.process = None
        self.stream_thread = None

    def stream_output(self):
        """Stream output from the process in real-time."""
        while self.process.poll() is None:
            # Read stdout
            output = self.process.stdout.readline()
            if output:
                timestamp = datetime.now().strftime('%H:%M:%S')
                self.output_queue.put(('stdout', output.decode().strip()))

            # Read stderr
            error = self.process.stderr.readline()
            if error:
                timestamp = datetime.now().strftime('%H:%M:%S')
                self.output_queue.put(('stderr', error.decode().strip()))

        # Read any remaining output
        stdout, stderr = self.process.communicate()
        if stdout:
            self.output_queue.put(('stdout', stdout.decode().strip()))
        if stderr:
            self.output_queue.put(('stderr', stderr.decode().strip()))

        self.output_queue.put(None)  # Signal that we're done

def create_progress() -> Progress:
    return Progress(
        TextColumn("[bold blue]{task.description}"),
        BarColumn(complete_style="green", finished_style="bright_green"),
        TaskProgressColumn(),
        TimeRemainingColumn(),
        console=console,
        expand=True
    )

def run_single_benchmark(benchmark: str, total_benchmarks: int, args: argparse.Namespace) -> dict:
    """Run a single benchmark and return its results."""
    base_dir = f"benchmark/{args.language}_transcoder"
    benchmark_path = os.path.join(base_dir, benchmark)
    command = [
        "python3",
        "torust.py",
        "--aws-profile", args.aws_profile,
        "--language", args.language,
        "--llm-attempts", str(args.llm_attempts),
        "--benchmark-dir", benchmark_path
    ]

    try:
        # Create panel for this benchmark
        console.print(Panel(
            f"[bold cyan]Starting benchmark[/bold cyan]: [yellow]{benchmark}[/yellow]\n"
            f"[dim]Command: {' '.join(command)}[/dim]",
            expand=False,
            border_style="blue"
        ))

        # Create streaming process
        sp = StreamingProcess(benchmark, total_benchmarks)
        
        # Start the process
        sp.process = subprocess.Popen(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=1,
            universal_newlines=False
        )

        # Start output streaming in a separate thread
        sp.stream_thread = threading.Thread(target=sp.stream_output)
        sp.stream_thread.start()

        compilation_failed = False
        bolero_failed = False
        
        # Process output queue
        while True:
            item = sp.output_queue.get()
            if item is None:
                break
            
            stream_type, content = item
            timestamp = datetime.now().strftime('%H:%M:%S')
            
            # Check for failure conditions in the output
            if "Failed to" in content or "bolero failed" in content:
                bolero_failed = True
            
            if stream_type == 'stdout':
                console.print(f"[dim]{timestamp}[/dim] [green]{content}[/green]")
            else:
                console.print(f"[dim]{timestamp}[/dim] [red]ERROR: {content}[/red]")

        sp.stream_thread.join()
        return_code = sp.process.poll()

        # Determine status based on both return code and output content
        if return_code == 0 and not bolero_failed:
            status_color = "green"
            status_text = "SUCCESS"
        elif return_code == 0 and bolero_failed:
            status_color = "yellow"
            status_text = "COMPILED BUT NOT PASSING BOLERO"
        else:
            status_color = "red"
            status_text = "FAILED"

        console.print(Panel(
            f"[{status_color}]Benchmark {status_text}[/{status_color}]: [yellow]{benchmark}[/yellow]\n"
            f"[dim]Return code: {return_code}[/dim]",
            expand=False,
            border_style=status_color
        ))

        return {
            'benchmark': benchmark,
            'success': return_code == 0 and not bolero_failed,
            'bolero_failed': bolero_failed,
            'return_code': return_code
        }

    except Exception as e:
        console.print(f"[red]Error running benchmark {benchmark}: {str(e)}[/red]")
        return {
            'benchmark': benchmark,
            'success': False,
            'error': str(e)
        }

def run_benchmarks_parallel(benchmark_names: List[str], args: argparse.Namespace) -> None:
    """Run benchmarks in parallel with progress bar."""
    total = len(benchmark_names)
    failed = []
    successful = []
    bolero_failed_list = []

    console.print(Panel(
        f"[bold cyan]Starting parallel execution of {total} benchmarks with {args.max_workers} workers[/bold cyan]",
        expand=False,
        border_style="cyan"
    ))

    with create_progress() as progress:
        task = progress.add_task("[cyan]Running benchmarks...", total=total)
        
        with ProcessPoolExecutor(max_workers=args.max_workers) as executor:
            future_to_benchmark = {
                executor.submit(run_single_benchmark, benchmark, total, args): benchmark
                for benchmark in benchmark_names
            }

            for future in as_completed(future_to_benchmark):
                benchmark = future_to_benchmark[future]
                try:
                    result = future.result()
                    if result['success']:
                        successful.append(benchmark)
                    elif result.get('bolero_failed', False):
                        bolero_failed_list.append(benchmark)
                    else:
                        failed.append((benchmark, result.get('error', f"Return code: {result.get('return_code')}")))
                except Exception as e:
                    failed.append((benchmark, str(e)))

                progress.update(task, advance=1)

    # Create summary table
    table = Table(show_header=True, header_style="bold magenta")
    table.add_column("Category", style="cyan")
    table.add_column("Count/Value", style="green")
    table.add_column("Details", style="yellow")

    # Add command arguments section
    table.add_row(
        "Command Arguments",
        "",
        ""
    )
    table.add_row(
        "└─ AWS Profile",
        args.aws_profile,
        ""
    )
    table.add_row(
        "└─ Language",
        args.language,
        ""
    )
    table.add_row(
        "└─ LLM Attempts",
        str(args.llm_attempts),
        ""
    )
    table.add_row(
        "└─ Max Workers",
        str(args.max_workers),
        ""
    )

    # Add empty row for spacing
    table.add_row("", "", "")

    # Add benchmark results
    table.add_row(
        "Total Benchmarks",
        str(total),
        ", ".join(benchmark_names)
    )
    table.add_row(
        "Successful",
        str(len(successful)),
        ", ".join(successful)
    )
    table.add_row(
        "Compiled but Failed Bolero",
        str(len(bolero_failed_list)),
        ", ".join(bolero_failed_list)
    )
    table.add_row(
        "Failed",
        str(len(failed)),
        "\n".join(f"{b}: {e}" for b, e in failed)
    )

    console.print("\n")
    console.print(Panel(
        table,
        title="[bold cyan]Execution Summary[/bold cyan]",
        border_style="cyan",
        expand=False
    ))


def get_benchmarks(language: str) -> List[str]:
    """Get all benchmark names from the language directory."""
    base_dir = f"benchmark/{language}_transcoder"
    
    # Check if directory exists
    if not os.path.exists(base_dir):
        console.print(f"[red]Error: Directory {base_dir} does not exist[/red]")
        return []
    
    # Get immediate subdirectories only
    try:
        benchmarks = [d for d in os.listdir(base_dir) 
                     if os.path.isdir(os.path.join(base_dir, d))]
        return sorted(benchmarks)  # Sort alphabetically for consistent ordering
    except Exception as e:
        console.print(f"[red]Error reading benchmarks from {base_dir}: {str(e)}[/red]")
        return []


SUBSET_BENCHMARKS_SMALL_EVAL = {
    "c": [
        "benchmark/c_transcoder/ADD_1_TO_A_GIVEN_NUMBER",
        "benchmark/c_transcoder/ADD_TWO_NUMBERS_WITHOUT_USING_ARITHMETIC_OPERATORS",
        "benchmark/c_transcoder/BASIC_AND_EXTENDED_EUCLIDEAN_ALGORITHMS",
        "benchmark/c_transcoder/CHECK_IF_A_NUMBER_IS_POWER_OF_ANOTHER_NUMBER",
        "benchmark/c_transcoder/CHECK_NUMBER_IS_PERFECT_SQUARE_USING_ADDITIONSUBTRACTION",

    ],
    "cpp": [
        "benchmark/cpp_transcoder/ADD_1_TO_A_GIVEN_NUMBER",
        "benchmark/cpp_transcoder/ADD_TWO_NUMBERS_WITHOUT_USING_ARITHMETIC_OPERATORS",
        "benchmark/cpp_transcoder/BASIC_AND_EXTENDED_EUCLIDEAN_ALGORITHMS",
        "benchmark/cpp_transcoder/CHECK_IF_A_NUMBER_IS_POWER_OF_ANOTHER_NUMBER",
        "benchmark/cpp_transcoder/CHECK_NUMBER_IS_PERFECT_SQUARE_USING_ADDITIONSUBTRACTION",

    ],
    "go": [
        "benchmark/go_transcoder/ADD_1_TO_A_GIVEN_NUMBER",
        "benchmark/go_transcoder/ADD_TWO_NUMBERS_WITHOUT_USING_ARITHMETIC_OPERATORS",
        "benchmark/go_transcoder/BASIC_AND_EXTENDED_EUCLIDEAN_ALGORITHMS",
        "benchmark/go_transcoder/CHECK_IF_A_NUMBER_IS_POWER_OF_ANOTHER_NUMBER",
        "benchmark/go_transcoder/CHECK_NUMBER_IS_PERFECT_SQUARE_USING_ADDITIONSUBTRACTION",
    ],
}

if __name__ == "__main__":
    try:
        args = parse_args()
        benchmarks = (
            get_benchmarks(args.language)
            if args.run_all
            else SUBSET_BENCHMARKS_SMALL_EVAL[args.language]
        )
        console.print(f"[green]Found {len(benchmarks)} benchmarks for {args.language}[/green]")
        #run_benchmarks_parallel(benchmarks, args)
    except KeyboardInterrupt:
        console.print("\n[red]Script terminated by user[/red]")
        sys.exit(1)
