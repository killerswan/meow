all: build build_docs build_test build_testloop

bindir: 
	@[ -d ./bin ] || mkdir ./bin

build: bindir
	rustc -o ./bin/main src/demo.rs --opt-level=3

build_docs: bindir
	pandoc slides/lightning-2013-12.md -o bin/lightning-2013-12.html \
	   --standalone -t slidy \
	   -V slidy-url=../slides/themes/default
	pandoc --standalone -t slidy -V slidy-url=../slides/themes/kevin-pink slides/talk.md -o bin/talk.html
	cp -R ./slides/static ./bin

build_docs_online: bindir
	pandoc slides/lightning-2013-12.md -o bin/lightning-2013-12.html \
	   --standalone -t slidy
	cp -R ./slides/static ./bin

build_docs_pdf: bindir
	pandoc slides/lightning-2013-12.md -o bin/lightning-2013-12.pdf

build_test: bindir
	rustc -o ./bin/main_test --test src/demo.rs \
	   --opt-level 0

build_testloop: bindir
	rustc -o ./bin/testloop src/testloop.rs \
	   --opt-level=3 -g

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

%.html:
	-@mkdir bin
	@cp -R slides/static bin/static
	@cp -R slides/themes bin/themes
	pandoc --standalone -t slidy -V slidy-url=../slides/themes/kevin-pink slides/$? -o bin/$@

talk.html: talk.md


