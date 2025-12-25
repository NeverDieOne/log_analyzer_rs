all:
	@cargo clippy -- -W clippy::all

pedantic:
	@cargo clippy -- -W clippy::pedantic
