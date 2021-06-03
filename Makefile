mac-debug:
	cd tray ; make clean ; make
	cd ui ; yarn build
	cargo build
	cp -f target/debug/zerotier_desktop_ui mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier

clean:
	rm -rf target ui/dist
	rm -f mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier tray/*.o
