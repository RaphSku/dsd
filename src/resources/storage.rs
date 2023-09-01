#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct LocalStorage {
    pub name: String,
    pub vertex_name: String
}

impl LocalStorage {
    pub fn new(name: String) -> Self {
        return LocalStorage {
            name,
            vertex_name: String::from("")
        }
    }
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct DistributedStorage {
    pub name: String,
    pub vertices_names: Vec<String>
}

impl DistributedStorage {
    pub fn new(name: String) -> Self {
        return DistributedStorage { 
            name,
            vertices_names: vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::{LocalStorage, DistributedStorage};

    #[rstest]
    fn test_local_storage_init_s01() {
        // Test Setup
        // Testing
        let expected_storage_name = "Storage A".to_string();
        let local_storage = LocalStorage::new(expected_storage_name.clone());

        // Test Verification
        let actual_storage_name = local_storage.name;
        let actual_vertex_name = local_storage.vertex_name;
        assert_eq!(actual_storage_name, expected_storage_name);
        assert_eq!(actual_vertex_name, "".to_string());
    }

    #[rstest]
    fn test_distributed_storage_init_s01() {
        // Test Setup
        // Testing
        let expected_storage_name = "Storage A".to_string();
        let distributed_storage = DistributedStorage::new(expected_storage_name.clone());

        // Test Verification
        let actual_storage_name = distributed_storage.name;
        let actual_vertices_names = distributed_storage.vertices_names;
        assert_eq!(actual_storage_name, expected_storage_name);
        assert_eq!(actual_vertices_names, Vec::<String>::new());
    }
}