extern crate pdfgen_bindings;
use pdfgen_bindings::*;
use std::ffi::CString;
fn main() {
    let info = PDFInfo {
        creator: [0; 64],
        producer: [0; 64],
        title: [0; 64],
        author: [0; 64],
        subject: [0; 64],
        date: [0; 64],
    };

    let filename = CString::new("empty.pdf").expect("?");
    unsafe {
        let pdf = pdf_create(A4_WIDTH, A4_HEIGHT, &info);
        pdf_append_page(pdf);
        pdf_save(pdf, filename.as_ptr());
        pdf_destroy(pdf);
    }
}
