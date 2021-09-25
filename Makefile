ptimer:
	gcc main.c
	mv ./a.out ./build/ptimer

install_bin: ptimer
	cp ./build/ptimer /usr/local/bin/ptimer
