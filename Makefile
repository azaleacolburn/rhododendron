SOURCES := $(wildcard ./*.c)

fcc: $(SOURCES)
	gcc $(SOURCES) -o rhododendron -Wno-switch