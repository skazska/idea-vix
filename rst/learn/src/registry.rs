// use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    name: String,
    department: String,
}

pub struct Registry {
    entries: Vec<Employee>,
    // department_index: HashMap<String, Vec<usize>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            entries: Vec::new(),
            // department_index: HashMap::new(),
        }
    }

    pub fn add(&mut self, emploee: &Employee) {
        self.entries.push(emploee.clone());
        // let index = self.entries.len() - 1;
        // self.department_index.entry(emploee.department.clone())
        //     .or_insert_with(Vec::new)
        //     .push(index);
    }

    pub fn list(&self, department: Option<&str>) -> Vec<&Employee> {
        let mut result: Vec<&Employee> = match department {
            Some(dept) => self.entries.iter().filter(|e| e.department == dept).collect(),
            None => self.entries.iter().collect(),
        };

        result.sort_by(|a, b| a.name.cmp(&b.name));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_new() {
        let registry: Registry = Registry::new();
        assert!(registry.entries.is_empty(), "New registry should be empty");
    }
    
    #[test]
    fn test_registry_add() {
        let mut registry = Registry::new();
        let employee = Employee {
            name: "Alice".to_string(),
            department: "Engineering".to_string(),
        };
        
        registry.add(&employee);
        
        assert_eq!(registry.entries.len(), 1, "Registry should contain one entry");
        assert_eq!(registry.entries[0], employee, "The added employee should match the expected employee");
    }

    #[test]
    fn test_registry_list() {
        let mut registry = Registry::new();
        let employee1 = Employee {
            name: "Alice".to_string(),
            department: "Engineering".to_string(),
        };
        let employee2 = Employee {
            name: "Bob".to_string(),
            department: "Marketing".to_string(),
        };
        
        registry.add(&employee2);
        registry.add(&employee1);
        
        let all_employees = registry.list(None);
        assert_eq!(all_employees.len(), 2, "Should list all employees");
        assert_eq!(all_employees[0], &employee1, "First employee should match");
        assert_eq!(all_employees[1], &employee2, "Second employee should match");
        
        let engineering_employees = registry.list(Some("Engineering"));
        assert_eq!(engineering_employees.len(), 1, "Should list one engineering employee");
        assert_eq!(engineering_employees[0], &employee1, "Engineering employee should match");
        
        let marketing_employees = registry.list(Some("Marketing"));
        assert_eq!(marketing_employees.len(), 1, "Should list one marketing employee");
        assert_eq!(marketing_employees[0], &employee2, "Marketing employee should match");
    }
}