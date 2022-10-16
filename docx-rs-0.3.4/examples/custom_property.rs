use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/custom_property.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .custom_property("hello", "world")
        .build()
        .pack(file)?;
    Ok(())
}
