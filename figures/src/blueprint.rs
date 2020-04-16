use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(1.0);

    let mut node = fig.node_builder(&|n| n.size(1.5).outline("white!0").text_color("gray"));
    node(&|n| n.y(0.95).text("1"));
    node(&|n| n.x(-2_f64.sqrt() - 0.5).y(2.0 + 0.8).text("6"));
    node(&|n| n.x(2_f64.sqrt() + 0.5).y(2.0 + 0.8).text("4"));
    node(&|n| n.y(4.0 + 0.95).text("9"));
    drop(node);

    let mut node = fig.node_builder(&|n| n.size(14.0).edge_offset(2.0));
    let s = node(&|n| n.text("inputs"));
    let w = node(&|n| n.x(-2_f64.sqrt()).y(2.0).text("depth 2"));
    let e = node(&|n| n.x(2_f64.sqrt()).y(2.0).text("depth 1"));
    let n = node(&|n| n.y(4.0).text("outputs"));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e.width(0.2).pos(0.1).yshift(2.0).text_color("gray").color("black"));
    edge(&s, &w, &|e| {
        e.text("2")
            .pos(0.9)
            .text_pos("below")
            .yshift(-2.0)
    });
    edge(&s, &e, &|e| {
        e.text("3")
            .pos(0.9)
            .text_pos("below")
            .yshift(-2.0)
    });
    edge(&e, &w, &|e| e.text("5").pos(0.5));
    edge(&w, &n, &|e| e.text("7"));
    edge(&e, &n, &|e| e.text("8"));
    drop(edge);

    fig
}
