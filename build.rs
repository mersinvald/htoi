extern crate cc;

fn main() {
    cc::Build::new()
        .file("htoi.c")
        .opt_level(3)
        .compile("libhtoi.a")
}
