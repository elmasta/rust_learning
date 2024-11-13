mod err;
use err::{ ParseErr, ReadErr };
use std::fs::File;
use std::io::Read;

pub use json::{parse, stringify, JsonValue, JsonError};
pub use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(err) => return Err(Box::new(ReadErr { child_err: Box::new(err) })),
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let parsed = parse(&contents);
        match parsed {
            Ok(ref f) => f,
            Err(err) => {
                println!("{:?}", err);
                return Err(ParseErr::Malformed(err.into()).into());
                //return Err(Box::new(ParseErr::Malformed { ch: Box::new(err) }));
                //return Err(ParseErr::Malformed(err.into()).into());
            },
            //json::Error::UnexpectedCharacter { ch, line, column } => ParseErr::UnexpectedCharacter { ch, line, column },
            //JsonError::UnexpectedCharacter(Variant::TupleVariant(ch, line, column)) => ParseErr::Malformed { ch, line, column },
        };
        let unwrap_parsed: JsonValue = parsed.unwrap();

        let title = unwrap_parsed["title"].as_str().ok_or(ParseErr::Empty)?.to_string();
        
        let tasks: Vec<Task> = unwrap_parsed["tasks"].members()
            .map(|task| {
                Ok(Task {
                    id: task["id"].as_u32().unwrap_or_default(),
                    description: task["description"].as_str().unwrap_or_default().to_string(),
                    level: task["level"].as_u32().unwrap_or_default(),
                })
            })
            .collect::<Result<Vec<_>, ParseErr>>()?;

        if tasks.is_empty() {
            return Err("Fail to parse todo".into());
        //     return Err(Box::new(ReadErr { child_err: Box::new("nothing") }));
        }
        Ok(TodoList { title, tasks })

        //Ok(TodoList{title: "test".to_string(), tasks: vec![Task{id: 1, description: "lolilol".to_string(), level: 9000}]})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json::{object, JsonValue};
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::Write;

    fn new_todo(s: String, v: Vec<Task>) -> TodoList {
        TodoList { title: s, tasks: v }
    }

    fn run(s: JsonValue, f: &str) -> Result<TodoList, Box<dyn Error>> {
        File::create(f)?;
        let mut file = OpenOptions::new().append(true).open(f)?;
        file.write_all(s.dump().as_bytes())?;
        let result = TodoList::get_todo(f);
        fs::remove_file(f)?;
        return result;
    }

    #[test]
    fn test_good_todo() {
        let file_name = "todo.json";
        let good_struct = new_todo(
            String::from("todo list for something"),
            vec![
                Task {
                    id: 0,
                    description: "do this".to_string(),
                    level: 0,
                },
                Task {
                    id: 1,
                    description: "do that".to_string(),
                    level: 5,
                },
            ],
        );
        let obj = object! {
            "title" : "todo list for something",
            "tasks": [
                { "id": 0, "description": "do this", "level": 0 },
                { "id": 1, "description": "do that", "level": 5 }
            ]
        };

        let result = run(obj, file_name).unwrap();

        assert_eq!(result.title, good_struct.title);
        assert_eq!(&result.tasks, &good_struct.tasks);
    }

    #[test]
    fn test_empty_tasks() {
        let result = run(
            object! {
            "title" : "empty tasks",
            "tasks": []},
            "empty_tasks.json",
        )
        .unwrap_err();

        assert_eq!(result.to_string(), "Fail to parses todo");
        assert!(result.source().is_none());
    }

    #[test]
    fn test_read() {
        let result = TodoList::get_todo("no_file.json").unwrap_err();
        assert_eq!(result.to_string(), "Fail to read todo file");
    }

    #[test]
    #[should_panic(
        expected = "Fail to read todo file Some(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })"
    )]
    fn test_read_error() {
        let result = TodoList::get_todo("no_file.json");
        result.unwrap_or_else(|e| panic!("{} {:?}", e.to_string(), e.source()));
    }

    #[test]
    #[should_panic(
        expected = "Fail to parse todo Some(Malformed(UnexpectedCharacter { ch: \',\', line: 1, column: 15 }))"
    )]
    fn test_malformed_error() {
        let file_name = "malformed.json";
        File::create(file_name).unwrap();
        let mut file = OpenOptions::new().append(true).open(file_name).unwrap();
        file.write_all(r#"{"something": ,}"#.as_bytes()).unwrap();
        let result = TodoList::get_todo(file_name);
        fs::remove_file(file_name).unwrap();

        result.unwrap_or_else(|e| panic!("{} {:?}", e.to_string(), e.source()));
    }
}
