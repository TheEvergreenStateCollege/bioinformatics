all: howtotree

howtotree: howtotree.cpp
	g++ -o howtotree howtotree.cpp

test: howtotree
	./howtotree > output.txt
	diff output.txt sorted.txt 