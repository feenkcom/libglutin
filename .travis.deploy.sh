#!/bin/bash
if [[ $TRAVIS_COMMIT_MESSAGE == *"[skip deploy]"* ]]
then
	echo "Skipping deploy stage"
else
	travis_fold start "Deploy"
	
	cd 
	
	if [[ $TRAVIS_OS_NAME == "osx" ]]; then
		scp libGlutin.so $FEENK_CLOUD:/var/www/html/Glutin/development/x86_64
	fi
	if [[ $TRAVIS_OS_NAME == "linux" ]]; then
		
	fi
	
	if [[ $TRAVIS_OS_NAME == "windows" ]]; then
		
	fi

	travis_fold end "Deploy"
fi
