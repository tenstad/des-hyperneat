use crate::cppn::genome;
use std::{fs::File, io::prelude::Write};

pub fn genome_to_dot(fname: String, genome: &genome::Genome) -> std::io::Result<()> {
    let mut file = File::create(fname)?;
    file.write_all(b"digraph g {\n")?;

    for link in genome.neat_genome.links.values() {
        let s = if link.enabled {
            format!(
                "    {} -> {} [ label = \"{:.2}\" ];\n",
                link.from, link.to, link.weight
            )
        } else {
            //format!("")
            format!(
                "    {} -> {} [ label = \"{:.2}\" style=dotted ];\n",
                link.from, link.to, link.weight
            )
        };
        file.write_all(s.as_bytes())?;
    }

    for node in genome.neat_genome.inputs.values() {
        let s = format!(
            "    {} [ label = \"{}\", shape=box, style=filled, color=\".0 .0 .7\"]\n",
            node.neat_node.node_ref, node.neat_node.node_ref
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.neat_genome.hidden_nodes.values() {
        let s = format!(
            "    {} [ label = \"{} {:.2} {}\"]\n",
            node.neat_node.node_ref, node.neat_node.node_ref, node.bias, node.activation
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.neat_genome.outputs.values() {
        let s = format!(
            "    {} [ label = \"{} {:.2} {}\", shape=box, style=filled, color=\".0 .0 .85\"]\n",
            node.neat_node.node_ref, node.neat_node.node_ref, node.bias, node.activation
        );
        file.write_all(s.as_bytes())?;
    }

    file.write_all(b"}\n")?;

    return Ok(());
}
