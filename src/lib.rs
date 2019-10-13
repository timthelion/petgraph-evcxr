extern crate petgraph;
use std::fmt::{self, Display};
use std::io::{self, Write};
use std::process::{Command, Stdio};

use base64;

use crate::petgraph::visit::{Data, GraphProp, NodeRef};
use crate::petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences, NodeIndexable};
use petgraph::dot::Dot;
use petgraph::graph::Graph;

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
        .write_fmt(format_args!("{}", dot));
    let output = child
        .wait_with_output()
        .expect("Failed to run dot is graphviz installed?");
    println!("{}", base64::encode(&output.stdout[..]));
    println!("EVCXR_END_CONTENT");
}

pub fn draw_graph<G>(g: G)
where
    G: NodeIndexable + IntoNodeReferences + IntoEdgeReferences,
    G: GraphProp,
    G::NodeWeight: fmt::Display,
    G::EdgeWeight: fmt::Display,
{
    draw_dot(Dot::new(&g));
}
