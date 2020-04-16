use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new();

    let mut node = fig.node_builder(&|n| n.size(12.0).edge_offset(2.0).text("d: 1"));
    let s = node(&|n| n);
    let w = node(&|n| n.x(-2_f64.sqrt()).y(2.0).text("d: 2"));
    let e = node(&|n| n.x(2_f64.sqrt()).y(2.0));
    let n = node(&|n| n.y(4.0));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e.width(0.2));
    edge(&s, &w, &|e| e);
    edge(&s, &e, &|e| e);
    edge(&w, &n, &|e| e);
    edge(&e, &n, &|e| e);
    edge(&e, &w, &|e| e);
    drop(edge);

    fig
}
