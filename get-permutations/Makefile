run: target/release/get-permutations a.out d.out
	@echo
	@echo ---- Rust ----
	time target/release/get-permutations
	@echo
	@echo ---- C++ ----
	time ./a.out
	@echo
	@echo ---- D ----
	time ./d.out

target/release/get-permutations: src/main.rs
	cargo build --release
a.out: src/main.cpp
	g++ -std=c++11 -O3 src/main.cpp
d.out: src/main.d
	ldc --release -O3 src/main.d --of=d.out
