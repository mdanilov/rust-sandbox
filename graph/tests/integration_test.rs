use graph;

#[test]
fn read_small_graph() {
    let test_file = String::from("tests/data/small.graph");
    let g = graph::utils::read_from_file(&test_file);
    assert_eq!(6, g.edges_count);
    assert_eq!(4, g.vertexes_count);
    assert_eq!([0, 0, 2, 4, 6], &g.x[..]);
    assert_eq!([2, 3, 1, 3, 1, 2], &g.a[..]);
}
