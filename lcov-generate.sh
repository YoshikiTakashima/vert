#!/bin/bash

BENCH_SET=$1

for bench in $(cat ${BENCH_SET}_results.csv | grep 'bolero=1' | awk -F, '{print $1}')
do
  (
  cd "benchmark/${BENCH_SET}_transcoder/$bench/out-rwasm-bolero";
  pwd;
  python ~/vert-fse-24/replication_package/bolero2proptest.py
)
done

