rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release
	

# 'all' target to format, lint, test, and run
all: 
	@echo "Running the 'all' target"
	# Call the make targets for format, lint, test, and run
	make format
	make lint
	make test
	make run



install_py:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test_py:
	python -m pytest -vv --cov=main --cov=mylib test_*.py

format_py:	
	black *.py 

lint_py:
	#disable comment to test speed
	#pylint --disable=R,C --ignore-patterns=test_.*?py *.py mylib/*.py
	#ruff linting is 10-100X faster than pylint
	ruff check *.py mylib/*.py

container-lint_py:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor_py: format lint

deploy_py:
	#deploy goes here
		
all_py: install_py lint_py test_py format_py deploy_py