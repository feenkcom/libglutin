#!/bin/bash
if [[ $TRAVIS_COMMIT_MESSAGE == *"[skip deploy]"* ]]
then
	echo "Skipping deploy stage"
else
	travis_fold start "Deploy"
	
	ls -la

	travis_fold end "Deploy"
fi
