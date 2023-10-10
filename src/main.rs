use std::path::Path;

use color_eyre::eyre::Result;
use lcs_image_diff::compare;
use pdfium_render::page::PdfPageRenderRotation;
use pdfium_render::pdfium::Pdfium;
use pdfium_render::render_config::PdfRenderConfig;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut before = pdf_to_image("./test/before.pdf")?;
    let mut after = pdf_to_image("./test/after.pdf")?;

    let diff = compare(&mut before, &mut after, 0.4)?;

    diff.save(Path::new("./test/diff.png"))?;
    before.save(Path::new("./test/before.png"))?;
    after.save(Path::new("./test/after.png"))?;

    Ok(())
}

fn pdf_to_image(path: &str) -> Result<image::DynamicImage> {
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let document = pdfium.load_pdf_from_file(path, None)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(1000)
        .set_maximum_height(3000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    let page = document.pages().get(0).expect("No pages in document");
    let image = page.render_with_config(&render_config)?.as_image();
    Ok(image)
}
