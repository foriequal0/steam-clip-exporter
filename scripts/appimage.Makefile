APPIMAGE = steam-clip-exporter.AppImage

BUILD_ROOT = build

# linuxdeploy plugin settings
export DEPLOY_GTK_VERSION = 4
export LDAI_OUTPUT=$(BUILD_ROOT)/$(APPIMAGE)

$(BUILD_ROOT)/$(APPIMAGE): $(BUILD_ROOT)/AppDir $(BUILD_ROOT)/linuxdeploy.AppImage
	$(BUILD_ROOT)/linuxdeploy.AppImage \
		--appdir $(BUILD_ROOT)/AppDir \
		--output appimage

$(BUILD_ROOT)/AppDir: $(BUILD_ROOT)
	meson install -C $(BUILD_ROOT) --destdir=AppDir

$(BUILD_ROOT):
	meson setup $(BUILD_ROOT) --prefix=/usr

$(BUILD_ROOT)/linuxdeploy.AppImage:
	wget -O $@ https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
	chmod +x $@
