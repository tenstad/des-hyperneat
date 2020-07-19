use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(1.0);

    let mut node = fig.node_builder(&|n| n.size(6.5).edge_offset(2.0).text_size("normalsize"));

    let input1 = node(&|n| n.x(-3.875).y(-0.5).text("$y$"));
    let input2 = node(&|n| n.x(-5.625).y(-0.5).text("$x$"));

    let hidden1 = node(&|n| n.x(-1.75).y(1.0).text("P"));
    let hidden2 = node(&|n| n.x(-3.5).y(1.0).text("P"));
    let hidden3 = node(&|n| n.x(-2.625).y(2.25).text("S"));
    let hidden4 = node(&|n| n.x(-4.75).y(2.0).text("P"));
    let hidden5 = node(&|n| n.x(-6.5).y(2.0).text("P"));
    let hidden6 = node(&|n| n.x(-4.125).y(3.5).text("P"));
    let hidden7 = node(&|n| n.x(-4.125).y(5.0).text("G"));
    let hidden8 = node(&|n| n.x(-4.125).y(6.2));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e);
    edge(&input1, &hidden1, &|e| e);
    edge(&input2, &hidden2, &|e| e);
    edge(&input1, &hidden4, &|e| e);
    edge(&input2, &hidden4, &|e| e);
    edge(&input2, &hidden5, &|e| e);
    edge(&hidden1, &hidden3, &|e| e);
    edge(&hidden2, &hidden3, &|e| e);
    edge(&hidden3, &hidden6, &|e| e);
    edge(&hidden4, &hidden6, &|e| e);
    edge(&hidden5, &hidden6, &|e| e);
    edge(&hidden6, &hidden7, &|e| e);
    edge(&hidden7, &hidden8, &|e| e);
    drop(edge);

    fig
}
