#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment{grade: None}
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker{name: name, role: role, next: self.grade.take()};
        self.grade = Some(Box::new(new_worker))
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        Some(self.grade.take().map(|worker| {
            self.grade = worker.next; 
            worker.name
        })?)
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        Some(self.grade.as_ref().map(|worker| (worker.name.clone(), worker.role.clone()))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = WorkEnvironment::new();
        assert!(list.grade.is_none());
    }

    #[test]
    fn test_one_worker() {
        let mut list = WorkEnvironment::new();
        list.add_worker(String::from("CEO"), String::from("Marie"));
        list.remove_worker();
        assert!(list.grade.is_none());
    }

    #[test]
    fn test_two_workers() {
        let mut list = WorkEnvironment::new();
        list.add_worker(String::from("CEO"), String::from("Marie"));
        list.add_worker(String::from("Manager"), String::from("Monica"));
        list.remove_worker();

        assert_eq!(list.grade.as_ref().unwrap().role, "CEO");
        assert_eq!(list.grade.as_ref().unwrap().name, "Marie");
    }

    #[test]
    fn test_more_workers() {
        let mut list = WorkEnvironment::new();
        list.add_worker(String::from("CEO"), String::from("Marie"));
        list.add_worker(String::from("Manager"), String::from("Monica"));
        list.add_worker(String::from("Normal Worker"), String::from("Ana"));
        list.add_worker(String::from("Normal Worker"), String::from("Alice"));
        list.remove_worker();

        assert_eq!(list.grade.as_ref().unwrap().role, "Normal Worker");
        assert_eq!(list.grade.as_ref().unwrap().name, "Ana");

        list.remove_worker();
        list.remove_worker();
        assert_eq!(list.grade.as_ref().unwrap().role, "CEO");
        assert_eq!(list.grade.as_ref().unwrap().name, "Marie");
    }

    #[test]
    fn test_last_worker() {
        let mut list = WorkEnvironment::new();
        list.add_worker(String::from("CEO"), String::from("Marie"));
        list.add_worker(String::from("Manager"), String::from("Monica"));
        list.add_worker(String::from("Normal Worker"), String::from("Ana"));
        list.add_worker(String::from("Normal Worker"), String::from("Alice"));

        assert_eq!(
            list.last_worker().unwrap(),
            (String::from("Alice"), String::from("Normal Worker"))
        );

        list.remove_worker();
        assert_eq!(
            list.last_worker().unwrap(),
            (String::from("Ana"), String::from("Normal Worker"))
        );

        list.remove_worker();
        assert_eq!(
            list.last_worker().unwrap(),
            (String::from("Monica"), String::from("Manager"))
        );

        list.remove_worker();
        assert_eq!(
            list.last_worker().unwrap(),
            (String::from("Marie"), String::from("CEO"))
        );
    }
}
