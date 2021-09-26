rust_build:
	cd rust_version/ptimer; \
		cargo build --release; \
		cp target/release/ptimer ../../build/ptimer

install_rust_bin: rust_build
	cp ./build/ptimer /usr/local/bin/ptimer

ptimer: rust_build

install_bin: ptimer install_rust_bin

