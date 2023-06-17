#!/bin/bash

BOOTSTRAP_VERSION="v5.2.3"
BOOTSTRAP_ICONS_VERSION="1.10.5"

echo "Fetching Bootstrap {$BOOTSTRAP_VERSION}"
if [ -d "vendor/bootstrap" ]
then
  (cd vendor/bootstrap || exit 1; git pull origin)
else
  git clone https://github.com/twbs/bootstrap.git vendor/bootstrap
fi
(cd vendor/bootstrap; git checkout $BOOTSTRAP_VERSION)


echo "Fetching underscore.js"
if [ -d "js/src/vendor/underscore" ]
then
  (cd js/src/vendor/underscore || exit 1; git pull origin)
else
  git clone https://github.com/jashkenas/underscore.git js/src/vendor/underscore
fi

echo "Fetching D3"
if [ -d "js/src/vendor/d3" ]
then
  (cd vendor/js/src/vendor/d3 || exit 1; git pull origin)
else
  git clone https://github.com/d3/d3.git js/src/vendor/d3
fi

echo "Fetching Bootstrap icons"
if [ -f temp/bootstrap-icons-${BOOTSTRAP_ICONS_VERSION}.zip ]
then
  rm temp/bootstrap-icons-${BOOTSTRAP_ICONS_VERSION}.zip
fi
wget https://github.com/twbs/icons/releases/download/v1.10.5/bootstrap-icons-${BOOTSTRAP_ICONS_VERSION}.zip -P temp/
