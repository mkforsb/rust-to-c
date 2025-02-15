.PHONY: all clean

all:
	gcc -c library.c
	ar rcs ../rust-app/liblibrary.a library.o

clean:
	rm -f library.o
