use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(0.65);

    /*let mut node = fig.node_builder(&|n| n.size(1.5).outline("none").text_color("gray!60"));
    node(&|n| n.x(-1.2 - 1.15).y(2.5).text("\\_#1"));
    node(&|n| n.x(1.2 + 1.15).y(2.5).text("\\_#2"));
    node(&|n| n.y(8.0 + 1.1).text("\\_#5"));
    drop(node);*/

    let mut substrate =
        fig.substrate_builder(&|s| s.size(1.6).cells(1).axis_arrow_offset(0.15).color("black"));
    let w_s = substrate(&|s| s.x(-2.0 * 1.2 - 2.0 + 0.4).y(5.0 - 2.0 + 0.4));
    let e_s = substrate(&|s| s.x(2.0 * 1.2 - 2.0 + 0.4).y(5.0 - 2.0 + 0.4));
    let n_s = substrate(&|s| s.x(-2.0 + 0.4).y(16.0 - 2.0 + 0.4));
    drop(substrate);

    let mut node = fig.node_builder(&|n| {
        n.size(6.0)
            .edge_offset(6.0)
            .outline("none")
            .shape("rectangle")
    });
    let w = node(&|n| n.x(-1.2).y(2.5).text(""));
    let e = node(&|n| n.x(1.2).y(2.5).text(""));
    let n = node(&|n| n.y(8.0).text(""));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e.width(0.2).pos(0.5).text_color("gray!60").color("gray"));
    edge(&w, &n, &|e| e.text("").pos(0.6).text_pos("right"));
    edge(&e, &n, &|e| e.text("").pos(0.6).text_pos("left"));
    drop(edge);

    let mut node = fig.node_builder(&|n| {
        n.size(5.0)
            .inner_sep(1.0)
            .edge_offset(2.0)
            .text_size("scriptsize")
    });
    let w_ = node(&|n| n.x(-1.2).y(2.5 + 0.3).text("$a$"));
    let e_ = node(&|n| n.x(1.2).y(2.5 + 0.3).text("$b$"));
    let n_ = node(&|n| n.x(0.0).y(8.0 + 0.3).text("$c$"));
    let ee_ = node(&|n| n.x(-0.7).y(5.0 + 0.3).text("$d$"));
    let ee_ = node(&|n| n.x(0.7).y(5.0 + 0.3).text("$e$"));
    drop(node);

    fig
}
