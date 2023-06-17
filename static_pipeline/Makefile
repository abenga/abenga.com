STATIC_PATH=../rs/abenga_site/assets/

all:	css js

#fonts:	vendor/font-awesome-4.7.0/fonts/FontAwesome.otf
#	cp vendor/font-awesome-4.7.0/fonts/* $(STATIC_PATH)fonts/fontawesome/

css:	vendor/bootstrap/scss
	sass --style compressed scss/custom.scss $(STATIC_PATH)css/abenga_site.min.css

js:	vendor/bootstrap/dist/js/bootstrap.min.js
	cp vendor/bootstrap/dist/js/bootstrap.min.js $(STATIC_PATH)js/lib/vendor/
	cp vendor/bootstrap/dist/js/bootstrap.min.js.map $(STATIC_PATH)js/lib/vendor/
