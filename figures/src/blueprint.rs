use figure;

pub fn create() -> figure::Figure {
    let mut fig = figure::Figure::new();

    fig.add(
        figure::substrate::SubstrateBuilder::default()
            .size(4.0)
            .cells(2)
            .build()
            .unwrap(),
    );

    fig
}
