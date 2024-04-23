use super::*;

#[test]
fn test_graph_init() {
    let g = init("w1 w2");
    let source = vec![{text: "w1 ", id: "s0"}, {text: "w2 ", id: "s1"}];
    //   const target = [{text: 'w1 ', id: 't0'}, {text: 'w2 ', id: 't1'}]
    //   const edges = edge_record([Edge(['s0', 't0'], []), Edge(['s1', 't1'], [])])
    //   g // => {source, target, edges}
    assert_eq!(g.source, source);
}
