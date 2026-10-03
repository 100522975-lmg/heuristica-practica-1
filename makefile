all: data app

data:
	mkdir -p data

app: src/main.rs
	rustc src/main.rs -o main
