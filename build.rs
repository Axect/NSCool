fn main() {
    cc::Build::new()
        .file("src/legacy/tc.c")
        .object("src/legacy/tc.o")
        .compile("tc");
}