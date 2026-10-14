//! NEXOPDF-owned Windows print preparation: CPU-rendered sheets, independent of
//! Windows.Data.Pdf, display DPI and graphics drivers. The temporary job is RAII-owned.

use std::sync::Arc;

use image::ImageEncoder;
use nexopdf_render::{PageRenderer, RenderConfig, RenderRequest, Tile};

use super::Error;

const MAX_JOB_BYTES: u64 = 1 << 30;
const MAX_SHEETS: usize = 1000;
const MAX_PIXELS: u64 = 64_000_000;

fn raster_plan(width: f32, height: f32) -> Result<(f32, u32, u32), Error> {
    // Match the physical paper limit of the Windows driver bridge (128 inches).
    if !width.is_finite() || !height.is_finite() || !(1.0..=9216.0).contains(&width) || !(1.0..=9216.0).contains(&height) {
        return Err(failed("sheet dimensions must be between 1 point and 128 inches"));
    }
    let mut scale = (600.0_f64 / 72.0)
        .min((MAX_PIXELS as f64 / (f64::from(width) * f64::from(height))).sqrt())
        .min(32768.0 / f64::from(width.max(height))) as f32;
    // Account for integer pixel rounding at the allocation boundary.
    loop {
        let w = nexopdf_render::device_pixels(width, scale);
        let h = nexopdf_render::device_pixels(height, scale);
        if w <= 32768 && h <= 32768 && u64::from(w) * u64::from(h) <= MAX_PIXELS {
            return Ok((scale, w, h));
        }
        scale *= 0.9999;
    }
}

fn render_sheet(renderer: &mut PageRenderer, page: usize, scale: f32, width: u32, height: u32) -> Result<Vec<u8>, Error> {
    let len = usize::try_from(u64::from(width) * u64::from(height) * 4).map_err(failed)?;
    let mut pixels = Vec::new();
    pixels.try_reserve_exact(len).map_err(failed)?;
    pixels.resize(len, 255);
    // Tiles preserve print resolution on sheets larger than the screen renderer's
    // 8192-pixel whole-page limit. Only one small tile is live beside the final image.
    for y in (0..height).step_by(2048) {
        for x in (0..width).step_by(2048) {
            let tile = Tile { x, y, w: 2048.min(width - x), h: 2048.min(height - y) };
            let raster = renderer.render(RenderRequest { page, scale, tile: Some(tile), tag: nexopdf_render::PRINT_TAG, ..Default::default() });
            if let Some(error) = raster.error {
                return Err(failed(format!("sheet {}: {error}", page + 1)));
            }
            if !raster.warnings.is_empty() || (raster.width, raster.height) != (tile.w, tile.h) {
                return Err(failed(format!("sheet {} could not be rendered completely", page + 1)));
            }
            for row in 0..tile.h {
                // raster_plan bounds every dimension and the allocation to 64M pixels.
                let source = (row * tile.w * 4) as usize;
                let target = (((y + row) * width + x) * 4) as usize;
                let count = (tile.w * 4) as usize;
                let src = raster.rgba.get(source..source + count).ok_or_else(|| failed("incomplete rendered tile"))?;
                let dst = pixels.get_mut(target..target + count).ok_or_else(|| failed("tile exceeds sheet bounds"))?;
                dst.copy_from_slice(src);
            }
        }
    }
    Ok(pixels)
}

fn failed(e: impl std::fmt::Display) -> Error {
    Error::Spool(format!("the print job could not be prepared: {e}"))
}

pub(super) fn prepare(pdf: &[u8]) -> Result<tempfile::TempDir, Error> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prepare_inner(pdf)))
        .map_err(|_| failed("the renderer failed; the document has not been changed"))?
}

fn prepare_inner(pdf: &[u8]) -> Result<tempfile::TempDir, Error> {
    if pdf.len() as u64 > MAX_JOB_BYTES {
        return Err(failed("print-ready PDF exceeds 1 GiB; print fewer sheets at a time"));
    }
    let mut owned = Vec::new();
    owned.try_reserve_exact(pdf.len()).map_err(failed)?;
    owned.extend_from_slice(pdf);
    let bytes = Arc::new(owned);
    let info = nexopdf_render::inspect(bytes.clone(), None).map_err(failed)?;
    if info.pages.is_empty() || info.pages.len() > MAX_SHEETS {
        return Err(failed("print between 1 and 1000 sheets per job; split larger jobs"));
    }
    let directory = tempfile::Builder::new().prefix("nexopdf-print-").tempdir().map_err(failed)?;
    let mut renderer = PageRenderer::new(bytes, RenderConfig { reject_oversize: true, ..Default::default() });
    let mut sheets = Vec::with_capacity(info.pages.len());
    let mut job_bytes = 0_u64;
    for (page, geometry) in info.pages.iter().enumerate() {
        let (width, height) = (geometry.width, geometry.height);
        let (scale, pixel_width, pixel_height) = raster_plan(width, height)?;
        let pixels = render_sheet(&mut renderer, page, scale, pixel_width, pixel_height)?;
        let name = format!("sheet-{page}.png");
        let path = directory.path().join(&name);
        let mut file = std::fs::File::create(&path).map_err(failed)?;
        image::codecs::png::PngEncoder::new(&mut file)
            .write_image(&pixels, pixel_width, pixel_height, image::ExtendedColorType::Rgba8)
            .map_err(failed)?;
        job_bytes = job_bytes.checked_add(file.metadata().map_err(failed)?.len()).ok_or_else(|| failed("job size overflow"))?;
        if job_bytes > MAX_JOB_BYTES {
            return Err(failed("rendered job exceeds 1 GiB; print fewer sheets at a time"));
        }
        sheets.push(serde_json::json!({"File":name, "WidthDip":f64::from(width)*96.0/72.0,
            "HeightDip":f64::from(height)*96.0/72.0, "PixelWidth":pixel_width, "PixelHeight":pixel_height}));
    }
    let manifest = std::fs::File::create(directory.path().join("job.json")).map_err(failed)?;
    serde_json::to_writer(manifest, &serde_json::json!({"Sheets":sheets})).map_err(failed)?;
    Ok(directory)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_document_is_rejected_before_spooling() {
        assert!(prepare(b"not a PDF").is_err());
    }

    #[test]
    fn print_resolution_preserves_detail_and_bounds_allocations() {
        let (_, w, h) = raster_plan(612.0, 792.0).unwrap();
        assert_eq!((w, h), (5100, 6600));
        for (w, h) in [(9216.0, 9216.0), (9216.0, 72.0), (1234.5, 9216.0)] {
            let (_, w, h) = raster_plan(w, h).unwrap();
            assert!(u64::from(w) * u64::from(h) <= MAX_PIXELS);
            assert!(w <= 32768 && h <= 32768);
        }
        for bad in [f32::NAN, f32::INFINITY, -1.0, 0.0, 9217.0] {
            assert!(raster_plan(bad, 72.0).is_err());
            assert!(raster_plan(72.0, bad).is_err());
        }
    }
}
