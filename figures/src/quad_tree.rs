use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new(1.15);

    let mut node = fig.node_builder(&|n| n.size(3.0).edge_offset(3.0));
    let n1 = node(&|n| n.x(0.0).y(0.0).fill("black").outline("blue"));
    let n2 = node(&|n| n.x(1.0).y(0.0).fill("black").outline("blue"));
    let n3 = node(&|n| n.x(2.0).y(0.0).fill("white").outline("blue"));
    let n4 = node(&|n| n.x(3.0).y(0.0).fill("black").outline("blue"));
    let n5 = node(&|n| n.x(0.0).y(1.0).fill("black").outline("teal"));
    let n6 = node(&|n| n.x(1.0).y(1.0).fill("black").outline("teal"));
    let n7 = node(&|n| n.x(2.0).y(1.0).fill("black").outline("white"));
    let n8 = node(&|n| n.x(3.0).y(1.0).fill("black").outline("teal"));
    let n9 = node(&|n| n.x(0.0).y(2.0).fill("black").outline("green"));
    let n10 = node(&|n| n.x(1.0).y(2.0).fill("black").outline("white"));
    let n11 = node(&|n| n.x(2.0).y(2.0).fill("black").outline("green"));
    let n12 = node(&|n| n.x(3.0).y(2.0).fill("black").outline("green"));
    let n13 = node(&|n| n.x(1.0).y(3.0).fill("black").outline("white"));
    drop(node);

    let mut edge = fig.edge_builder(&|e| e.width(0.2).pos(0.5).text_color("blue").color("gray"));
    edge(&n13, &n9, &|e| e);
    edge(&n13, &n10, &|e| e);
    edge(&n13, &n11, &|e| e);
    edge(&n13, &n12, &|e| e);
    edge(&n10, &n5, &|e| e);
    edge(&n10, &n6, &|e| e);
    edge(&n10, &n7, &|e| e);
    edge(&n10, &n8, &|e| e);
    edge(&n7, &n1, &|e| e);
    edge(&n7, &n2, &|e| e);
    edge(&n7, &n3, &|e| e);
    edge(&n7, &n4, &|e| e);
    drop(edge);

    fig
}
