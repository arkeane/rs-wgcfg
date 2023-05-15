build:
	cargo build

release:
	cargo build --release

install: release
	cargo install --path .

uninstall:
	cargo uninstall

clean:
	cargo clean