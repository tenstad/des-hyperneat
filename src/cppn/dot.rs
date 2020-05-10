use crate::cppn::genome::Genome as CppnGenome;
use std::{fs::File, io::prelude::Write, path::Path};

pub fn genome_to_dot<P: AsRef<Path>>(fname: P, genome: &CppnGenome) -> std::io::Result<()> {
    let mut fname = String::from(fname.as_ref().to_str().unwrap());
    if !fname.ends_with(".dot") {
        fname.push_str(".dot");
    }

    let mut file = File::create(fname)?;
    file.write_all(b"digraph g {\n")?;

    for link in genome.core.links.values() {
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
