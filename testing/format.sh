#!/bin/bash 
FILENAME="$1" 
sed 's/[][]//g' <$FILENAME  | tr , '\n' >"data.txt"
