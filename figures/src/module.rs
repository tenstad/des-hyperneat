use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(1.0);

    let mut node = fig.node_builder(&|n| n.size(6.5).edge_offset(2.0).text_size("normalsize"));
    let i0 = node(&|n| n.text("$x_1$"));
    let i1 = node(&|n| n.text("$y_1$").x(1.0));
    let i2 = node(&|n| n.text("$x_2$").x(2.0));
    let i3 = node(&|n| n.text("$y_2$").x(3.0));
    let o0 = node(&|n| n.text("$b$").x(0.5).y(2.6));
    let o1 = node(&|n| n.text("$e$").x(1.5).y(2.6));
    let o2 = node(&|n| n.text("$w$").x(2.5).y(2.6));
    let c = node(&|n| {
        n.text("CPPN")
            .shape("rectangle")
            .height(10.0)
            .width(20.0)
            .x(1.5)
            .y(1.3)
    });
    drop(node);

    let mut edge = fig.edge_builder(&|e| e);

    edge(&i0, &c, &|e| e);
    edge(&i1, &c, &|e| e);
    edge(&i2, &c, &|e| e);
    edge(&i3, &c, &|e| e);
    edge(&c, &o0, &|e| e);
    edge(&c, &o1, &|e| e);
    edge(&c, &o2, &|e| e);
    drop(edge);

    fig
}
