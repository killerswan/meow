all: build build_docs build_test build_testloop

bindir: 
	@[ -d ./bin ] || mkdir ./bin

build: bindir
	rustc -o ./bin/main --bin --opt-level=3 src/demo.rs

build_docs: bindir
	pandoc --standalone -t slidy -V slidy-url=../slides/themes/default slides/lightning-2013-12.md -o bin/lightning-2013-12.html
	cp -R ./slides/static ./bin

build_docs_online: bindir
	pandoc --standalone -t slidy slides/lightning-2013-12.md -o bin/lightning-2013-12.html
	cp -R ./slides/static ./bin

build_docs_pdf: bindir
	pandoc slides/lightning-2013-12.md -o bin/lightning-2013-12.pdf

build_test: bindir
	rustc -o ./bin/main_test --test src/demo.rs --allow dead_code --opt-level 0

build_testloop: bindir
	rustc -o ./bin/testloop --bin -Z debug-info -Z extra-debug-info --opt-level=3 src/testloop.rs

run:
	./bin/main

run_test:
	./bin/main_test --test

run_testbench:
	./bin/main_test --test --bench

run_loop:
	./bin/testloop ./src/demo.rs

run_loopbench:
	./bin/testloop ./src/demo.rs --test --bench

clean:
	@rm -rf ./bin

help:
	@echo "Usage: make [clean]"
	@echo "       make [run | run_test | run_testbench]"
	@echo "       make [run_loop | run_loopbench]"
	@echo "       make [build_docs | build_docs_pdf | build_docs_online]"
	@echo "       make [help]"

