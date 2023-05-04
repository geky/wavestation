.PHONY: all build
all build:
	cargo build --release
	cp target/release/wavestation wavestation

.PHONY: run
run:
	./wavestation 64

.PHONY: clean
clean:
	cargo clean
	rm -f wavestation
