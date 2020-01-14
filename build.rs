fn main() {
    cc::Build::new()
        .file("src/legacy/tc.c")
        .object("src/legacy/tc.o")
        .compile("tc");

    cc::Build::new()
        .file("src/legacy/boundary.c")
        .object("src/legacy/boundary.o")
        .compile("boundary");
}