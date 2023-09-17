SOURCES := $(wildcard ./*.c)

compiler: $(SOURCES)
	gcc $(SOURCES) -o compiler -Wno-switch