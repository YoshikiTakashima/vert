import os
import sys

profiles = [
    "hlzeast",
    "hlzwest",
    "bpeast",
    "bpwest",
    "ayeast",
    "aywest",
    "huieast",
    "huiwest"
]

def get_profile():
    ret = profiles.pop(0)
    profiles.append(ret)
    return ret

def main():
    if len(sys.argv) != 3:
        print(f"Usage: python3 {sys.argv[0]} <language> <benchmark_dir>")
        return
    lang = sys.argv[1]
    benchmark_dir = sys.argv[2]


    for bdir in os.listdir(benchmark_dir):
        cmd = (f"python3 torust.py --language {lang} --benchmark-dir {benchmark_dir}/{bdir} --aws-profile {get_profile()} --llm-attempts 1 "
               f"&> {benchmark_dir}/{bdir}/log.txt")
        print(cmd)

if  __name__ == "__main__":
    main()
