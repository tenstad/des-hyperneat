use crate::gen_text;
use crate::Component;

#[derive(Builder, Clone)]
pub struct Label {
    #[builder(default = "0.0")]
    pub x: f64,
    #[builder(default = "0.0")]
    pub y: f64,
    #[builder(default = "\"\"")]
    text: &'static str,
    #[builder(default = "1.0")]
    opacity: f64,
    #[builder(default = "\"black\"")]
    text_color: &'static str,
    #[builder(default = "\"scriptsize\"")]
    text_size: &'static str,
    #[builder(default = "8")]
    line_height: usize,
    #[builder(default = "\"left\"")]
    align: &'static str,
}

impl Component for Label {
    fn to_str(&self) -> String {
        format!(
        "   \\node[anchor=west, align={align}, text={text_color}, draw opacity={opacity}, execute at begin node=\\setlength{{\\baselineskip}}{{{line_height}pt}}] at ({x}, {y}) {{{text}}};",
            x = self.x,
            y = self.y,
            opacity = self.opacity,
            text_color = self.text_color,
            text = gen_text(self.text, self.text_size),
            line_height = self.line_height,
            align = self.align,
        )
    }

    fn priority(&self) -> usize {
        50
    }
}
