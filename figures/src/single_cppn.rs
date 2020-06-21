use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(0.75);

    let mut node = fig.node_builder(&|n| n.size(6.5).edge_offset(2.0).text_size("normalsize"));
    let i0 = node(&|n| n.text("$x_1$"));
    let i1 = node(&|n| n.text("$y_1$").x(1.0));
    let i2 = node(&|n| n.text("$x_2$").x(2.0));
    let i3 = node(&|n| n.text("$y_2$").x(3.0));
    let o0 = node(&|n| n.text("$b_a$").x(0.5 - 2.5).y(3.5 - 0.3));
    let o1 = node(&|n| n.text("$e_a$").x(1.5 - 2.5).y(3.5 - 0.12));
    let o2 = node(&|n| n.text("$w_a$").x(2.5 - 2.5).y(3.5));
    let o3 = node(&|n| n.text("$b_b$").x(0.5).y(4.6 - 0.2));
    let o4 = node(&|n| n.text("$e_b$").x(1.5).y(4.6));
    let o5 = node(&|n| n.text("$w_b$").x(2.5).y(4.6 - 0.2));
    let o6 = node(&|n| n.text("$b_c$").x(0.5 + 2.5).y(3.5));
    let o7 = node(&|n| n.text("$e_c$").x(1.5 + 2.5).y(3.5 - 0.12));
    let o8 = node(&|n| n.text("$w_c$").x(2.5 + 2.5).y(3.5 - 0.3));
    let c = node(&|n| {
        n.text("CPPN")
            .shape("rectangle")
            .height(10.0)
            .width(20.0)
            .x(1.5)
            .y(1.6)
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
    edge(&c, &o3, &|e| e);
    edge(&c, &o4, &|e| e);
    edge(&c, &o5, &|e| e);
    edge(&c, &o6, &|e| e);
    edge(&c, &o7, &|e| e);
    edge(&c, &o8, &|e| e);
    drop(edge);

    fig
}
