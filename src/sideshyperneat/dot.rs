use crate::cppn::dot::genome_to_dot as cppn_genome_to_dot;
use crate::sideshyperneat::genome::Genome;
use std::{fs::File, io::prelude::Write, path::Path};

pub fn genome_to_dot<P: AsRef<Path>>(fname: P, genome: &Genome) -> std::io::Result<()> {
    let fname = String::from(fname.as_ref().to_str().unwrap());
    let mut cppn_fname = fname.clone();
    let mut topo_fname = fname.clone();
    cppn_fname.push_str("_cppn");
    topo_fname.push_str("_topology.dot");
    cppn_genome_to_dot(cppn_fname, &genome.cppn).ok();

    let mut file = File::create(topo_fname)?;
    file.write_all(b"digraph g {\n")?;

    for link in genome.topology.links.values() {
        let s = if link.neat.enabled {
            format!(
                "    {} -> {} [ label = \"{:.2}\" ];\n",
                link.neat.from, link.neat.to, link.neat.weight
            )
        } else {
            //format!("")
            format!(
                "    {} -> {} [ label = \"{:.2} {}\" style=dotted ];\n",
                link.neat.from, link.neat.to, link.neat.weight, link.cppn_output_id
            )
        };
        file.write_all(s.as_bytes())?;
    }

    for node in genome.topology.inputs.values() {
        let s = format!(
            "    {} [ label = \"{} {}\", shape=box, style=filled, color=\".0 .0 .7\"]\n",
            node.neat.node_ref, node.neat.node_ref, node.cppn_output_id
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.topology.hidden_nodes.values() {
        let s = format!(
            "    {} [ label = \"{} {}\"]\n",
            node.neat.node_ref, node.neat.node_ref, node.cppn_output_id
        );
        file.write_all(s.as_bytes())?;
    }

    for node in genome.topology.outputs.values() {
        let s = format!(
            "    {} [ label = \"{} {}\", shape=box, style=filled, color=\".0 .0 .85\"]\n",
            node.neat.node_ref, node.neat.node_ref, node.cppn_output_id
        );
        file.write_all(s.as_bytes())?;
    }

    file.write_all(b"}\n")?;

    return Ok(());
}
