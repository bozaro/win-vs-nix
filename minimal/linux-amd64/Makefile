all: minimal
clean:
	rm -f minimal.o minimal
minimal: minimal.o
	ld -e _start -o minimal minimal.o && strip minimal
minimal.o: minimal.asm
	nasm -f elf64 minimal.asm
