#!/bin/bash
if [ -z "$1" ]
then
    echo "Please invoke with $0 problem_number, e.g. $0 30";
    exit 1;
fi

mkdir "src/problem_$1";
touch "src/problem_$1/mod.rs";
echo "pub mod problem_$1;" >> src/lib.rs
