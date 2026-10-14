//! NEXOPDF home materials. This is alpha composition over an internal backdrop, not blur.

use crate::theme::Tokens;
use egui::{Color32, Rect};

/// Bounded opacity; labels, icons, documents and native windows are never faded.
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub struct GlassOpacity(u8);

impl Default for GlassOpacity {
    fn default() -> Self {
        Self(90)
    }
}

impl TryFrom<u8> for GlassOpacity {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (70..=100).contains(&value) { Ok(Self(value)) } else { Err("glass_opacity must be an integer from 70 to 100") }
    }
}

impl From<GlassOpacity> for u8 {
    fn from(value: GlassOpacity) -> Self {
        value.0
    }
}

impl GlassOpacity {
    pub fn percent(self) -> u8 {
        self.0
    }

    pub fn fill(self, color: Color32) -> Color32 {
        let alpha = (u16::from(self.0) * 255 / 100) as u8;
        Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
    }
}

/// Four vertices only: no animation, texture, desktop capture or additional render pass.
pub fn backdrop(ui: &egui::Ui, rect: Rect, t: &Tokens) {
    let mut mesh = egui::Mesh::default();
    mesh.colored_vertex(rect.left_top(), t.pasteboard.lerp_to_gamma(t.border, 0.38));
    mesh.colored_vertex(rect.right_top(), t.chrome);
    mesh.colored_vertex(rect.right_bottom(), t.pasteboard);
    mesh.colored_vertex(rect.left_bottom(), t.titlebar);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    ui.painter().add(egui::Shape::mesh(mesh));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solid_mode_preserves_the_surface_and_translucency_is_bounded() {
        let base = Color32::from_rgb(37, 43, 52);
        assert_eq!(GlassOpacity::try_from(100).unwrap().fill(base), base);
        assert_eq!(GlassOpacity::try_from(70).unwrap().fill(base).a(), 178);
        assert_eq!(GlassOpacity::default().fill(base).a(), 229);
        for value in [0, 69, 101, 255] {
            assert!(GlassOpacity::try_from(value).is_err());
        }
    }
}
