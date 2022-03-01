CPP := g++
CPP_FLAGS := --std=c++17 -lstdc++fs -Wall -g

TARGET := test
# $(wildcard *.cpp /xxx/xxx/*.cpp): get all .cpp files from the current directory and dir "/xxx/xxx/"
SRCS := $(wildcard *.cpp)
# $(patsubst %.cpp,%.o,$(SRCS)): substitute all ".cpp" file name strings to ".o" file name strings
OBJS := $(patsubst %.cpp,%.o,$(SRCS))

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CPP) $(CPP_FLAGS) $^ -o $@

%.o: %.cpp
	$(CPP) $(CPP_FLAGS) -c $<

clean:
	rm -rf $(TARGET) *.o