CODESIGN=echo
PRODUCTSIGN=echo
CODESIGN_APP_CERT=
CODESIGN_INSTALLER_CERT=
NOTARIZE=echo
NOTARIZE_USER_ID=null

ifeq ($(ZT_OFFICIAL_RELEASE),1)
        CODESIGN=codesign
        PRODUCTSIGN=productsign
        CODESIGN_APP_CERT="Developer ID Application: ZeroTier, Inc (8ZD9JUCZ4V)"
        CODESIGN_INSTALLER_CERT="Developer ID Installer: ZeroTier, Inc (8ZD9JUCZ4V)"
        NOTARIZE=xcrun altool
        NOTARIZE_USER_ID="adam.ierymenko@gmail.com"
endif

mac: FORCE
	cd tray ; make clean
	cd tray ; make zt_lib
	cd ui ; yarn build
	MACOSX_DEPLOYMENT_TARGET=10.13 cargo build --release
	cp -f target/release/zerotier_desktop_ui mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier
	cp -f ui/dist/index.html mac-app-template/ZeroTier.app/Contents/Resources/ui.html
	cp -f ui/dist/dark.css mac-app-template/ZeroTier.app/Contents/Resources/dark.css
	cp -f ui/dist/light.css mac-app-template/ZeroTier.app/Contents/Resources/light.css
	cd mac-app-template ; $(CODESIGN) -f --options=runtime -s $(CODESIGN_APP_CERT) ZeroTier.app

# This is a shortcut to rebuild only the web UI and run the main webview window. It only works on Macs.
# We use it for edit, test, edit, test cycle sort of UI development.
mac-test-webui: FORCE
	cd ui ; yarn build
	cp -f ui/dist/index.html mac-app-template/ZeroTier.app/Contents/Resources/ui.html
	cp -f ui/dist/dark.css mac-app-template/ZeroTier.app/Contents/Resources/dark.css
	cp -f ui/dist/light.css mac-app-template/ZeroTier.app/Contents/Resources/light.css
	mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier

clean: FORCE
	rm -rf target web-view/target mac-app-template/ZeroTier.app/Contents/MacOS/ZeroTier mac-app-template/ZeroTier.app/Contents/Resources/*.html mac-app-template/ZeroTier.app/Contents/Resources/*.css mac-app-template/ZeroTier.app/Contents/_CodeSignature tray/*.o tray/*.a

FORCE:
