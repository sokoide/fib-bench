TARGET = go-fib
OUTDIR = ./

COMMONFILES =
SRCS = main.go

.PHONY: default build run clean

default: $(TARGET)

$(TARGET): build

build: $(SRCS)
	go build -o $(OUTDIR)/$(TARGET)

run: $(TARGET)
	$(OUTDIR)/$(TARGET)

clean:
	rm -rf $(OUTDIR)/$(TARGET)
