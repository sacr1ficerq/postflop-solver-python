build:
	RUSTFLAGS="-W dangerous_implicit_autorefs -A warnings" maturin build --release

dev:
	RUSTFLAGS="-W dangerous_implicit_autorefs -A warnings" maturin develop --release

bench:
	time python -c "import oxipostflop as ox; ox.demonstrate();"
