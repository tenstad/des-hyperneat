use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(0.95);

    let mut node = fig.node_builder(&|n| n.size(8.0).edge_offset(6.0).outline("none"));
    let s = node(&|n| n);
    let m = node(&|n| n.y(2.0));
    let n = node(&|n| n.y(4.0));
    drop(node);

    let mut substrate = fig.substrate_builder(&|s| {
        s.size(1.0)
            .cells(1)
            .axis_arrow_offset(0.15)
            .color("black!80")
    });
    let s_s = substrate(&|s| s.x(-1.4 + 0.4).y(-1.4 + 0.4));
    let m_s = substrate(&|s| s.x(-1.4 + 0.4).y(4.0 - 1.4 + 0.4));
    let n_s = substrate(&|s| s.x(-1.4 + 0.4).y(8.0 - 1.4 + 0.4));
    drop(substrate);

    let mut edge = fig.edge_builder(&|e| e.width(0.5).pos(0.5).text_color("gray!60").color("gray"));
    edge(&s, &m, &|e| e);
    edge(&m, &n, &|e| e);
    drop(edge);

    let mut node = fig.node_builder(&|n| n.fill("white!20").opacity(0.3));
    node(&|n| n.x(-0.2));
    node(&|n| n.x(0.2));
    node(&|n| n.y(4.0).fill("white!20"));
    drop(node);

    let mut label = fig.label_builder(&|l| l.x(-2.3).text_color("blue"));
    label(&|l| {
        l.y(0.0)
            .text("\\begin{tabular}{ c }input \\\\ substrate\\end{tabular}")
    });
    label(&|l| {
        l.y(2.0)
            .text("\\begin{tabular}{ c }hidden \\\\ substrate\\end{tabular}")
    });
    label(&|l| {
        l.y(4.0)
            .text("\\begin{tabular}{ c }output \\\\ substrate\\end{tabular}")
    });
    label(&|l| l.x(-1.0).y(1.0).text("path"));
    drop(label);

    fig
}
