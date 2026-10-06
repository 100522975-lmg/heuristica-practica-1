import sys, os

external_bin = "main-1"

if len(sys.argv) == 3:
    input_file = sys.argv[1]
    output_file = sys.argv[2]
else:
    raise Exception("Not enough arguments (Expected input-file and output-file)")

if os.path.exists(external_bin):
    os.system(f"./{external_bin} {input_file} {output_file}")
else:
    print(f"No file named {external_bin} in {os.getcwd()}")
