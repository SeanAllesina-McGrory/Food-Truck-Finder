all:
	echo "All"

clean:
	find .\server\( -name target -o -name target-trunk -o -name dist \) -type d | xargs rm -rf
