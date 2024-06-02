TARGET = go-fib
OUTDIR = ./

COMMONFILES =
SRCS = main.go

.PHONY: default build run clean

default: $(TARGET)

$(TARGET): build

build: $(SRCS)
	# -l: inlining
	# -B: remove bounds check (risky)
	# -w: remove debug info
	# -s: remove symbol table
	# go build -a -gcflags=all="-l -B" -ldflags "-s -w" -o $(OUTDIR)/$(TARGET)
	go build -o $(OUTDIR)/$(TARGET)

run: $(TARGET)
	$(OUTDIR)/$(TARGET)

clean:
	rm -rf $(OUTDIR)/$(TARGET)
