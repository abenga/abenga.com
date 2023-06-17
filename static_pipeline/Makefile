STATIC_PATH=../rs/abenga_site/assets/
BOOTSTRAP_ICONS_VERSION=1.10.5

all:	fetch css js fonts

local:	css js fonts

fetch:	fetch_dependencies.sh
	./fetch_dependencies.sh

css:	vendor/bootstrap/scss/_variables.scss scss/abenga_site.scss
	sass --style compressed scss/custom.scss $(STATIC_PATH)css/abenga_site.min.css

js:	vendor/bootstrap/dist/js/bootstrap.min.js
	cp vendor/bootstrap/dist/js/bootstrap.min.js $(STATIC_PATH)js/lib/vendor/
	cp vendor/bootstrap/dist/js/bootstrap.min.js.map $(STATIC_PATH)js/lib/vendor/

fonts:	temp/bootstrap-icons-$(BOOTSTRAP_ICONS_VERSION).zip
	rm -r fonts/*
	unzip -q -d fonts/ temp/bootstrap-icons-$(BOOTSTRAP_ICONS_VERSION).zip
	cp -r fonts/bootstrap-icons-$(BOOTSTRAP_ICONS_VERSION)/font/fonts/* $(STATIC_PATH)fonts/bootstrap-icons/
