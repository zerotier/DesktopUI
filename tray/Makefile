ifeq ($(OS),Windows_NT)
	TRAY_CFLAGS := -O -DTRAY_WINAPI=1 -std=c99
#	TRAY_LDFLAGS :=
else ifeq ($(shell uname -s),Linux)
	TRAY_CFLAGS := -O -DTRAY_APPINDICATOR=1 $(shell pkg-config --cflags appindicator3-0.1) -std=c99
#	TRAY_LDFLAGS := $(shell pkg-config --libs appindicator3-0.1)
else ifeq ($(shell uname -s),Darwin)
	TRAY_CFLAGS := -Os -DTRAY_APPKIT=1 -DOBJC_OLD_DISPATCH_PROTOTYPES=1 -std=c99 -mmacosx-version-min=10.13
#	TRAY_LDFLAGS := -framework Cocoa
endif

CFLAGS := $(TRAY_CFLAGS)
#LDFLAGS := $(TRAY_LDFLAGS)

all: zt_lib

zt_lib: zt_desktop_tray.o
	ar rcs libzt_desktop_tray.a zt_desktop_tray.o

#example: example.o
#	$(CC) $^ $(LDFLAGS) -o $@
#
#example.o: example.c tray.h

clean:
	rm -f *.o *.a
