.PHONY: release

release:
	# build from scratch to ensure version in binary is updated
	rm -rf target/;
	cargo release $@;
