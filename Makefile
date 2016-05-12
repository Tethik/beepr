default: build

install: copy_bin copy_sounds

build:
	cargo build --release

copy_sounds:
	cp sounds/ -r /usr/share/beepr/

copy_bin:
	cp target/release/beepr /usr/bin

clean:
	cargo clean
