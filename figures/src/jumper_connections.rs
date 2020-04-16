use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(1.0);

    let mut substrate = fig.substrate_builder(&|s| s.size(4.0).cells(2).axis_arrow_offset(0.15));
    let s = substrate(&|s| s);
    drop(substrate);

    let mut node = fig.node_builder(&|n| n);
    let n1 = node(&|n| n.x(2.5).y(0.6));
    let n2 = node(&|n| n.x(1.0).y(1.0));
    let n3 = node(&|n| n.x(3.8).y(1.8));
    let n4 = node(&|n| n.x(2.3).y(2.1));
    let n5 = node(&|n| n.x(0.4).y(2.8));
    let n6 = node(&|n| n.x(2.4).y(3.6));
    drop(node);

    let mut node = fig.node_builder(&|n| n.visible(false));
    let h1 = node(&|n| n.x(0.2).y(-0.5));
    let h2 = node(&|n| n.x(2.4).y(-0.5));
    let h3 = node(&|n| n.x(3.6).y(-0.5));
    let h4 = node(&|n| n.x(1.1).y(4.5));
    let h5 = node(&|n| n.x(2.7).y(4.5));
    let h6 = node(&|n| n.x(3.9).y(4.5));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e.color("gray!35"));
    edge(&n2, &n1, &|e| e);
    edge(&n2, &n5, &|e| e);
    edge(&n5, &n4, &|e| e);
    edge(&n1, &n4, &|e| e);
    edge(&n1, &n3, &|e| e);
    edge(&n3, &n6, &|e| e);
    drop(edge);

    let mut edge = fig.edge_builder(&|e| e.width(0.3).color("black"));
    edge(&h1, &n2, &|e| e);
    edge(&h2, &n1, &|e| e);
    edge(&h3, &n1, &|e| e);
    edge(&n5, &h4, &|e| e.style("--"));
    edge(&n6, &h5, &|e| e.style("--"));
    edge(&n4, &h6, &|e| e.style("--"));
    edge(&n4, &h4, &|e| e.style("--"));
    drop(edge);

    fig
}
