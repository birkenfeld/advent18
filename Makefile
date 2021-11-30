SHELL = /bin/bash
MODE = release
ifeq ($(MODE),release)
	MODEARG = --release
endif

all:
	@cargo build $(MODEARG)
	@for s in src/bin/day*; do t=`basename $$s .rs`; echo -e '\n\x1b[01m'$$t'\x1b[0m'; target/$(MODE)/$$t || exit 1; done
time:
	@cargo build $(MODEARG)
	@for s in src/bin/day*; do t=`basename $$s .rs`; echo -e '\n\x1b[01m'$$t'\x1b[0m'; perf stat --null target/$(MODE)/$$t 2>&1 | grep elapsed; done
