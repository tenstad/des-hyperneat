use crate::cppn::genome;
use std::{fs::File, io::prelude::Write};

pub fn genome_to_dot(fname: String, genome: &genome::Genome) -> std::io::Result<()> {
    let mut file = File::create(fname)?;
    file.write_all(b"digraph g {\n")?;

    for link in genome.core.links.values() {
        let s = if link.core.enabled {
            format!(
                "    {} -> {} [ label = \"{:.2}\" ];\n",
                link.core.from, link.core.to, link.core.weight
            )
        } else {
            //format!("")
            format!(
                "    {} -> {} [ label = \"{:.2}\" style=dotted ];\n",
                link.core.from, link.core.to, link.core.weight
            )
        };
        file.write_all(s.as_bytes())?;
    }

    for node in genome.core.inputs.values() {
        let s = format!(
            "    {} [ label = \"{}\", shape=box, style=filled, color=\".0 .0 .7\"]\n",
            node.core.node_ref, node.core.node_ref
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.core.hidden_nodes.values() {
        let s = format!(
            "    {} [ label = \"{} {:.2} {}\"]\n",
            node.core.node_ref, node.core.node_ref, node.bias, node.activation
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.core.outputs.values() {
        let s = format!(
            "    {} [ label = \"{} {:.2} {}\", shape=box, style=filled, color=\".0 .0 .85\"]\n",
            node.core.node_ref, node.core.node_ref, node.bias, node.activation
        );
        file.write_all(s.as_bytes())?;
    }

    file.write_all(b"}\n")?;

    return Ok(());
}
