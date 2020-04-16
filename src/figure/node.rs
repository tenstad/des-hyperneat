use crate::figure;
use crate::figure::edge;

#[derive(Builder, Clone)]
pub struct Node {
    #[builder(default = "uuid::Uuid::new_v4()")]
    pub id: uuid::Uuid,
    #[builder(default = "0.0")]
    pub x: f64,
    #[builder(default = "0.0")]
    pub y: f64,
    #[builder(default = "2.0")]
    size: f64,
    #[builder(default = "\"\"")]
    text: &'static str,
    #[builder(default = "\"black\"")]
    outline: &'static str,
    #[builder(default = "\"white\"")]
    fill: &'static str,
    #[builder(default = "0.8")]
    edge_offset: f64,
}

impl Node {
    pub fn connect(&self, other: &Self) -> edge::EdgeBuilder {
        edge::EdgeBuilder::new(&self, &other)
    }
}

impl figure::Component for Node {
    fn to_str(&self) -> String {
        format!(
"   \\node[shape=circle, minimum size={offset}mm] at ({x}, {y}) ({id}) {{}};
    \\node[draw, shape=circle, draw={outline}, fill={fill}, inner sep=0pt, minimum size={size}mm] at ({x}, {y}) {{{text}}};",
            id = self.id,
            x = self.x,
            y = self.y,
            size = self.size,
            text = self.text,
            outline = self.outline,
            fill = self.fill,
            offset = self.size + self.edge_offset,
        )
    }

    fn priority(&self) -> usize {
        20
    }
}
