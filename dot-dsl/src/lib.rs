pub mod graph {
    use crate::graph::graph_items::{edge::Edge,node::Node};
    pub use std::collections::HashMap;

    pub struct Graph {
        nodes:  Vec<Node>,
        edges:  Vec<Edge>,
        attrs:  HashMap<String,String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes   :   Vec::new(),
                edges   :   Vec::new(),
                attrs   :   HashMap::new(),
            }
        }
        pub fn with_nodes(&mut self, nodes: &[Node]) {
            for i in nodes.into_iter() {
                self.nodes.push(i.clone());
            }
        }
        pub fn get_node(&self, name: &str) -> Option<Node> {
            let n = self.nodes.into_iter();
            for i in n {
                if i.get_name() == name {return Some(i);}
            }
            None
        }
        pub fn with_edges(&mut self, edges: &[Edge]) {
            for i in edges.into_iter() {
                self.edges.push(i.clone());
            }
        }
        pub fn with_attrs(&mut self, attrs: &[(&str,&str)]) {
            for i in attrs {
                self.attrs.insert(i.0.to_string(),i.1.to_string());
            }
        }
    }
    pub mod graph_items {
        pub mod edge {
            pub use std::collections::HashMap;

            #[derive(Debug, Clone)]
            pub struct Edge {
                from    :   String,
                to      :   String,
                attrs   :   HashMap<String,String>,
            }
            impl Edge {
                pub fn new<T: Into<String>>(from: T, to: T) -> Self {
                    Edge {from: from.into(), to:   to.into(), attrs: HashMap::new(),}
                }
                pub fn with_attrs(&mut self, attrs:  &[(&str,&str)]) {
                    for i in attrs {
                        self.attrs.insert(i.0.to_string(),i.1.to_string());
                    }
                }
            }
        }
        pub mod node {
            pub use std::collections::HashMap;

            #[derive(Debug, Clone)]
            pub struct Node {
                name    :   String,
                attrs   :   HashMap<String,String>,
            }
            impl Node {
                pub fn new<T: Into<String>>(name: T) -> Self {
                    Node {name: name.into(), attrs: HashMap::new(),}
                }
                pub fn get_name(&self) -> &str {
                    self.name.as_str()
                }
                pub fn with_attrs(&mut self, attrs: &[(&str,&str)]) {
                    for i in attrs {
                        self.attrs.insert(i.0.to_string(),i.1.to_string());
                    }
                }
                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    let a = self.attrs.into_iter();
                    for (k,v) in a {
                        if &k == key {return Some(v.as_str());}
                    }
                    None
                }

            }
        }
    }
}
