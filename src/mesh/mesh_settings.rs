use imgui::{CollapsingHeader, Ui};

use crate::curve_editor::{curve::Curve, curve_widget::CurveEditor};

#[derive(Clone, PartialEq)]
pub struct MeshSettings {
    pub name: String,
    pub strength: f32,
    pub curve: Curve,
    pub level_of_detail: i32,
}

impl MeshSettings {
    pub fn new(name: String, strength: f32, curve: Curve, level_of_detail: i32) -> Self {
        Self {
            name,
            strength,
            curve,
            level_of_detail: level_of_detail.clamp(0, 6),
        }
    }

    pub unsafe fn render(&mut self, ui: &Ui) {
        if CollapsingHeader::new(&self.name).build(ui) {
            ui.slider(
                format!("Strength##{}", self.name),
                0.0,
                25.0,
                &mut self.strength,
            );
            ui.slider("Detail", 0, 6, &mut self.level_of_detail);

            CurveEditor::new("Terrain Curve Editor").render(ui, &mut self.curve);
        }
    }
}
