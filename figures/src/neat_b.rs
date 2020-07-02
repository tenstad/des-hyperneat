use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(0.65);

    let mut node = fig.node_builder(&|n| {
        n.size(5.0)
            .inner_sep(1.0)
            .edge_offset(2.0)
            .text_size("normalsize")
    });
    let n1 = node(&|n| n.x(0.0).text("$a$"));
    let n2 = node(&|n| n.x(3.0).text("$b$"));
    let n3 = node(&|n| n.x(6.0).text("$c$"));
    let n5 = node(&|n| n.x(3.0).y(1.6).text("$e$"));
    let n6 = node(&|n| n.x(3.0).y(3.2).text("$f$"));
    let n4 = node(&|n| n.x(3.0).y(6.0).text("$d$"));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e.width(0.08).pos(0.5).text_color("blue").color("black"));
    edge(&n1, &n4, &|e| e.text("1").pos(0.5).text_pos("left"));
    edge(&n1, &n6, &|e| e.text("10").pos(0.9).text_pos("left"));
    edge(&n6, &n4, &|e| e.text("7").pos(0.2).text_pos("right"));
    edge(&n2, &n5, &|e| e.text("4").pos(0.35).text_pos("left"));
    edge(&n5, &n6, &|e| e.text("6").pos(0.5).text_pos("right"));
    edge(&n3, &n5, &|e| {
        e.text("\\raisebox{-1em}{9}").pos(0.3).text_pos("left")
    });
    edge(&n3, &n4, &|e| e.text("3").pos(0.5).text_pos("right"));
    drop(edge);

    fig
}
