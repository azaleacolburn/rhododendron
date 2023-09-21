SOURCES := $(wildcard ./*.c)

fcc: $(SOURCES)
	gcc $(SOURCES) -o fcc -Wno-switch