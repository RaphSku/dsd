use crate::resources::process::Process;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Vertex {
    pub name: String,
    pub processes: Vec<Process>
}

impl Vertex {
    pub fn new(name: String, processes: Vec<Process>) -> Self {
        return Vertex {
            name,
            processes
        };
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::Vertex;

    use crate::resources::process::{Process, StorageAccess, Operations};

    #[rstest]
    fn test_vertex_init_s01() {
        let storage_access = StorageAccess::new("Storage A".to_string(), vec![Operations::WRITE]);

        let process = Process::new("Process A".to_string(), false, vec![storage_access]);

        let expected_vertex_name = "Node A".to_string();
        let vertex = Vertex::new(expected_vertex_name.clone(), vec![process.clone()]);

        let actual_vertex_name = vertex.name;
        assert_eq!(actual_vertex_name, expected_vertex_name);

        let actual_vertex_processes = vertex.processes;
        let expected_vertex_processes = vec![process];
        assert_eq!(actual_vertex_processes, expected_vertex_processes);
    }
}