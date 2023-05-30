use imgui::{CollapsingHeader, Ui};

use crate::curve_editor::{curve::Curve, curve_widget::CurveEditor};

#[derive(Clone, PartialEq)]
pub struct MeshSettings {
    pub name: String,
    pub strength: f32,
    pub curve: Curve,
}

impl MeshSettings {
    pub fn new(name: String, strength: f32, curve: Curve) -> Self {
        Self {
            name,
            strength,
            curve,
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

            CurveEditor::new("Terrain Curve Editor").render(ui, &mut self.curve);
        }
    }
}
