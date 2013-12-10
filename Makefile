main:
	rustc -o ./bin/main       --bin  src/demo.rs
	rustc -o ./bin/main_tests --test src/demo.rs
