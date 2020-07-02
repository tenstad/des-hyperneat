use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(0.6);

    let mut substrate = fig.substrate_builder(&|s| s.size(2.0).cells(2).visible_axis(false));
    let s = substrate(&|s| s);
    let w = substrate(&|s| s.x(-2.5).y(5.0));
    let e = substrate(&|s| s.x(2.5).y(5.0));
    let n = substrate(&|s| s.y(10.0));
    drop(substrate);

    let mut node = fig.node_builder(&|n| n.size(1.5).edge_offset(0.0).fill("white").y(1.0));
    let i0 = node(&|n| n.x(1.0 / 3.0));
    let i1 = node(&|n| n.x(1.0));
    let i2 = node(&|n| n.x(5.0 / 3.0));
    let o = node(&|n| n.x(1.0).y(6.0));
    drop(node);

    let mut node = fig.node_builder(&|n| n.size(1.5).fill("green").edge_offset(0.0));
    let s10 = node(&|n| n.x(0.4).y(2.95));
    let s12 = node(&|n| n.x(-0.9).y(4.0));
    let s20 = node(&|n| n.x(0.4 + 2.5).y(2.95));
    let s22 = node(&|n| n.x(-0.9 + 2.5).y(4.0));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e.width(0.1).color("black"));
    edge(&i1, &s10, &|e| e);
    edge(&i0, &s12, &|e| e);
    edge(&s10, &s20, &|e| e);
    edge(&s12, &s22, &|e| e);
    edge(&s20, &o, &|e| e);
    edge(&s22, &o, &|e| e);

    drop(edge);

    fig
}
