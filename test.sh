#!/usr/bin/env bash
set -euo pipefail

# Install grcov if it doesn't already exist
if ! command -v grcov &> /dev/null
then
	while true; do
		read -p "grcov not found. Do you wish to install 'grcov' and related components? (y/n) " yn
		case $yn in
			[Yy]* ) cargo install grcov --registry crates-io && rustup component add llvm-tools-preview; break;;
			[Nn]* ) exit 1;;
			* ) echo "Please answer (Y)es or (N)o.";;
		esac
	done
fi

# Clean up previous coverage reports
rm -fr target/coverage

# Run tests with coverage enabled
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target/coverage/cargo-test-%p-%m.profraw' cargo test --locked

# ** Test Coverage **
# Exclusions:
# - Lines containing `// coverage-ignore-line`
# - Lines between `// coverage-ignore-begin` and `// coverage-ignore-end`, inclusive
# - Everything that follows a `mod tests` block (Using notes from from jcdickinson at https://github.com/mozilla/grcov/pull/416#issuecomment-612562052; note that the exclusion zone begins in the file at the test block, and ends at the end of the file)

# Turn coverage results into lcov files
echo "Collecting code coverage..."
grcov target/coverage --binary-path ./target/debug/deps/ -s functions -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" --excl-br-start "mod tests \{|// coverage-ignore-begin" --excl-start "mod tests \{|// coverage-ignore-begin" --excl-br-stop '// coverage-ignore-end' --excl-stop '// coverage-ignore-end' --excl-br-line '// coverage-ignore-line' --excl-line '// coverage-ignore-line' -o target/coverage/lcov.info

# Print a nice coverage report table
grcov target/coverage --binary-path ./target/debug/deps/ -s functions -t markdown --branch --ignore-not-existing --ignore '../*' --ignore "/*" --excl-br-start "mod tests \{|// coverage-ignore-begin" --excl-start "mod tests \{|// coverage-ignore-begin" --excl-br-stop '// coverage-ignore-end' --excl-stop '// coverage-ignore-end' --excl-br-line '// coverage-ignore-line' --excl-line '// coverage-ignore-line' -o target/coverage/coverage.markdown
cat target/coverage/coverage.markdown
