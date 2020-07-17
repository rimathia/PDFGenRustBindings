extern crate pdfgen_bindings;
use pdfgen_bindings::*;
use std::ffi::CString;
use std::io::Read;
fn main() {
    let info = PDFInfo {
        creator: [0; 64],
        producer: [0; 64],
        title: [0; 64],
        author: [0; 64],
        subject: [0; 64],
        date: [0; 64],
    };

    let filename = CString::new("penguin.pdf").expect("?");
    let mut penguin = Vec::<u8>::new();
    std::fs::File::open("data/penguin.jpg")
        .unwrap()
        .read_to_end(&mut penguin).unwrap();

    unsafe {
        let pdf = pdf_create(A4_WIDTH, A4_HEIGHT, &info);
        for _ in 0..2 {
            let page = pdf_append_page(pdf);
            pdf_add_jpeg_data(
                pdf,
                page,
                mm_to_point(13.0),
                mm_to_point(18.0),
                mm_to_point(3.0 * 87.0 / 680.0 * 480.0),
                mm_to_point(3.0 * 87.0),
                penguin.as_slice().as_ptr(),
                penguin.len(),
        );
    }
        pdf_save(pdf, filename.as_ptr());
        pdf_destroy(pdf);
    }
}
