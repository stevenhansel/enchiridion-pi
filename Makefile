arch = $(shell rustc -Vv | grep host | cut -f2 -d' ')

.PHONY: build
build:
	pyinstaller -F camera/main.py --distpath src-tauri/bin --clean -n camera-$(arch)
