use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(1.0);

    let mut substrate = fig.substrate_builder(&|s| s.size(1.6).cells(1).axis_arrow_offset(0.15));
    let w_s = substrate(&|s| s.x(-2.0 * 1.2 - 2.0 + 0.4).y(-2.0 + 0.4));
    let e_s = substrate(&|s| s.x(2.0 * 1.2 - 2.0 + 0.4).y(-2.0 + 0.4));
    drop(substrate);

    let mut node = fig.node_builder(&|n| n.fill("teal"));
    node(&|n| n.x(-1.2 - 0.5));
    node(&|n| n.x(-1.2));
    node(&|n| n.x(-1.2 + 0.5));
    node(&|n| n.x(1.2).fill("blue"));
    drop(node);

    let mut label = fig.label_builder(&|l| l.x(2.85));
    label(&|l| l.y(0.7).text("Output"));
    label(&|l| l.y(0.25).text("Input"));
    drop(label);

    let mut node = fig.node_builder(&|n| n.x(2.7));
    node(&|n| n.y(0.7).fill("blue"));
    node(&|n| n.y(0.25).fill("teal"));
    drop(node);

    fig
}
