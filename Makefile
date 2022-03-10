ifeq ($(OS),Windows_NT)
	CC=gcc
	MAINTARGET=windows
else ifeq ($(shell uname -s),Linux)
	MAINTARGET=linux
else ifeq ($(shell uname -s),Darwin)
	MAINTARGET=mac
endif

CODESIGN=echo
CODESIGN_APP_CERT=
ifeq ($(ZT_OFFICIAL_RELEASE),1)
	CODESIGN=codesign
	CODESIGN_APP_CERT="Developer ID Application: ZeroTier, Inc (8ZD9JUCZ4V)"
	CARGO_FLAGS=--release
	CARGO_TARGET_DIR=release
else
	CARGO_FLAGS=
	CARGO_TARGET_DIR=debug
endif

all:    $(MAINTARGET)

windows: FORCE
	make -C tray clean
	make -C tray zt_lib
	cargo build $(CARGO_FLAGS) --target=x86_64-pc-windows-msvc
	make -C tray clean
	make -C tray zt_lib WIN_32BIT=1
	set "RUSTFLAGS=-C link-args=/SAFESEH:NO" && cargo build $(CARGO_FLAGS) --target=i686-pc-windows-msvc

linux: FORCE
	cd tray ; make clean
	cd tray ; make zt_lib
	cargo build $(CARGO_FLAGS)

mac: FORCE
	cd tray ; make clean
	cd tray ; make -j2 zt_lib
	MACOSX_DEPLOYMENT_TARGET=10.13 cargo build $(CARGO_FLAGS) --target=aarch64-apple-darwin
	MACOSX_DEPLOYMENT_TARGET=10.13 cargo build $(CARGO_FLAGS) --target=x86_64-apple-darwin
	lipo -create target/aarch64-apple-darwin/$(CARGO_TARGET_DIR)/zerotier_desktop_ui target/x86_64-apple-darwin/$(CARGO_TARGET_DIR)/zerotier_desktop_ui -output target/$(CARGO_TARGET_DIR)/zerotier_desktop_ui
	make mac-assemble-app

mac-assemble-app: FORCE
	rm -rf ZeroTier.app
	cp -av mac-app-template/ZeroTier.app .
	mkdir -p ZeroTier.app/Contents/MacOS
	cp -f target/$(CARGO_TARGET_DIR)/zerotier_desktop_ui ZeroTier.app/Contents/MacOS/ZeroTier
	cp -f ui/dist/index.html ZeroTier.app/Contents/Resources/ui.html
	cp -f ui/dist/dark.css ZeroTier.app/Contents/Resources/dark.css
	cp -f ui/dist/light.css ZeroTier.app/Contents/Resources/light.css
	xattr -cr ZeroTier.app
	$(CODESIGN) -f --options=runtime -s $(CODESIGN_APP_CERT) ZeroTier.app

ui: FORCE
	cd ui ; yarn build

ifeq ($(OS),Windows_NT)
clean: FORCE
	-make -C tray clean
	-rmdir /Q /S target
else
clean: FORCE
	rm -f tray/*.o tray/*.a tray/example tray/example.exe
	rm -rf ZeroTier.app target
endif

ubuntudeb_64:	linux
	fpm -t deb \
	-p zerotier-ui.0.1.0-1.amd64.deb \
	--version 0.1.0-1 \
	--architecture amd64

FORCE:
