all: data part-1

data:
	mkdir -p data

part-1: src/main.rs
	rustc src/main.rs -o main-1
