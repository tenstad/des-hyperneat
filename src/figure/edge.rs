use crate::figure;
use crate::figure::node;
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
    #[builder(default = "\"gray\"")]
    color: &'static str,
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

impl figure::Component for Edge {
    fn to_str(&self) -> String {
        format!(
"   \\draw[{style}, draw={color}, line width={width}mm, draw opacity={opacity}] ({source}.{angle1}) -- ({target}.{angle2});",
            source = self.source,
            target = self.target,
            angle1 = self.angle,
            angle2 = self.angle + 180.0,
            width = self.width,
            style = self.style,
            color = self.color,
            opacity = self.opacity,
        )
    }
}
