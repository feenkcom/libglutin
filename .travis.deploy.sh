#!/bin/bash
if [[ $TRAVIS_COMMIT_MESSAGE == *"[skip deploy]"* ]]
then
	echo "Skipping deploy stage"
else
	travis_fold start "Deploy"
	
	ls -la
	
	cd release
	
	if [[ $TRAVIS_OS_NAME == "osx" ]]; then
		scp libGlutin.dylib $FEENK_CLOUD:/var/www/html/Glutin/osx/development/x86_64
	fi
	if [[ $TRAVIS_OS_NAME == "linux" ]]; then
		scp libGlutin.so $FEENK_CLOUD:/var/www/html/Glutin/linux/development/x86_64
	fi
	
	if [[ $TRAVIS_OS_NAME == "windows" ]]; then
		scp libGlutin.dll $FEENK_CLOUD:/var/www/html/Glutin/windows/development/x86_64
	fi

	travis_fold end "Deploy"
fi
