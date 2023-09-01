use std::collections::HashMap;

use super::linked_vertices::VerticesLinks;

type VertexName = String;

pub struct Graph<'a> {
    pub adjacency_links: HashMap<VertexName, VerticesLinks<'a>>
}