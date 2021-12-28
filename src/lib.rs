extern crate libc;
use libc::{c_char, c_float, c_int, c_uchar, size_t};

/// Convert a value in inches into a number of points.
/// Always returns an integer value
/// @param inch inches value to convert to points
pub fn inch_to_point(inch: f32) -> f32 {
    inch * 72.0f32
}

/// Convert a value in milli-meters into a number of points.
/// Always returns an integer value
/// @param mm millimeter value to convert to points
pub fn mm_to_point(mm: f32) -> f32 {
    mm * 72.0f32 / 25.4f32
}

/// Point width of a standard A4 page
pub fn a4_width_points() ->f32 {
    mm_to_point(210.0f32)
}

/// Point height of a standard A4 page
pub fn a4_height_points() -> f32 { mm_to_point(297.0f32)}

/// pdf_info describes the metadata to be inserted into the header of the output PDF
#[repr(C)]
pub struct PDFInfo {
    /// Software used to create the PDF
    pub creator: [c_char; 64],
    /// Software used to create the PDF
    pub producer: [c_char; 64],
    /// The title of the PDF (typically displayed in the window bar when viewing)
    pub title: [c_char; 64],
    /// Who created the PDF
    pub author: [c_char; 64],
    /// What is the PDF about
    pub subject: [c_char; 64],
    /// The date the PDF was created
    pub date: [c_char; 64],
}

/// opaque type for a pdf document
#[repr(C)]
pub struct PDFDoc {
    _private: [u8; 0],
}

/// opaque type for a pdf object
#[repr(C)]
pub struct PDFObject {
    _private: [u8; 0],
}

#[link(name = "pdfgen", kind = "static")]
extern "C" {
    /// Create a new PDF object, with the given page
    /// width/height
    /// @param width Width of the page
    /// @param height Height of the page
    /// @param info Optional information to be put into the PDF header
    /// @return PDF document object, or NULL on failure
    pub fn pdf_create(width: c_float, height: c_float, info: *const PDFInfo) -> *mut PDFDoc;

    /// Add a new page to the given pdf
    /// @param pdf PDF document to append page to
    /// @return new page object
    pub fn pdf_append_page(pdf: *mut PDFDoc) -> *mut PDFObject;

    /// Add JPEG data as an image to the document
    /// @param pdf PDF document to add JPEG to
    /// @param page Page to add JPEG to (NULL => most recently added page)
    /// @param x X offset to put JPEG at
    /// @param y Y offset to put JPEG at
    /// @param display_width Displayed width of image
    /// @param display_height Displayed height of image
    /// @param jpeg_data JPEG data to add
    /// @param len Length of JPEG data
    /// @return < 0 on failure, >= 0 on success
    pub fn pdf_add_jpeg_data(
        pdf_doc: *mut PDFDoc,
        page: *mut PDFObject,
        x: c_float,
        y: c_float,
        display_width: c_float,
        display_height: c_float,
        jpeg_data: *const c_uchar,
        len: size_t,
    ) -> c_int;

    /// Save the given pdf document to the supplied filename.
    /// @param pdf PDF document to save
    /// @param filename Name of the file to store the PDF into (NULL for stdout)
    /// @return < 0 on failure, >= 0 on success
    pub fn pdf_save(pdf: *mut PDFDoc, filename: *const c_char) -> c_int;

    /// Destroy the pdf object, and all of its associated memory
    /// @param pdf PDF document to clean up
    pub fn pdf_destroy(pdf: *mut PDFDoc);
}
