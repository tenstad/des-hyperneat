pub mod edge;
pub mod node;
pub mod substrate;

use std::fs;
use std::io::Write;

pub trait Component {
    fn to_str(&self) -> String;
    fn priority(&self) -> usize {
        100
    }
}

pub trait ComponentBuilder {
    fn build<T: Component>(&self) -> T;
}

pub struct Figure {
    components: Vec<Box<dyn Component>>,
    scale: f64,
}

impl Figure {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            scale: 1.0,
        }
    }

    pub fn add<T: Component + Clone + 'static>(&mut self, component: T) -> T {
        self.components.push(Box::new(component.clone()));
        component
    }

    pub fn to_file(&mut self, fname: String) {
        let mut file = fs::File::create(fname).expect("unable to create file");
        file.write_all(self.to_str().as_bytes())
            .expect("unable to write file");
    }

    fn to_str(&mut self) -> String {
        self.components.sort_by_key(|c| c.priority());
        format!(
            "\\begin{{tikzpicture}}[scale={}]\n{}\n\\end{{tikzpicture}}\n",
            self.scale,
            self.components
                .iter()
                .map(|component| component.to_str())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
