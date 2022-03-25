CPP := g++
CPP_FLAGS := --std=c++2a -fconcepts -lstdc++fs -Wall -g

TARGET := test

SRCDIRS := $(shell find . -type d)
SRCS := $(wildcard $(addsuffix /*.cpp,$(SRCDIRS)))
# $(wildcard *.cpp /xxx/xxx/*.cpp): get all .cpp files from the current directory and dir "/xxx/xxx/"
# SRCS := $(wildcard *.cpp index/*.cpp score/*.cpp scrape/*.cpp shared/*.cpp text/*.cpp)
# $(patsubst %.cpp,%.o,$(SRCS)): substitute all ".cpp" file name strings to ".o" file name strings
OBJS := $(patsubst %.cpp,%.o,$(SRCS))

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CPP) $(CPP_FLAGS) $^ -o $@

%.o: %.cpp
	$(CPP) $(CPP_FLAGS) -c $< -o $@

clean:
	rm -rf $(TARGET) $(OBJS) *.o