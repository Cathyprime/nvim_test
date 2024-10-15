mya:
	cargo build --release
	mkdir -p lua/
	cp target/release/libnvim_test.so lua/nvim_test.so
