pub mod graph {
    pub  use crate::graph::graph_items::{edge::Edge,node::Node};
    pub use std::collections::HashMap;

    pub struct Graph {
        pub nodes:  Vec<Node>,
        pub edges:  Vec<Edge>,
        pub attrs:  HashMap<String,String>,
    }

    impl Graph {
        pub fn new() -> Self {
            return Graph {
                nodes   :   vec![],
                edges   :   vec![],
                attrs   :   HashMap::new()
            };
        }
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            for i in nodes.into_iter() {
                self.nodes.push(i.clone());
            }
            self
        }
       pub fn get_node(&self, name: &str) -> Option<Node> {
            let n = self.nodes.iter();
            for i in n {
                if &*i.name == name {return Some(i.clone());}
            }
            None
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            for i in edges.into_iter() {
                self.edges.push(i.clone());
            }
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str,&str)]) -> Self {
            for i in attrs {
                self.attrs.insert(i.0.to_string(),i.1.to_string());
            }
            self
        }
    }
    pub mod graph_items {

        pub mod edge {
            pub use std::collections::HashMap;
            pub use std::rc::Rc;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                from    :   Rc<str>,
                to      :   Rc<str>,
                attrs   :   HashMap<String,String>,
            }
            impl Edge {
                pub fn new<T: Into<Rc<str>>>(from: T, to: T) -> Self {
                    return Edge {
                        from: from.into(), 
                        to:   to.into(), 
                        attrs: HashMap::new()};
                }
                pub fn with_attrs(mut self, attrs:  &[(&str,&str)]) -> Self {
                    for i in attrs {
                        self.attrs.insert(i.0.to_string(),i.1.to_string());
                    }
                    self
                }
            }
        }
        pub mod node {
            pub use std::collections::HashMap;
            pub use std::rc::Rc;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub name    :   Rc<str>,
                attrs   :   HashMap<String,String>,
            }
            impl Node {
                pub fn new<T: Into<Rc<str>>>(name: T) -> Self {
                    Node {name: name.into(), attrs: HashMap::new(),}
                }
                pub fn with_attrs(mut self, attrs: &[(&str,&str)]) -> Self {
                    for i in attrs {
                        self.attrs.insert(i.0.to_string(),i.1.to_string());
                    }
                    self
                }
                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    let a = self.attrs.iter();
                    for (k,v) in a {
                        if &*k == key {return Some(v.as_str());}
                    }
                    None
                }

            }
        }
    }
}
