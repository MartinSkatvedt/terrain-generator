extern crate bezier_rs as bezier;

use bezier::{Bezier, TValue};

#[derive(Clone, PartialEq)]
pub struct Curve {
    pub curve: Bezier,
}

impl Curve {
    pub fn linear() -> Self {
        Curve {
            curve: Bezier::from_linear_coordinates(0.0, 0.0, 1.0, 1.0),
        }
    }

    pub fn quadratic() -> Self {
        Curve {
            curve: Bezier::from_quadratic_coordinates(0.0, 0.0, 0.5, 1.0, 1.0, 0.0),
        }
    }

    pub fn cubic() -> Self {
        Curve {
            curve: Bezier::from_cubic_coordinates(0.0, 0.0, 0.25, 0.1, 0.75, 0.1, 1.0, 1.0),
        }
    }

    pub fn evaluate(&self, t: f64) -> f64 {
        let t_val = TValue::Parametric(t);
        let point = self.curve.evaluate(t_val);

        point.x
    }
}
