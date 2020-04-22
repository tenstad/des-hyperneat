use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(1.0);

    let mut node = fig.node_builder(&|n| n.size(1.5).outline("white!0").text_color("gray!60"));
    node(&|n| n.y(-1.1).text("\\_#1"));
    node(&|n| n.x(-1.2 - 1.15).y(2.24).text("\\_#6"));
    node(&|n| n.x(1.2 + 1.15).y(2.24).text("\\_#4"));
    node(&|n| n.y(4.48 + 1.1).text("\\_#9"));
    drop(node);

    let mut node = fig.node_builder(&|n| n.size(18.0).edge_offset(2.0));
    let s = node(&|n| n.text("\\raisebox{7ex}{inputs}"));
    let w = node(&|n| n.x(-1.2).y(2.24).text("depth 2"));
    let e = node(&|n| n.x(1.2).y(2.24).text("depth 1"));
    let n = node(&|n| n.y(4.48).text("\\raisebox{7ex}{outputs}"));
    drop(node);

    let mut edge =
        fig.edge_builder(&|e| e.width(0.2).pos(0.5).text_color("gray!60").color("black"));
    edge(&s, &w, &|e| {
        e.text("\\_#2").pos(0.3).text_pos("left").xshift(-0.8)
    });
    edge(&s, &e, &|e| e.text("\\_#3").pos(0.3).text_pos("right"));
    edge(&e, &w, &|e| e.text("\\_#5"));
    edge(&w, &n, &|e| e.text("\\_#7").pos(0.6).text_pos("left"));
    edge(&e, &n, &|e| e.text("\\_#8").pos(0.6).text_pos("right"));
    drop(edge);

    let mut label = fig.label_builder(&|l| l.y(5.05).text_color("gray!60"));
    label(&|l| l.x(1.2).text("\\#"));
    label(&|l| l.x(1.6).text("Topological\\\\ ordering"));
    drop(label);

    let mut node = fig.node_builder(&|n| n.fill("gray!40"));
    node(&|n| n.x(-0.5));
    node(&|n| n);
    node(&|n| n.x(0.5));
    node(&|n| n.y(4.48).fill("gray"));
    drop(node);

    fig
}
