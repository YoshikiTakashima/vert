import subprocess
import os
from typing import List
import sys
from datetime import datetime
from concurrent.futures import ProcessPoolExecutor, as_completed
from tqdm import tqdm
import threading
from queue import Queue
import time
from rich.console import Console
from rich.panel import Panel
from rich.live import Live
from rich.table import Table
from rich import print as rprint
from rich.progress import (
    Progress,
    TextColumn,
    BarColumn,
    TaskProgressColumn,
    TimeRemainingColumn,
)

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

def run_single_benchmark(benchmark: str, total_benchmarks: int) -> dict:
    """Run a single benchmark and return its results."""
    base_dir = "benchmark/c_transcoder"
    benchmark_path = os.path.join(base_dir, benchmark)
    command = [
        "python3",
        "torust.py",
        "--aws-profile", "default",
        "--language", "c",
        "--llm-attempts", "5",
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

        # Process output queue
        while True:
            item = sp.output_queue.get()
            if item is None:
                break
            
            stream_type, content = item
            timestamp = datetime.now().strftime('%H:%M:%S')
            
            if stream_type == 'stdout':
                console.print(f"[dim]{timestamp}[/dim] [green]{content}[/green]")
            else:
                console.print(f"[dim]{timestamp}[/dim] [red]ERROR: {content}[/red]")

        sp.stream_thread.join()
        return_code = sp.process.poll()

        # Print completion status
        status_color = "green" if return_code == 0 else "red"
        status_text = "SUCCESS" if return_code == 0 else "FAILED"
        console.print(Panel(
            f"[{status_color}]Benchmark {status_text}[/{status_color}]: [yellow]{benchmark}[/yellow]\n"
            f"[dim]Return code: {return_code}[/dim]",
            expand=False,
            border_style=status_color
        ))

        return {
            'benchmark': benchmark,
            'success': return_code == 0,
            'return_code': return_code
        }

    except Exception as e:
        console.print(f"[red]Error running benchmark {benchmark}: {str(e)}[/red]")
        return {
            'benchmark': benchmark,
            'success': False,
            'error': str(e)
        }

def run_benchmarks_parallel(benchmark_names: List[str], max_workers: int = 3) -> None:
    """Run benchmarks in parallel with progress bar."""
    total = len(benchmark_names)
    completed = 0
    failed = []
    successful = []

    console.print(Panel(
        f"[bold cyan]Starting parallel execution of {total} benchmarks with {max_workers} workers[/bold cyan]",
        expand=False,
        border_style="cyan"
    ))

    with create_progress() as progress:
        task = progress.add_task("[cyan]Running benchmarks...", total=total)
        
        with ProcessPoolExecutor(max_workers=max_workers) as executor:
            future_to_benchmark = {
                executor.submit(run_single_benchmark, benchmark, total): benchmark
                for benchmark in benchmark_names
            }

            for future in as_completed(future_to_benchmark):
                benchmark = future_to_benchmark[future]
                try:
                    result = future.result()
                    if result['success']:
                        successful.append(benchmark)
                    else:
                        failed.append((benchmark, result.get('error', f"Return code: {result.get('return_code')}")))
                except Exception as e:
                    failed.append((benchmark, str(e)))

                progress.update(task, advance=1)

    # Create summary table
    table = Table(show_header=True, header_style="bold magenta")
    table.add_column("Category", style="cyan")
    table.add_column("Count", style="green")
    table.add_column("Details", style="yellow")

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

benchmarks = [
    'AREA_OF_A_HEXAGON',
    'CHECK_NUMBER_IS_PERFECT_SQUARE_USING_ADDITIONSUBTRACTION',
    'ADD_1_TO_A_GIVEN_NUMBER',
    'ADD_1_TO_A_GIVEN_NUMBER_1',
    'BELL_NUMBERS_NUMBER_OF_WAYS_TO_PARTITION_A_SET',
    'CHECK_ARRAY_REPRESENTS_INORDER_BINARY_SEARCH_TREE_NOT',
    'CASSINIS_IDENTITY',
    'CHECK_IF_ALL_THE_ELEMENTS_CAN_BE_MADE_OF_SAME_PARITY_BY_INVERTING_ADJACENT_ELEMENTS',
    'BIRTHDAY_PARADOX',
    'AREA_SQUARE_CIRCUMSCRIBED_CIRCLE',
    'CALCULATING_FACTORIALS_USING_STIRLING_APPROXIMATION',
    'BASIC_AND_EXTENDED_EUCLIDEAN_ALGORITHMS',
    'CEILING_IN_A_SORTED_ARRAY_1',
    'CHECK_IF_X_CAN_GIVE_CHANGE_TO_EVERY_PERSON_IN_THE_QUEUE',
    'CALCULATE_VOLUME_DODECAHEDRON',
    'CEILING_IN_A_SORTED_ARRAY',
    'BIN_PACKING_PROBLEM_MINIMIZE_NUMBER_OF_USED_BINS',
    'CHECK_IF_A_NUMBER_IS_JUMBLED_OR_NOT',
    'ADD_TWO_NUMBERS_WITHOUT_USING_ARITHMETIC_OPERATORS',
    'CHECK_INTEGER_OVERFLOW_MULTIPLICATION'
]

if __name__ == "__main__":
    try:
        run_benchmarks_parallel(benchmarks, max_workers=3)
    except KeyboardInterrupt:
        console.print("\n[red]Script terminated by user[/red]")
        sys.exit(1)
