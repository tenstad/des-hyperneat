use crate::Component;

#[derive(Builder, Clone)]
pub struct Substrate {
    #[builder(default = "0.0")]
    pub x: f64,
    #[builder(default = "0.0")]
    pub y: f64,
    #[builder(default = "1.0")]
    size: f64,
    #[builder(default = "1")]
    cells: usize,
    #[builder(default = "0.1")]
    axis_arrow_offset: f64,
    #[builder(default = "true")]
    visible_axis: bool,
    #[builder(default = "\"gray!40\"")]
    color: &'static str,
}

impl Component for Substrate {
    fn to_str(&self) -> String {
        if self.visible_axis {
            format!(
"   \\draw[style=help lines, draw={color}, step={step}, xshift={x}, yshift={y}] (0, 0) grid ({size}, {size});
    \\draw[->, draw={color}, xshift={x}, yshift={y}] ({size}, 0) -- ({arrow_length}, 0) node[above, xshift=-7]{{}};
    \\draw[->, draw={color}, xshift={x}, yshift={y}] (0, {size}) -- (0, {arrow_length}) node[below, xshift=5]{{}};",
                x = self.x * 14.225,
                y = self.y * 14.225,
                size = self.size,
                step = self.size / self.cells as f64,
                arrow_length = self.size + self.axis_arrow_offset,
                color = self.color,
            )
        } else {
            format!(
"   \\draw[style=help lines, draw={color}, step={step}, xshift={x}, yshift={y}] (0, 0) grid ({size}, {size});",
                x = self.x * 14.225,
                y = self.y * 14.225,
                size = self.size,
                step = self.size / self.cells as f64,
                color = self.color,
            )
        }
    }

    fn priority(&self) -> usize {
        10
    }
}
