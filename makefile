DIR_NAME = day$(day)

create_day:
	@if [ -d $(DIR_NAME) ]; then\
		echo "Directory already exists, will not overwrite!";\
	else\
		cp -r template $(DIR_NAME);\
		touch $(DIR_NAME)/example_input1.txt;\
		touch $(DIR_NAME)/example_input2.txt;\
		echo "Template copied to $(DIR_NAME)";\
	fi

