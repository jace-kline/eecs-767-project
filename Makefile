CPP := g++
CPP_FLAGS := --std=c++17 -lstdc++fs -Wall

.PHONY: clean

# all: scrape.out

filesystem.out: filesystem.cpp
	$(CPP) $(CPP_FLAGS) $< -o $@

scrape.out: scrape.cpp
	$(CPP) $(CPP_FLAGS) $< -o $@

clean:
	rm -rf *.o *.out


# CC := g++
# CFLAGS := -Wall -g
# TARGET := test

# # $(wildcard *.cpp /xxx/xxx/*.cpp): get all .cpp files from the current directory and dir "/xxx/xxx/"
# SRCS := $(wildcard *.cpp)
# # $(patsubst %.cpp,%.o,$(SRCS)): substitute all ".cpp" file name strings to ".o" file name strings
# OBJS := $(patsubst %.cpp,%.o,$(SRCS))

# all: $(TARGET)
# $(TARGET): $(OBJS)
# 	$(CC) -o $@ $^
# %.o: %.cpp
# 	$(CC) $(CFLAGS) -c $<
# clean:
# 	rm -rf $(TARGET) *.o
	
# .PHONY: all clean