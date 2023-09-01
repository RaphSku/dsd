use super::vertex::Vertex;

pub struct VerticesLinks<'a> {
    pub vertices: Vec<Vertex<'a>>
}

impl<'a> VerticesLinks<'a> {
    pub fn new(vertices: Vec<Vertex<'a>>) -> VerticesLinks<'a> {
        return VerticesLinks { 
            vertices 
        };
    }

    pub fn show(&self) -> String {
        let mut result = String::new();
        let iter = self.vertices.iter();
        for vertex in iter {
            let message = format!("{}{}", vertex.name, " -> ");
            result = format!("{}{}", result, message);
        }
        let message = format!("{}", "None");
        result = format!("{}{}", result, message);
        return result;
    }

    pub fn push(&mut self, vertex: Vertex<'a>) {
        self.vertices.push(vertex);
    }

    pub fn remove(&mut self, name: String) -> Option<Vertex<'a>> {
        let mut target_index: isize = -1; 
        for (i, vertex) in self.vertices.iter().enumerate() {
            if vertex.name == name {
                target_index = i as isize;
                break;
            }
        }
        if target_index == -1 {
            return None;
        }
        let vertex = self.vertices.remove(target_index as usize);
        return Some(vertex);
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use rstest::{rstest, fixture};

    use super::{Vertex, VerticesLinks};

    use crate::model::vertex::ProcessName;
    use crate::resources::process::{Process, StorageAccess, Operations};

    #[fixture]
    fn vertices() -> Vec<Vertex<'static>> {
        let storage_access_a = StorageAccess::new("Storage A".to_string(), vec![Operations::WRITE]);
        let storage_access_b = StorageAccess::new("Storage B".to_string(), vec![Operations::APPEND]);

        let process_a = Process::new("Process A".to_string(), false);
        let process_b = Process::new("Process B".to_string(), true);

        let mut process_map_a = HashMap::<ProcessName, Process>::new();
        process_map_a.insert(process_a.name.clone(), process_a);
        let mut process_map_b = HashMap::<ProcessName, Process>::new();
        process_map_b.insert(process_b.name.clone(), process_b);

        let vertices = vec![
            Vertex::new("Node A".to_string(), process_map_a, HashMap::new(), HashMap::new()),
            Vertex::new("Node B".to_string(), process_map_b, HashMap::new(), HashMap::new())
        ];
        return vertices;
    }

    #[rstest]
    fn test_vertices_link_init_s01(vertices: Vec<Vertex>) {
        // Test Setup
        // Testing
        let vertices_link = VerticesLinks::new(vertices.clone());

        // Test Verification
        let actual_vertices = vertices_link.vertices;
        assert_eq!(actual_vertices, vertices);
    }

    #[rstest]
    fn test_vertices_link_show_s01(vertices: Vec<Vertex>) {
        // Test Setup
        let mut expected_message = "".to_string();
        for vertex in vertices.iter() {
            expected_message = format!("{}{}{}", expected_message, vertex.name, " -> ");
        }
        expected_message = format!("{}{}", expected_message, "None");

        // Testing
        let vertices_link = VerticesLinks::new(vertices);
        let actual_message = vertices_link.show();

        // Test Verification
        assert_eq!(actual_message, expected_message);
    }

    #[rstest]
    fn test_vertices_link_show_s02() {
        // Test Setup
        let expected_message = "None".to_string();

        // Testing
        let vertices_link = VerticesLinks::new(vec![]);
        let actual_message = vertices_link.show();

        // Test Verification
        assert_eq!(actual_message, expected_message);
    }

    #[rstest]
    fn test_vertices_link_insertion_s01(vertices: Vec<Vertex>) {
        // Test Setup
        let mut process_map = HashMap::<ProcessName, Process>::new();
        let vertex = Vertex::new("Node Test".to_string(), process_map, HashMap::new(), HashMap::new());

        // Testing
        let mut vertices_link = VerticesLinks::new(vertices);
        vertices_link.push(vertex.clone());

        // Test Verification
        let actual_vertex = &vertices_link.vertices[2];
        assert_eq!(actual_vertex, &vertex);
    }

    #[rstest]
    fn test_vertices_link_removal_s01(vertices: Vec<Vertex>) {
        // Test Setup
        let mut vertices_link = VerticesLinks::new(vertices);

        // Testing
        let actual_vertex = vertices_link.remove("Node A".to_string());

        // Test Verification
        assert_ne!(actual_vertex, None);

        let first_element = &vertices_link.vertices[0];
        assert_ne!(first_element, &actual_vertex.unwrap());
    }
}