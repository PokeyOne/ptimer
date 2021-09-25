ptimer:
	gcc main.c
	mv ./a.out ./build/ptimer

install_bin: ptimer
	cp ./build/ptimer /usr/local/bin/ptimer

rust_build:
	cd rust_version/ptimer; \
		cargo build --release; \
		cp target/release/ptimer ../../build/ptimer

install_rust_bin: rust_build
	cp ./build/ptimer /usr/local/bin/ptimer
