compile:
	(cd goofspiel && cargo build)

run_after_compile:
	goofspiel/target/debug/goofspiel

run:
	(cd goofspiel && cargo run)

fmt:
	rustfmt goofspiel/src/main.rs
