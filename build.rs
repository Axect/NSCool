fn main() {
    cc::Build::new()
        .file("src/legacy/tc.c")
        .object("src/legacy/tc.o")
        .compile("tc");

    cc::Build::new()
        .file("src/legacy/boundary.c")
        .object("src/legacy/boundary.o")
        .compile("boundary");

    cc::Build::new()
        .file("src/legacy/conductivity_crust.c")
        .object("src/legacy/conductivity_crust.o")
        .compile("conductivity_crust");
}