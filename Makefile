mac: FORCE
	cd tray ; make clean
	cd tray ; make zt_lib
	cd ui ; yarn build
	cargo build --release
	cp -f target/release/zerotier_desktop_ui mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier
	cp -f ui/dist/index.html mac-app-template/ZeroTier.app/Contents/Resources/ui.html
	cp -f ui/node_modules/@elastic/eui/dist/eui_theme_amsterdam_dark.css mac-app-template/ZeroTier.app/Contents/Resources/dark.css
	cp -f ui/node_modules/@elastic/eui/dist/eui_theme_amsterdam_light.css mac-app-template/ZeroTier.app/Contents/Resources/light.css

# This is a shortcut to rebuild only the web UI and run the main webview window. It only works on Macs.
# We use it for edit, test, edit, test cycle sort of UI development.
mac-ui-test: FORCE
	cd ui ; yarn build
	cp -f ui/dist/index.html mac-app-template/ZeroTier.app/Contents/Resources/ui.html
	cp -f ui/node_modules/@elastic/eui/dist/eui_theme_amsterdam_dark.css mac-app-template/ZeroTier.app/Contents/Resources/dark.css
	cp -f ui/node_modules/@elastic/eui/dist/eui_theme_amsterdam_light.css mac-app-template/ZeroTier.app/Contents/Resources/light.css
	mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier window Main 1280 330

clean: FORCE
	rm -rf target web-view/target ui/dist mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier mac-app-template/ZeroTier.app/Contents/Resources/ui.html tray/*.o tray/*.a

FORCE:
