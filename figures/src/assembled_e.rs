use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(0.6);

    let mut substrate = fig.substrate_builder(&|s| s.size(2.0).cells(2).visible_axis(false));
    let s = substrate(&|s| s);
    let w = substrate(&|s| s.x(-2.5).y(5.0));
    let e = substrate(&|s| s.x(2.5).y(5.0));
    let n = substrate(&|s| s.y(10.0));
    drop(substrate);

    let mut node = fig.node_builder(&|n| n.size(1.5).edge_offset(0.0).fill("gray!40").y(1.0));
    let i0 = node(&|n| n.x(1.0 / 3.0));
    let i1 = node(&|n| n.x(1.0));
    let i2 = node(&|n| n.x(5.0 / 3.0));
    let o = node(&|n| n.x(1.0).y(6.0).fill("gray"));
    drop(node);

    let mut node = fig.node_builder(&|n| n.size(1.5).edge_offset(0.0));
    let s10 = node(&|n| n.x(0.4).y(2.95));
    //let s11 = node(&|n| n.x(0.0).y(3.55));
    let s12 = node(&|n| n.x(-0.9).y(4.0).opacity(0.6));
    //let s13 = node(&|n| n.x(0.4).y(4.15));

    let s20 = node(&|n| n.x(2.9).y(2.75).opacity(0.6));
    let s21 = node(&|n| n.x(1.6).y(3.2).opacity(0.6));
    let s22 = node(&|n| n.x(2.4).y(4.15).opacity(0.6));
    let s23 = node(&|n| n.x(2.3).y(2.7).opacity(0.6));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e.width(0.1).color("black"));
    edge(&i1, &s10, &|e| e.color("gray!35"));
    edge(&i0, &s12, &|e| e.color("gray!35"));

    edge(&i1, &s21, &|e| e.color("gray!35"));
    edge(&i2, &s21, &|e| e.color("gray!35"));
    edge(&i2, &s20, &|e| e.color("gray!35"));

    edge(&s21, &s23, &|e| e.color("gray!35"));
    edge(&s21, &s22, &|e| e.color("gray!35"));
    edge(&s20, &s22, &|e| e.color("gray!35"));
    edge(&s21, &s10, &|e| e);

    /*edge(&s12, &s11, &|e| e);
    edge(&s10, &s11, &|e| e);
    edge(&s11, &s13, &|e| e);

    edge(&s12, &o, &|e| e);
    edge(&s13, &o, &|e| e);
    edge(&s22, &o, &|e| e);
    edge(&s21, &o, &|e| e);*/
    drop(edge);

    fig
}
