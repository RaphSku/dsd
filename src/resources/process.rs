#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Operations {
    READ,
    WRITE,
    APPEND
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct StorageAccess {
    pub storage_name: String,
    pub operations: Vec<Operations>
}

impl StorageAccess {
    pub fn new(storage_name: String, operations: Vec<Operations>) -> Self {
        return StorageAccess { 
            storage_name, 
            operations 
        }
    }
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Process {
    pub name: String,
    pub multi_threaded: bool,
    pub storage_accesses: Vec<StorageAccess>
}

impl Process {
    pub fn new(name: String, multi_threaded: bool) -> Self {
        return Process { 
            name, 
            multi_threaded, 
            storage_accesses: vec![] 
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::StorageAccess;
    use super::Operations;
    use super::Process;

    #[rstest]
    fn test_storage_access_init_s01() {
        // Test Setup
        // Testing
        let expected_storage_name = "Storage A".to_string();
        let storage_access = StorageAccess::new(expected_storage_name.clone(), vec![Operations::WRITE]);

        // Test Verification
        let actual_storage_name = storage_access.storage_name.clone();
        assert_eq!(actual_storage_name, expected_storage_name);

        let actual_operations = storage_access.operations;
        let expected_operations = vec![Operations::WRITE];
        assert_eq!(actual_operations, expected_operations);
    }

    #[rstest]
    fn test_storage_access_init_s02() {
        // Test Setup
        // Testing
        let expected_storage_name = "Storage A".to_string();
        let storage_access = StorageAccess::new(expected_storage_name.clone(), vec![Operations::READ]);

        // Test Verification
        let actual_storage_name = storage_access.storage_name.clone();
        assert_eq!(actual_storage_name, expected_storage_name);

        let actual_operations = storage_access.operations;
        let expected_operations = vec![Operations::READ];
        assert_eq!(actual_operations, expected_operations);
    }

    #[rstest] 
    fn test_storage_access_init_s03() {
        // Test Setup
        // Testing
        let expected_storage_name = "Storage A".to_string();
        let storage_access = StorageAccess::new(expected_storage_name.clone(), vec![Operations::APPEND]);

        // Test Verification
        let actual_storage_name = storage_access.storage_name.clone();
        assert_eq!(actual_storage_name, expected_storage_name);

        let actual_operations = storage_access.operations;
        let expected_operations = vec![Operations::APPEND];
        assert_eq!(actual_operations, expected_operations);
    }

    #[rstest]
    fn test_process_init_s01() {
        // Test Setup
        // Testing
        let expected_process_name = "Process A".to_string();
        let process = Process::new(expected_process_name.clone(), true);

        // Test Verification
        let actual_process_name = process.name.clone();
        assert_eq!(actual_process_name, expected_process_name);

        let actual_multi_threaded_flag = process.multi_threaded.clone();
        assert_eq!(actual_multi_threaded_flag, true);

        let actual_storage_accesses = process.storage_accesses.clone();
        assert_eq!(actual_storage_accesses, Vec::<StorageAccess>::new());
    }
}