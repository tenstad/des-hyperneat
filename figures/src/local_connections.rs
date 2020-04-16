use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(1.0);

    let mut substrate = fig.substrate_builder(&|s| s.size(4.0).cells(2).axis_arrow_offset(0.15));
    let s = substrate(&|s| s);
    drop(substrate);

    let mut node = fig.node_builder(&|n| n);
    node(&|n| n.x(0.2).y(-0.5).visible(false));

    let n1 = node(&|n| n.x(2.5).y(0.6));
    let n2 = node(&|n| n.x(1.0).y(1.0));
    let n3 = node(&|n| n.x(3.8).y(1.8));
    let n4 = node(&|n| n.x(2.3).y(2.1));
    let n5 = node(&|n| n.x(0.4).y(2.8));
    let n6 = node(&|n| n.x(2.4).y(3.6));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e.width(0.3).color("black"));
    edge(&n2, &n1, &|e| e);
    edge(&n2, &n5, &|e| e);
    edge(&n5, &n4, &|e| e);
    edge(&n1, &n4, &|e| e);
    edge(&n1, &n3, &|e| e);
    edge(&n3, &n6, &|e| e);
    drop(edge);

    fig
}