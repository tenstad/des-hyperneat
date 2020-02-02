use crate::neat::genome;
use std::fs::File;
use std::io::prelude::*;

pub fn genome_to_dot(fname: String, genome: &genome::Genome) -> std::io::Result<()> {
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
        let s = format!("    i{} [ label = \"i{}\", shape=box, style=filled, color=\".0 .0 .7\"]\n", node.id, node.id);
        file.write_all(s.as_bytes())?;
    }

    for node in genome.hidden_nodes.values() {
        let s = format!("    h{} [ label = \"h{} {:.2} {}\"]\n", node.id, node.id, node.bias, node.activation);
        file.write_all(s.as_bytes())?;
    }

    for node in genome.outputs.values() {
        let s = format!("    o{} [ label = \"o{} {:.2} {}\", shape=box, style=filled, color=\".0 .0 .85\"]\n", node.id, node.id, node.bias, node.activation);
        file.write_all(s.as_bytes())?;
    }

    file.write_all(b"}\n")?;

    return Ok(());
}
