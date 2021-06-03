mac:
	cd tray ; make clean ; make
	cd ui ; yarn build
	cargo build --release
	cp -f ui/dist/index.html mac-app-template/ZeroTier.app/Contents/Resources/ui.html
	cp -f target/release/zerotier_desktop_ui mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier

clean:
	rm -rf target web-view/target ui/dist mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier mac-app-template/ZeroTier.app/Contents/Resources/ui.html tray/*.o tray/*.a
