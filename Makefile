release:
	cargo build --release
	sudo cp ./target/release/unilp /usr/bin/