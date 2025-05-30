#!/bin/bash

hello1_txt_test="/home/$USER/Schreibtisch/hello1.txt.adel"
hello1_dir_test="/home/$USER/Schreibtisch/hellodir1.adel"

hello2_txt_test="/home/$USER/Downloads/hello2.txt.adel"
hello2_dir_test="/home/$USER/Downloads/hellodir2.adel"

while true; do
	if [[ ! -f "$hello1_txt_test" ]]; then
		printf "Test file created: %s\n" "$hello1_txt_test"
		touch $hello1_txt_test
	fi

	if [[ ! -f "$hello2_txt_test" ]]; then
		printf "Test file created: %s\n" "$hello2_txt_test"
		touch $hello2_txt_test
	fi

	if [[ ! -d "$hello1_dir_test" ]]; then
		printf "Test dir created: %s\n" "$hello1_dir_test"
		mkdir $hello1_dir_test
	fi

	if [[ ! -d "$hello2_dir_test" ]]; then
		printf "Test dir created: %s\n" "$hello2_dir_test"
		mkdir $hello2_dir_test
	fi

	sleep 1
done
