GIR = gir/target/bin/gir
GIR_SRC = gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
GIR_FILES = gir-files/WebKit2-4.0.gir

# Run `gir` generating the bindings
gir : src/auto/mod.rs

gir-sys : webkit2gtk-sys/src/lib.rs

regen_check: $(GIR) $(GIR_FILES)
	rm src/auto/*
	rm webkit2gtk-sys/tests/*
	$(GIR) -c Gir.toml
	$(GIR) -c webkit2gtk-sys/Gir.toml
	git diff -R --exit-code

src/auto/mod.rs : Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir.toml

webkit2gtk-sys/src/lib.rs : webkit2gtk-sys/Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c webkit2gtk-sys/Gir.toml

$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init
