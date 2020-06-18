use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(0.97);

    let mut node = fig.node_builder(&|n| n.size(1.5).outline("none").text_color("gray!60"));
    node(&|n| n.y(-1.1).text("\\_#1"));
    node(&|n| n.x(-1.2 - 1.15).y(2.5).text("\\_#6"));
    node(&|n| n.x(1.2 + 1.15).y(2.5).text("\\_#4"));
    node(&|n| n.y(5.0 + 1.1).text("\\_#9"));
    drop(node);

    let mut node = fig.node_builder(&|n| n.size(14.0).edge_offset(6.0).outline("none"));
    let s = node(&|n| n.text("\\raisebox{7ex}{depth 0}"));
    let w = node(&|n| n.x(-1.2).y(2.5).text("depth 3"));
    let e = node(&|n| n.x(1.2).y(2.5).text("depth 1"));
    let n = node(&|n| n.y(5.0).text("\\raisebox{7ex}{depth 0}"));
    drop(node);

    let mut substrate = fig.substrate_builder(&|s| s.size(1.6).cells(1).axis_arrow_offset(0.15));
    let s_s = substrate(&|s| s.x(-2.0 + 0.4).y(-2.0 + 0.4));
    let w_s = substrate(&|s| s.x(-2.0 * 1.2 - 2.0 + 0.4).y(5.0 - 2.0 + 0.4));
    let e_s = substrate(&|s| s.x(2.0 * 1.2 - 2.0 + 0.4).y(5.0 - 2.0 + 0.4));
    let n_s = substrate(&|s| s.x(-2.0 + 0.4).y(10.0 - 2.0 + 0.4));
    drop(substrate);

    let mut edge =
        fig.edge_builder(&|e| e.width(0.5).pos(0.5).text_color("gray!60").color("gray!90"));
    edge(&s, &w, &|e| {
        e.text("\\_#2").pos(0.3).text_pos("left").xshift(-0.8)
    });
    edge(&s, &e, &|e| e.text("\\_#3").pos(0.3).text_pos("right"));
    edge(&e, &w, &|e| e.text("\\_#5").yshift(0.5));
    edge(&w, &n, &|e| e.text("\\_#7").pos(0.6).text_pos("left"));
    edge(&e, &n, &|e| e.text("\\_#8").pos(0.6).text_pos("right"));
    drop(edge);

    let mut label = fig.label_builder(&|l| l.y(5.05).text_color("gray!60"));
    label(&|l| l.x(1.2).text("\\#"));
    label(&|l| l.x(1.6).text("Topological\\\\ ordering"));
    drop(label);

    let mut node = fig.node_builder(&|n| n.fill("teal"));
    node(&|n| n.x(-0.5));
    node(&|n| n);
    node(&|n| n.x(0.5));
    node(&|n| n.y(5.0).fill("blue"));
    drop(node);

    fig
}
