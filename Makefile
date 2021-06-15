mac: FORCE
	cd tray ; make clean
	cd tray ; make zt_lib
	cd ui ; yarn build
	MACOSX_DEPLOYMENT_TARGET=10.13 cargo build --release
	cp -f target/release/zerotier_desktop_ui mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier
	cp -f ui/dist/index.html mac-app-template/ZeroTier.app/Contents/Resources/ui.html
	cp -f ui/dist/dark.css mac-app-template/ZeroTier.app/Contents/Resources/dark.css
	cp -f ui/dist/light.css mac-app-template/ZeroTier.app/Contents/Resources/light.css

# This is a shortcut to rebuild only the web UI and run the main webview window. It only works on Macs.
# We use it for edit, test, edit, test cycle sort of UI development.
mac-ui: FORCE
	cd ui ; yarn build
	cp -f ui/dist/index.html mac-app-template/ZeroTier.app/Contents/Resources/ui.html
	cp -f ui/dist/dark.css mac-app-template/ZeroTier.app/Contents/Resources/dark.css
	cp -f ui/dist/light.css mac-app-template/ZeroTier.app/Contents/Resources/light.css
	#mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier window Main 1280 360

clean: FORCE
	rm -rf target web-view/target mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier mac-app-template/ZeroTier.app/Contents/Resources/*.html mac-app-template/ZeroTier.app/Contents/Resources/*.css tray/*.o tray/*.a

FORCE:
