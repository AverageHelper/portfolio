#!/usr/bin/env bash
set -euo pipefail

# Install cargo-tarpaulin if it doesn't already exist
if ! cargo install --list | grep cargo-tarpaulin &> /dev/null
then
	while true; do
		read -p "cargo-tarpaulin not found. Do you wish to install 'cargo-tarpaulin'? (y/n) " yn
		case $yn in
			[Yy]* ) cargo install cargo-tarpaulin --registry crates-io; break;;
			[Nn]* ) exit 1;;
			* ) echo "Please answer (Y)es or (N)o.";;
		esac
	done
fi

# ** Test Coverage **
# Exclusions:
# - Blocks with `#[cfg(not(tarpaulin_include))]` set

# Turn coverage results into lcov files
echo "Collecting code coverage..."
cargo tarpaulin -o lcov --locked --target-dir target/tarpaulin-target/ --skip-clean "$@"

# TODO: Figure some way to generate and display a cute code-coverage sticker in README.md
