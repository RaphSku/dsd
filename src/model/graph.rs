use std::collections::HashMap;

use super::linked_vertices::VerticesLinks;

pub struct Graph {
    pub adjacency_links: HashMap<String, VerticesLinks>
}