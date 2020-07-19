use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(1.0);

    let mut node = fig.node_builder(&|n| n.size(6.5).edge_offset(2.0).text_size("normalsize"));

    let input1 = node(&|n| n.x(0.0).y(0.0).text("$x_1$"));
    let input2 = node(&|n| n.x(1.5).y(0.0).text("$y_1$"));
    let input3 = node(&|n| n.x(3.0).y(0.0).text("$x_2$"));
    let input4 = node(&|n| n.x(4.5).y(0.0).text("$y_2$"));

    let hidden1 = node(&|n| n.x(0.5).y(1.5));
    let hidden2 = node(&|n| n.x(2.6).y(1.5));
    let hidden3 = node(&|n| n.x(3.7).y(2.4));
    let hidden4 = node(&|n| n.x(1.5).y(2.7));
    let hidden5 = node(&|n| n.x(2.25).y(4.0));
    let hidden6 = node(&|n| n.x(2.25).y(5.45).text("$w$"));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e);
    edge(&input1, &hidden1, &|e| e);
    edge(&input3, &hidden1, &|e| e);
    edge(&input2, &hidden2, &|e| e);
    edge(&input4, &hidden2, &|e| e);
    edge(&input3, &hidden3, &|e| e);
    edge(&input4, &hidden3, &|e| e);
    edge(&hidden1, &hidden4, &|e| e);
    edge(&input2, &hidden4, &|e| e);
    edge(&hidden2, &hidden4, &|e| e);
    edge(&hidden4, &hidden5, &|e| e);
    edge(&hidden2, &hidden5, &|e| e);
    edge(&hidden3, &hidden5, &|e| e);
    edge(&hidden5, &hidden6, &|e| e);
    drop(edge);

    fig
}
