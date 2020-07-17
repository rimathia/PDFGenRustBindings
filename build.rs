extern crate cc;
fn main() {
    println!("cargo:rerun-if-changed=src/pdfgen.c");
    println!("cargo:rerun-if-changed=src/pdfgen.h");

    cc::Build::new().file("src/pdfgen.c").compile("pdfgen");
}
