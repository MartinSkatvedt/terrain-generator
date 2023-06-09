use bezier_rs::TValueType;
use imgui::{ImString, Ui};

use super::curve::Curve;

pub struct CurveEditor {
    name: ImString,
}

impl CurveEditor {
    pub fn new(name: &str) -> Self {
        Self {
            name: ImString::new(name),
        }
    }

    pub unsafe fn render(&self, ui: &Ui, curve: &mut Curve) {
        ui.text(&self.name);

        let draw_list = ui.get_window_draw_list();

        let o: [f32; 2] = ui.cursor_screen_pos();
        let ws = ui.content_region_avail();

        let margin_top: f32 = 5.0;
        let margin_bottom: f32 = 5.0;
        let height: f32 = 100.0;

        let canvas_top_left = [o[0], o[1] + margin_top];
        let canvas_bottom_right = [ws[0] + o[0], height + o[1] + 5.0];

        //draw canvas
        draw_list
            .add_rect(canvas_top_left, canvas_bottom_right, [1.0, 1.0, 1.0, 1.0])
            .build();

        //draw gridlines inside canvas
        let gridline_spacing = height / 4.0;
        let gridline_color = [0.5, 0.5, 0.5, 1.0];

        let mut x = canvas_top_left[0] + gridline_spacing;
        while x < canvas_bottom_right[0] {
            draw_list
                .add_line(
                    [x, canvas_top_left[1]],
                    [x, canvas_bottom_right[1]],
                    gridline_color,
                )
                .thickness(0.3)
                .build();
            x += gridline_spacing;
        }

        let mut y = canvas_top_left[1] + gridline_spacing;

        while y < canvas_bottom_right[1] {
            draw_list
                .add_line(
                    [canvas_top_left[0], y],
                    [canvas_bottom_right[0], y],
                    gridline_color,
                )
                .thickness(0.3)
                .build();
            y += gridline_spacing;
        }

        let detail: usize = 20;

        let points = curve
            .curve
            .compute_lookup_table(Some(detail), Some(TValueType::Parametric));

        let canvas_width = canvas_bottom_right[0] - canvas_top_left[0];

        for point in points {
            let x = (canvas_width as f64 * point.x) + canvas_top_left[0] as f64;
            let y = canvas_bottom_right[1] as f64 - (height as f64 * point.y);

            draw_list
                .add_circle([x as f32, y as f32], 2.0, [1.0, 1.0, 0.0, 1.0])
                .build();
        }

        //move cursor for next widget
        ui.set_cursor_screen_pos([o[0] + 5.0, canvas_bottom_right[1] + margin_bottom]);
    }
}
