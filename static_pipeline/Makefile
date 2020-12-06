STATIC_PATH=../rs/abenga_site/static/

all:	css fonts js

fonts:	vendor/font-awesome-4.7.0/fonts/FontAwesome.otf
	cp vendor/font-awesome-4.7.0/fonts/* $(STATIC_PATH)fonts/fontawesome/

css:	vendor/bootstrap/scss
	sass --style compressed scss/custom.scss $(STATIC_PATH)css/abenga_site.min.css

js:	vendor/bootstrap/dist/js/bootstrap.min.js js/src/vendor/*
	cp vendor/bootstrap/dist/js/bootstrap.min.js $(STATIC_PATH)js/lib/vendor/
	cp vendor/bootstrap/dist/js/bootstrap.min.js.map $(STATIC_PATH)js/lib/vendor/
	cp js/src/vendor/jquery/jquery-3.5.1.min.js $(STATIC_PATH)js/lib/vendor/
	cp js/src/vendor/underscore/underscore-min.js $(STATIC_PATH)js/lib/vendor/
	cp js/src/vendor/backbone/backbone.js $(STATIC_PATH)js/lib/vendor/
	cp js/src/vendor/d3/d3.min.js $(STATIC_PATH)js/lib/vendor/
