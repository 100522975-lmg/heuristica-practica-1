#!/usr/bin/python3
import sys, os

external_bin = "main"

if len(sys.argv) != 3:
    raise Exception("Not enough arguments (Expected input-file and output-file)")

input_file = sys.argv[1]
output_file = sys.argv[2]
model = 1

if not os.path.exists(external_bin):
    raise Exception(f"No file named {external_bin} in {os.getcwd()}")

os.system(f"./{external_bin} {input_file} {output_file} {model}")
