use graphics::prelude::*;

fn main() {
    let items = GraphicItems(vec![
        Path(
            PathSegments(vec![M(0., 0.), L(50., 50.), L(100., 0.), L(0., 0.)]),
            Strokestyle(5., Orange),
            NoFill,
            PathCacheInfo::Cache("Hey".to_string(), 0.0, 0.0),
        ),
        Path(
            PathSegments(vec![M(0., 0.), L(50., 50.), L(100., 0.), L(0., 0.)]),
            Strokestyle(5., Red),
            NoFill,
            PathCacheInfo::Cache("Hey".to_string(), 10.0, 0.0),
        ),
    ]);
    let svg = SvgBuilder::new().build(items, None).unwrap();
    std::fs::write("./examples/graphicstest.svg", svg).unwrap();
}
