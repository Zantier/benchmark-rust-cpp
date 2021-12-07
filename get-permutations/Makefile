run: target/release/get-permutations a.out
	@echo
	@echo ---- Rust ----
	time target/release/get-permutations
	@echo
	@echo ---- C++ ----
	time ./a.out

target/release/get-permutations: src/main.rs
	cargo build --release
a.out: src/main.cpp
	g++ -std=c++11 -O3 src/main.cpp
