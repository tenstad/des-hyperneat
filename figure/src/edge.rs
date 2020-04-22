use crate::gen_text;
use crate::node;
use std::f64;

#[derive(Builder, Clone)]
pub struct Edge {
    source: uuid::Uuid,
    target: uuid::Uuid,
    angle: f64,
    #[builder(default = "0.2")]
    width: f64,
    #[builder(default = "\"->\"")]
    style: &'static str,
    #[builder(default = "\"\"")]
    text: &'static str,
    #[builder(default = "0.5")]
    pos: f64,
    #[builder(default = "\"above\"")]
    text_pos: &'static str,
    #[builder(default = "0.0")]
    xshift: f64,
    #[builder(default = "0.0")]
    yshift: f64,
    #[builder(default = "\"gray\"")]
    color: &'static str,
    #[builder(default = "\"black\"")]
    text_color: &'static str,
    #[builder(default = "\"scriptsize\"")]
    text_size: &'static str,
    #[builder(default = "1.0")]
    opacity: f64,
}

impl EdgeBuilder {
    pub fn new(from: &node::Node, to: &node::Node) -> EdgeBuilder {
        EdgeBuilder::default()
            .source(from.id)
            .target(to.id)
            .angle(180.0 / f64::consts::PI * (to.y - from.y).atan2(to.x - from.x))
            .clone()
    }
}

impl crate::Component for Edge {
    fn to_str(&self) -> String {
        format!(
        "   \\draw[{style}, draw={color}, line width={width}mm, draw opacity={opacity}] ({source}.{angle1}) -- ({target}.{angle2}) node[pos={pos}, xshift={xshift}mm, yshift={yshift}mm, {text_pos}, text={text_color}] {{\\{text_size} {text}}};",
            source = self.source,
            target = self.target,
            angle1 = self.angle,
            angle2 = self.angle + 180.0,
            width = self.width,
            style = self.style,
            text = gen_text(self.text, self.text_size),
            pos = self.pos,
            text_pos = self.text_pos,
            xshift = self.xshift,
            yshift = self.yshift,
            color = self.color,
            text_color = self.text_color,
            text_size = self.text_size,
            opacity = self.opacity,
        )
    }
}
