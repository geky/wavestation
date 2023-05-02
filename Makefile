.PHONY: all build
all build:
	cargo build
	cp target/debug/wavestation wavestation

.PHONY: run
run:
	./wavestation 64

.PHONY: clean
clean:
	cargo clean
	rm wavestation
