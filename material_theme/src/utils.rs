use iced_widget::core::Color;

pub fn elevation(elevation_level: u8) -> f32 {
    (match elevation_level {
        0 => 0.0,
        1 => 1.0,
        2 => 3.0,
        3 => 6.0,
        4 => 8.0,
        _ => 12.0,
    } as f32)
}

pub fn mix(color1: Color, color2: Color, p2: f32) -> Color {
    if p2 <= 0.0 {
        return color1;
    } else if p2 >= 1.0 {
        return color2;
    }

    let p1 = 1.0 - p2;

    if color1.a != 1.0 || color2.a != 1.0 {
        let a = color1.a * p1 + color2.a * p2;
        if a > 0.0 {
            let c1 = color1.into_linear().map(|c| c * color1.a * p1);
            let c2 = color2.into_linear().map(|c| c * color2.a * p2);

            let [r, g, b] =
                [c1[0] + c2[0], c1[1] + c2[1], c1[2] + c2[2]].map(|u| u / a);

            return Color::from_linear_rgba(r, g, b, a);
        }
    }

    let c1 = color1.into_linear().map(|c| c * p1);
    let c2 = color2.into_linear().map(|c| c * p2);

    Color::from_linear_rgba(
        c1[0] + c2[0],
        c1[1] + c2[1],
        c1[2] + c2[2],
        c1[3] + c2[3],
    )
}

#[cfg(test)]
mod tests {
    use super::{Color, mix};

    #[test]
    fn mixing_works() {
        let base = Color::from_rgba(1.0, 0.0, 0.0, 0.7);
        let overlay = Color::from_rgba(0.0, 1.0, 0.0, 0.2);

        assert_eq!(
            mix(base, overlay, 0.75).into_rgba8(),
            Color::from_linear_rgba(0.53846, 0.46154, 0.0, 0.325).into_rgba8()
        );
    }
}
