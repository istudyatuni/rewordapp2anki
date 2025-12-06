name := "rewordapp2anki"
win-target := "x86_64-pc-windows-msvc"
linux-target := "x86_64-unknown-linux-musl"

[private]
@default:
	just --list

build-all: build-linux build-win

# build static binary
build-linux: && pack-linux
	@# CARGO_HOME and /tmp/.cargo is used to use local cargo download cache
	docker run --rm -it \
		-v "$(pwd)":/build \
		-v "$HOME/.cargo":/tmp/.cargo \
		-w /build \
		--env=CARGO_HOME=/tmp/.cargo \
		ghcr.io/rust-cross/rust-musl-cross:x86_64-musl \
		cargo build --release \
			--features sqlite-bundled \
			--target={{ linux-target }} \
			--config build.rustc-wrapper="''"

build-win: && pack-win
	cargo xwin b --release --target={{ win-target }} --features sqlite-bundled

[private]
pack-linux:
	rm -f "target/{{ name }}-linux.tar.gz"
	tar -C "target/{{ linux-target }}/release" -czf "target/{{ name }}-linux.tar.gz" "{{ name }}"

[private]
pack-win:
	rm -f "target/{{ name }}-windows.zip"
	cd "target/{{ win-target }}/release" && zip "../../{{ name }}-windows.zip" "{{ name }}.exe"
