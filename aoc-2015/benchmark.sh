#!/bin/zsh
{
for i in `seq 1 25`
do
	cargo aoc bench -d $i
done
} > benchmark_results.txt
