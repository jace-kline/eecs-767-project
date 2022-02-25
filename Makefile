CPP := g++
CPP_FLAGS := --std=c++17 -lstdc++fs -Wall

.PHONY: all clean

all: scrape.out

scrape.out: scrape.cpp
	$(CPP) $(CPP_FLAGS) scrape.cpp -o scrape.out

clean:
	rm -rf *.o *.out