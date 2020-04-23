use crate::edge;
use crate::gen_text;
use crate::Component;

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
    #[builder(default = "0.0")]
    width: f64,
    #[builder(default = "0.0")]
    height: f64,
    #[builder(default = "\"\"")]
    text: &'static str,
    #[builder(default = "\"circle\"")]
    shape: &'static str,
    #[builder(default = "\"black\"")]
    outline: &'static str,
    #[builder(default = "\"white\"")]
    fill: &'static str,
    #[builder(default = "1.0")]
    opacity: f64,
    #[builder(default = "\"black\"")]
    text_color: &'static str,
    #[builder(default = "\"scriptsize\"")]
    text_size: &'static str,
    #[builder(default = "0.8")]
    edge_offset: f64,
    #[builder(default = "0.0")]
    inner_sep: f64,
    #[builder(default = "true")]
    visible: bool,
}

impl Node {
    pub fn connect(&self, other: &Self) -> edge::EdgeBuilder {
        edge::EdgeBuilder::new(&self, &other)
    }
}

impl Component for Node {
    fn to_str(&self) -> String {
        if self.visible {
            format!(
"   \\node[shape={shape}, minimum size={offset}mm{w_offset}{h_offset}] at ({x}, {y}) ({id}) {{}};
    \\node[draw, shape={shape}, draw={outline}, fill={fill}, text={text_color}, inner sep={inner_sep}pt, minimum size={size}mm{width}{height}, draw opacity={opacity}] at ({x}, {y}) {{\\{text_size} {text}}};",
                id = self.id,
                x = self.x,
                y = self.y,
                size = self.size,
                width = if self.width > 0.0 { format!(", minimum width={}mm", self.width) } else { "".to_string() },
                height = if self.height > 0.0 { format!(", minimum height={}mm", self.height) } else { "".to_string() },
                text = gen_text(self.text, self.text_size),
                shape = self.shape,
                outline = self.outline,
                fill = self.fill,
                opacity = self.opacity,
                text_color = self.text_color,
                text_size = self.text_size,
                offset = self.size + self.edge_offset,
                w_offset = if self.width > 0.0 { format!(", minimum width={}mm", self.width + self.edge_offset) } else { "".to_string() },
                h_offset = if self.height > 0.0 { format!(", minimum height={}mm", self.height + self.edge_offset) } else { "".to_string() },
                inner_sep = self.inner_sep,
            )
        } else {
            format!(
                "   \\node[shape=circle, minimum size={offset}mm] at ({x}, {y}) ({id}) {{}};",
                id = self.id,
                x = self.x,
                y = self.y,
                offset = self.size + self.edge_offset,
            )
        }
    }

    fn priority(&self) -> usize {
        20
    }
}
