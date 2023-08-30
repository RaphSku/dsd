use super::vertex::Vertex;

pub struct VerticesLinks {
    pub vertices: Vec<Vertex>
}

impl VerticesLinks {
    pub fn new(vertices: Vec<Vertex>) -> Self {
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

    pub fn push(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn remove(&mut self, name: String) -> Option<Vertex> {
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
    use rstest::{rstest, fixture};

    use super::{Vertex, VerticesLinks};

    use crate::resources::process::{Process, StorageAccess, Operations};

    #[fixture]
    fn vertices() -> Vec<Vertex> {
        let storage_access_a = StorageAccess::new("Storage A".to_string(), vec![Operations::WRITE]);
        let storage_access_b = StorageAccess::new("Storage B".to_string(), vec![Operations::APPEND]);

        let process_a = Process::new("Process A".to_string(), false, vec![storage_access_a]);
        let process_b = Process::new("Process B".to_string(), true, vec![storage_access_b]);

        let vertices = vec![
            Vertex::new("Node A".to_string(), vec![process_a]),
            Vertex::new("Node B".to_string(), vec![process_b])
        ];
        return vertices;
    }

    #[rstest]
    fn test_vertices_link_init_s01(vertices: Vec<Vertex>) {
        let vertices_link = VerticesLinks::new(vertices.clone());

        let actual_vertices = vertices_link.vertices;
        assert_eq!(actual_vertices, vertices);
    }

    #[rstest]
    fn test_vertices_link_show_s01(vertices: Vec<Vertex>) {
        let mut expected_message = "".to_string();
        for vertex in vertices.iter() {
            expected_message = format!("{}{}{}", expected_message, vertex.name, " -> ");
        }
        expected_message = format!("{}{}", expected_message, "None");

        let vertices_link = VerticesLinks::new(vertices);

        let actual_message = vertices_link.show();
        assert_eq!(actual_message, expected_message);
    }

    #[rstest]
    fn test_vertices_link_show_s02() {
        let expected_message = "None".to_string();

        let vertices_link = VerticesLinks::new(vec![]);

        let actual_message = vertices_link.show();
        assert_eq!(actual_message, expected_message);
    }

    #[rstest]
    fn test_vertices_link_insertion_s01(vertices: Vec<Vertex>) {
        let vertex = Vertex::new("Node Test".to_string(), vec![]);

        let mut vertices_link = VerticesLinks::new(vertices);
        vertices_link.push(vertex.clone());

        let actual_vertex = &vertices_link.vertices[2];
        assert_eq!(actual_vertex, &vertex);
    }

    #[rstest]
    fn test_vertices_link_removal_s01(vertices: Vec<Vertex>) {
        let mut vertices_link = VerticesLinks::new(vertices);
        let actual_vertex = vertices_link.remove("Node A".to_string());

        assert_ne!(actual_vertex, None);

        let first_element = &vertices_link.vertices[0];
        assert_ne!(first_element, &actual_vertex.unwrap());
    }
}