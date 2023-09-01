use std::collections::HashMap;
use crate::resources::{process::{Process, StorageAccess}, storage::{LocalStorage, DistributedStorage}};

pub type ProcessName = String;
pub type StorageName = String;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Vertex<'a> {
    pub name: String,
    pub processes: HashMap<ProcessName, Process>,
    pub local_storages: HashMap<StorageName, LocalStorage>,
    pub distributed_storages: HashMap<StorageName, &'a DistributedStorage>
}

impl<'a> Vertex<'_> {
    pub fn new(name: String, processes: HashMap<ProcessName, Process>, local_storages: HashMap<StorageName, LocalStorage>, distributed_storages: HashMap<StorageName, &'a DistributedStorage>) -> Vertex<'a> {
        return Vertex {
            name,
            processes,
            local_storages,
            distributed_storages,
        };
    }

    pub fn attach_process_to_storage(&mut self, process_name: String, storage_access: StorageAccess, local: bool) {
        let process = self.processes.get_mut(&process_name);
        let target_process: &mut Process;
        match process {
            None => panic!("{}", format!("No process could be found for the process name: {}", &process_name)),
            Some(value) => {
                target_process = value;
            } 
        }
        if local {
            let local_storage = self.local_storages.get(&storage_access.storage_name);
            match local_storage {
                None => panic!("{}", format!("No local storage could be found for the storage name: {}", &storage_access.storage_name)),
                Some(_) => target_process.storage_accesses.push(storage_access)
            }
            return;
        }
        let distributed_storage = self.distributed_storages.get(&storage_access.storage_name);
        match distributed_storage {
            None => panic!("{}", format!("No distributed storage could be found for the storage name: {}", &storage_access.storage_name)),
            Some(_) => target_process.storage_accesses.push(storage_access)
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::fixture;
    use rstest::rstest;

    use super::{ProcessName, StorageName, Vertex};

    use std::{collections::HashMap};
    use crate::resources::{process::{Process, StorageAccess, Operations}, storage::{LocalStorage, DistributedStorage}};

    #[fixture]
    fn process() -> Process {
        let process = Process::new("Process A".to_string(), false);
        return process;
    }

    #[fixture]
    fn storages() -> (LocalStorage, DistributedStorage) {
        let local_storage = LocalStorage::new("Local Storage A".to_string());
        let distributed_storage = DistributedStorage::new("Distributed Storage A".to_string());
        return (local_storage, distributed_storage);
    }

    #[rstest]
    fn test_vertex_init_s01<'a>(process: Process, storages: (LocalStorage, DistributedStorage)) {
        // Test Setup
        let (local_storage, distributed_storage) = storages;

        let mut local_storages = HashMap::<StorageName, LocalStorage>::new();
        local_storages.insert(local_storage.name.clone(), local_storage);

        let mut distributed_storages = HashMap::<StorageName, &'a DistributedStorage>::new();
        distributed_storages.insert(distributed_storage.name.clone(), &distributed_storage);

        let mut process_map = HashMap::<ProcessName, Process>::new();
        process_map.insert(process.name.clone(), process);

        // Testing
        let expected_vertex_name = "Node A".to_string();
        let vertex = Vertex::new(expected_vertex_name.clone(), process_map.clone(), local_storages, distributed_storages);

        // Test Verification
        let actual_vertex_name = vertex.name;
        assert_eq!(actual_vertex_name, expected_vertex_name);

        let actual_vertex_processes = vertex.processes;
        assert_eq!(actual_vertex_processes, process_map);
    }
}