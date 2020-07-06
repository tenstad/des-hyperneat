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
            .color("gray!20")
    });
    let s_s = substrate(&|s| s.x(-1.4 + 0.4).y(-1.4 + 0.4));
    let m_s = substrate(&|s| s.x(-1.4 + 0.4).y(4.0 - 1.4 + 0.4));
    let n_s = substrate(&|s| s.x(-1.4 + 0.4).y(8.0 - 1.4 + 0.4));
    drop(substrate);

    let mut edge =
        fig.edge_builder(&|e| e.width(0.5).pos(0.5).text_color("gray!60").color("gray!20"));
    edge(&s, &m, &|e| e);
    edge(&m, &n, &|e| e);
    drop(edge);

    let mut node = fig.node_builder(&|n| n.fill("white"));
    let n1 = node(&|n| n.x(-0.2).fill("white"));
    let n2 = node(&|n| n.x(0.2).fill("white"));
    let n3 = node(&|n| n.x(-0.25).y(1.8));
    let n4 = node(&|n| n.x(0.3).y(2.2));
    let n5 = node(&|n| n.y(4.0).fill("white"));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e.width(0.2));
    edge(&n1, &n3, &|e| e);
    edge(&n2, &n4, &|e| e);
    edge(&n3, &n4, &|e| e);
    edge(&n3, &n5, &|e| e);
    edge(&n4, &n5, &|e| e);
    drop(edge);

    /*let mut label = fig.label_builder(&|l| l.x(1.85));
    label(&|l| l.y(4.75).text("Output node"));
    label(&|l| l.y(4.3).text("Input node"));
    label(&|l| l.y(3.85).text("Evolved node"));
    drop(label);*/

    let mut label = fig.label_builder(&|l| l.x(0.5).text_color("blue"));
    label(&|l| l.x(0.4).y(1.0).text("connections"));
    label(&|l| {
        l.y(0.0)
            .text("\\begin{tabular}{ c }input \\\\ nodes\\end{tabular}")
    });
    label(&|l| {
        l.y(2.0)
            .text("\\begin{tabular}{ c }hidden \\\\ nodes\\end{tabular}")
    });
    label(&|l| {
        l.y(4.0)
            .text("\\begin{tabular}{ c }output \\\\ node\\end{tabular}")
    });
    drop(label);

    fig
}
