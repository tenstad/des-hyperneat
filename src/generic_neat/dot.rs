use crate::generic_neat::genome;
use crate::generic_neat::link;
use crate::generic_neat::node;
use std::fs::File;
use std::io::prelude::*;

pub fn genome_to_dot<I: node::Custom, H: node::Custom, O: node::Custom, L: link::Custom>(
    fname: String,
    genome: &genome::Genome<I, H, O, L>,
) -> std::io::Result<()> {
    let mut file = File::create(fname)?;
    file.write_all(b"digraph g {\n")?;

    for link in genome.links.values() {
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

    for node in genome.inputs.values() {
        let s = format!(
            "    {} [ label = \"{}\", shape=box, style=filled, color=\".0 .0 .7\"]\n",
            node.node_ref, node.node_ref
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.hidden_nodes.values() {
        let s = format!(
            "    {} [ label = \"{} {:.2} {}\"]\n",
            node.node_ref, node.node_ref, node.bias, node.activation
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.outputs.values() {
        let s = format!(
            "    {} [ label = \"{} {:.2} {}\", shape=box, style=filled, color=\".0 .0 .85\"]\n",
            node.node_ref, node.node_ref, node.bias, node.activation
        );
        file.write_all(s.as_bytes())?;
    }

    file.write_all(b"}\n")?;

    return Ok(());
}
