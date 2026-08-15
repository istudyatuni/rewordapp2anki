name := "rewordapp2anki"
win-target := "x86_64-pc-windows-msvc"
linux-target := "x86_64-unknown-linux-musl"

[private]
@default:
	just --list

@build-all: check-xwin build-linux build-win
	echo Built linux and windows binaries:
	fd -d 1 '{{ name }}-' target

# build static linux binary
build-linux: && pack-linux
	@# CARGO_HOME and /tmp/.cargo is used to use local cargo download cache
	docker run --rm -it \
		-v "$(pwd)":/build \
		-v "$HOME/.cargo":/tmp/.cargo \
		-w /build \
		--env=CARGO_HOME=/tmp/.cargo \
		--env=RUSTFLAGS="--remap-path-prefix /tmp=/build" \
		ghcr.io/rust-cross/rust-musl-cross:x86_64-musl \
		cargo build --release \
			--features sqlite-bundled \
			--target={{ linux-target }} \
			--config build.rustc-wrapper="''"

# build static windows binary
build-win: check-xwin && pack-win
	RUSTFLAGS="--remap-path-prefix $HOME=/build" \
	cargo xwin b --release --target={{ win-target }} --features sqlite-bundled

[private]
pack-linux:
	rm -f "target/{{ name }}-linux.tar.gz"
	tar -C "target/{{ linux-target }}/release" -czf "target/{{ name }}-linux.tar.gz" "{{ name }}"

[private]
pack-win:
	rm -f "target/{{ name }}-windows.zip"
	cd "target/{{ win-target }}/release" && zip "../../{{ name }}-windows.zip" "{{ name }}.exe"

[private]
check-xwin:
	which cargo-xwin
