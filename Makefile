mac:
	cd tray ; make clean
	cd tray ; make zt_lib
	cd ui ; yarn build
	cargo build --release
	cp -f ui/dist/index.html mac-app-template/ZeroTier.app/Contents/Resources/ui.html
	cp -f target/release/zerotier_desktop_ui mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier
	cp -f ui/node_modules/@elastic/eui/dist/eui_theme_amsterdam_dark.css mac-app-template/ZeroTier.app/Contents/Resources/dark.css
	cp -f ui/node_modules/@elastic/eui/dist/eui_theme_amsterdam_light.css mac-app-template/ZeroTier.app/Contents/Resources/light.css

clean:
	rm -rf target web-view/target ui/dist mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier mac-app-template/ZeroTier.app/Contents/Resources/ui.html tray/*.o tray/*.a
