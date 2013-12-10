main:
	@[ -d ./bin ] || mkdir ./bin
	rustc -o ./bin/main       --bin  src/demo.rs
	rustc -o ./bin/main_tests --test src/demo.rs
	pandoc --data-dir=slides --self-contained -s -t slidy slides/lightning-2013-12.md -o bin/lightning-2013-12.html

run:
	./bin/main

test:
	./bin/main_tests --test --bench

clean:
	@rm -rf ./bin

help:
	@echo "Usage: make [main | run | test | clean | help]"

