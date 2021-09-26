ptimer:
	cargo build --release
	cp target/release/ptimer ./build/ptimer

install_bin: ptimer
	cp ./build/ptimer /usr/local/bin/ptimer

