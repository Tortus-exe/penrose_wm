all: build

.PHONY: build
build:
	$(shell [[ $EUID -eq 0 ]] && echo "build cannot be run as root" && exit 1)
	@echo ":: Building release..."
	@cargo build --release

.PHONY: build-debug
	$(shell [[ $EUID -eq 0 ]] && echo "build cannot be run as root" && exit 1)
	@echo ":: Building a debug binary..."
	@cargo build

.PHONY: install-helpers
install-helpers:
	@echo ":: Installing ./bin..."
	@mkdir -p /usr/local/bin
	@cp -R bin/. /usr/local/bin
	@ls bin | xargs -I {} chmod 755 /usr/local/bin/{}
	@echo ":: Copying over xsession file..."
	@cp penrose.desktop /usr/share/xsessions/

.PHONY: install-penrose-release
install-penrose-release:
	@echo ":: Installing release build of penrose..."
	@mkdir -p /usr/local/bin
	@cp -f target/release/penrose_wm /usr/local/bin
	@chmod 755 /usr/local/bin/penrose_wm

.PHONY: install-penrose-debug
install-penrose-debug:
	@echo ":: Installing debug build of penrose..."
	@strip target/debug/penrose_wm
	@mkdir -p /usr/local/bin
	@cp -f target/debug/penrose_wm /usr/local/bin
	@chmod 755 /usr/local/bin/penrose_wm

.PHONY: install
install: install-penrose-release install-helpers
	@echo ":: Done"

.PHONY: install-debug
install-debug: install-penrose-debug install-helpers
	@echo ":: Done"

.PHONY: uninstall
uninstall:
	@echo ":: Removing binaries..."
	@ls bin | xargs -I {} rm -f /usr/local/bin/{}
	@rm -f /usr/local/bin/penrose_wm
	@echo ":: Done"