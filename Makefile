all: main main_test docs

output_dir: 
	@[ -d ./bin ] || mkdir ./bin

main: output_dir
	@[ -d ./bin ] || mkdir ./bin
	rustc -o ./bin/main       --bin  src/demo.rs

docs: output_dir
	pandoc --standalone -t slidy slides/lightning-2013-12.md -o bin/lightning-2013-12.html
	cp -R ./slides/static ./bin

main_test: output_dir
	rustc -o ./bin/main_test --test src/demo.rs --allow dead_code --opt-level 0

run:
	./bin/main

run_test:
	./bin/main_test --test

run_testbench:
	./bin/main_test --test --bench

run_loop_test: main_test run_test
run_loop_testbench: main_test run_testbench

clean:
	@rm -rf ./bin

help:
	@echo "Usage: make [main | main_test | docs | clean]"
	@echo "       make [run | run_test | run_testbench | testloop]"
	@echo "       make [help]"

