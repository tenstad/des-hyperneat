#[macro_use]
extern crate derive_builder;

pub mod edge;
pub mod node;
pub mod substrate;

use std::fs;
use std::io::Write;
use std::path::Path;

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

    pub fn node_builder<'a>(
        &'a mut self,
        default_configure: &'a dyn Fn(&mut node::NodeBuilder) -> &mut node::NodeBuilder,
    ) -> impl FnMut(&'a dyn Fn(&mut node::NodeBuilder) -> &mut node::NodeBuilder) -> node::Node
    {
        move |configure| {
            self.add(
                configure(default_configure(&mut node::NodeBuilder::default()))
                    .build()
                    .unwrap(),
            )
        }
    }

    pub fn edge_builder<'a>(
        &'a mut self,
        default_configure: &'a dyn Fn(&mut edge::EdgeBuilder) -> &mut edge::EdgeBuilder,
    ) -> impl FnMut(
        &node::Node,
        &node::Node,
        &'a dyn Fn(&mut edge::EdgeBuilder) -> &mut edge::EdgeBuilder,
    ) -> edge::Edge {
        move |source, target, configure| {
            self.add(
                configure(default_configure(&mut source.connect(target)))
                    .build()
                    .unwrap(),
            )
        }
    }

    pub fn substrate_builder<'a>(
        &'a mut self,
        default_configure: &'a dyn Fn(
            &mut substrate::SubstrateBuilder,
        ) -> &mut substrate::SubstrateBuilder,
    ) -> impl FnMut(
        &'a dyn Fn(&mut substrate::SubstrateBuilder) -> &mut substrate::SubstrateBuilder,
    ) -> substrate::Substrate {
        move |configure| {
            self.add(
                configure(default_configure(
                    &mut substrate::SubstrateBuilder::default(),
                ))
                .build()
                .unwrap(),
            )
        }
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) {
        let mut file = fs::File::create(path).expect("unable to create file");
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
