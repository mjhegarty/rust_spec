#!/bin/bash 


#script to generate a raw audio file
rm test.txt data.txt
cargo run

FILENAME="test.txt" 
sed 's/[][]//g' <$FILENAME  | tr , '\n' >"data.txt"

python demod.py data.txt
