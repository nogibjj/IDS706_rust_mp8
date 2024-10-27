all: install format lint test

install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

format:	
	black src --line-length 100 --verbose

lint:
	ruff check src/ --fix --verbose

test:
	python -m pytest -vv src/

run:
	python src/main.py

clean:
	rm -rf $(VENV)

rust-install:
	cargo build 

rust-format:
	cargo fmt

rust-lint:
	 cargo clippy --all-targets --

rust-test:
	cargo test 

rust-run:
	cargo run 
