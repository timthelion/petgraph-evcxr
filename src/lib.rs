//! Used for displaying petgraph graphs in jupyter using the evcxr rust engine.
extern crate petgraph;
use std::fmt::{self};
use std::io::{Write};
use std::process::{Command, Stdio};

use base64;

use crate::petgraph::visit::{GraphProp};
use crate::petgraph::visit::{IntoEdgeReferences, IntoNodeReferences, NodeIndexable};
use petgraph::dot::{Dot, Config};

/// Draw the contents of a dot file.
/// ```rust
/// # use petgraph_evcxr::draw_dot;
/// let dot = "digraph {\
/// 0 [label=\"a\"]\
/// 1 [label=\"b\"]\
/// 0 -> 1 [label=\"a → b\"]\
/// }";
/// draw_dot(dot);
/// ```
pub fn draw_dot<D>(dot: D)
where
    D: fmt::Display,
{
    println!("EVCXR_BEGIN_CONTENT image/png");
    let mut child = Command::new("dot")
        .args(&["-Tpng"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Error running graphviz dot is graphviz installed?");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_fmt(format_args!("{}", dot))
        .expect("Writing failed.");
    let output = child
        .wait_with_output()
        .expect("Failed to run dot is graphviz installed?");
    println!("{}", base64::encode(&output.stdout[..]));
    println!("EVCXR_END_CONTENT");
}

/// Draw a petgraph graph
/// ```rust
/// # extern crate petgraph;
/// # use petgraph::graph::Graph;
/// # use petgraph::dot::Dot;
/// # use petgraph_evcxr::draw_graph;
/// let mut g : Graph<&str, &str> = Graph::new();
/// let a = g.add_node("a");
/// let b = g.add_node("b");
/// g.add_edge(a, b, "a to b");
/// draw_graph(&g);
/// ```
pub fn draw_graph<G>(g: G)
where
    G: NodeIndexable + IntoNodeReferences + IntoEdgeReferences,
    G: GraphProp,
    G::NodeWeight: fmt::Display,
    G::EdgeWeight: fmt::Display,
{
    draw_dot(Dot::new(g));
}


pub fn draw_graph_with_attr_getters<'a, G>(
    g: G,
    config: &'a [Config],
    get_edge_attributes: &'a dyn Fn(G, G::EdgeRef) -> String,
    get_node_attributes: &'a dyn Fn(G, G::NodeRef) -> String,
)
where
    G: NodeIndexable + IntoNodeReferences + IntoEdgeReferences,
    G: GraphProp,
    G::NodeWeight: fmt::Display,
    G::EdgeWeight: fmt::Display,
{
    draw_dot(Dot::with_attr_getters(g, config, get_edge_attributes, get_node_attributes));
}
