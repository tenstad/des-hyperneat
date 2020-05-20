use crate::cppn::genome::Genome as CppnGenome;
use std::{fs::File, io::prelude::Write, path::Path};

pub fn genome_to_dot<P: AsRef<Path>>(fname: P, genome: &CppnGenome) -> std::io::Result<()> {
    let mut fname = String::from(fname.as_ref().to_str().unwrap());
    if !fname.ends_with(".dot") {
        fname.push_str(".dot");
    }

    let mut file = File::create(fname)?;
    file.write_all(b"digraph g {\n")?;

    for link in genome.neat.links.values() {
        let s = format!(
            "    {} -> {} [ label = \"{:.2}\" ];\n",
            link.from, link.to, link.weight
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.neat.inputs.values() {
        let s = format!(
            "    {} [ label = \"{}\", shape=box, style=filled, color=\".0 .0 .7\"]\n",
            node.neat.node_ref, node.neat.node_ref
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.neat.hidden_nodes.values() {
        let s = format!(
            "    {} [ label = \"{} {:.2} {}\"]\n",
            node.neat.node_ref, node.neat.node_ref, node.bias, node.activation
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.neat.outputs.values() {
        let s = format!(
            "    {} [ label = \"{} {:.2} {}\", shape=box, style=filled, color=\".0 .0 .85\"]\n",
            node.neat.node_ref, node.neat.node_ref, node.bias, node.activation
        );
        file.write_all(s.as_bytes())?;
    }

    file.write_all(b"}\n")?;

    return Ok(());
}
