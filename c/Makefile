TARGET = c-fib
OUTDIR = ./

COMMONFILES =
SRCS = main.c
HEADERS :=  $(shell find . -type f -name '*.h')
OBJS = $(SRCS:.c=.o)

OUTOBJS = $(addprefix $(OUTDIR)/,$(OBJS))

# set CXX if not defines
# you can override it by `CXX=gcc make run`
CXX ?= clang
CXXFLAGS = -std=c11 -Wall -O2 -I.

.PHONY: default build run clean

default: $(TARGET)

$(TARGET): build

build: $(OUTOBJS)
	$(CXX) $(CXXFLAGS) $^ -o $(OUTDIR)/$(TARGET) $(LDFLAGS)

$(OUTDIR)/%.o : %.c | $(OUTDIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(OUTDIR):
	mkdir -p $(OUTDIR)

run: $(TARGET)
	$(OUTDIR)/$(TARGET)

clean:
	rm -rf $(OUTDIR)/*.o $(OUTDIR)/$(TARGET)
