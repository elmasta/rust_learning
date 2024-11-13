mod messenger;
pub use messenger::Logger;
pub use messenger::Tracker;

pub use std::collections::HashMap;
pub use std::rc::Rc;
pub use std::cell::RefCell;

#[derive(Debug)]
pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: usize) -> Worker {
        Worker {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        println!("war");
        self.all_messages.borrow_mut().push(msg.to_string());
        if let Some(stripped_str) = msg.strip_prefix("Warning: ") {
            self.mapped_messages.borrow_mut().insert("Warning".to_string(), stripped_str.to_string());
        }
    }

    fn info(&self, msg: &str) {
        println!("inf");
        self.all_messages.borrow_mut().push(msg.to_string());
        if let Some(stripped_str) = msg.strip_prefix("Info: ") {
            self.mapped_messages.borrow_mut().insert("Info".to_string(), stripped_str.to_string());
        }
    }

    fn error(&self, msg: &str) {
        println!("err");
        self.all_messages.borrow_mut().push(msg.to_string());
        if let Some(stripped_str) = msg.strip_prefix("Error: ") {
            self.mapped_messages.borrow_mut().insert("Error".to_string(), stripped_str.to_string());
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_module() {
//         #[derive(Clone, Debug)]
//         struct TestMs {
//             value: Rc<usize>,
//             ms: RefCell<Vec<String>>,
//             correct: Vec<String>,
//         }

//         impl Logger for TestMs {
//             fn warning(&self, message: &str) {
//                 self.ms.borrow_mut().push(message.to_string());
//             }
//             fn info(&self, message: &str) {
//                 self.ms.borrow_mut().push(message.to_string());
//             }
//             fn error(&self, message: &str) {
//                 self.ms.borrow_mut().push(message.to_string());
//             }
//         }

//         let l = TestMs {
//             value: Rc::new(115),
//             ms: RefCell::new(vec![]),
//             correct: vec![
//                 String::from("Info: you are using up to 40% of your quota"),
//                 String::from(
//                     "Warning: you have used up over 80% of your quota! Proceeds with precaution",
//                 ),
//                 String::from("Error: you are over your quota!"),
//             ],
//         };

//         let track = Tracker::new(&l, 5);
//         let _a = l.value.clone();
//         track.peek(&l.value); // 40%
//         let _a1 = l.value.clone();
//         let _a2 = l.value.clone();
//         track.set_value(&l.value); // 80%
//         let _a3 = l.value.clone();
//         track.set_value(&l.value); // 100%

//         for (i, v) in l.ms.into_inner().iter().enumerate() {
//             assert_eq!(v, &l.correct[i])
//         }
//     }

//     #[test]
//     fn test_module_usage_hasmap() {
//         let log = Worker::new(1000);
        
//         let track = Tracker::new(&log, 12);
//         let _clone_test = log.track_value.clone();
//         let _clone_test1 = log.track_value.clone();
//         let _clone_test2 = log.track_value.clone();
//         let _clone_test3 = log.track_value.clone();
//         let _clone_test4 = log.track_value.clone();
//         let _clone_test5 = log.track_value.clone();
//         let _clone_test6 = log.track_value.clone();
//         let _clone_test7 = log.track_value.clone();

//         // warning: 75% of the quota
//         track.set_value(&log.track_value);
//         assert_eq!(
//             log.mapped_messages.borrow().get("Warning").unwrap(),
//             "you have used up over 75% of your quota! Proceeds with precaution"
//         );

//         let _clone_test8 = log.track_value.clone();

//         // warning: 83% of the quota <- most resent of the messages last onw to be added to the hashmap
//         track.set_value(&log.track_value);
//         assert_eq!(
//             log.mapped_messages.borrow().get("Warning").unwrap(),
//             "you have used up over 83% of your quota! Proceeds with precaution"
//         );

//         // info: 83%
//         track.peek(&log.track_value);
//         assert_eq!(
//             log.mapped_messages.borrow().get("Info").unwrap(),
//             "you are using up to 83% of your quota"
//         );

//         let _clone_test9 = log.track_value.clone();
//         // info: 91%
//         track.peek(&log.track_value);
//         assert_eq!(
//             log.mapped_messages.borrow().get("Info").unwrap(),
//             "you are using up to 91% of your quota"
//         );

//         let _clone_test10 = log.track_value.clone();
//         // error: passed the quota
//         track.set_value(&log.track_value);
//         assert_eq!(
//             log.mapped_messages.borrow().get("Error").unwrap(),
//             "you are over your quota!"
//         );
//     }

//     #[test]
//     fn test_module_usage_vector() {
//         let correct = vec![
//             "Info: you are using up to 40% of your quota",
//             "Warning: you have used up over 80% of your quota! Proceeds with precaution",
//             "Info: you are using up to 80% of your quota",
//             "Error: you are over your quota!",
//         ];
//         let log = Worker::new(1);
//         let track = Tracker::new(&log, 5);
//         let _a = log.track_value.clone();
//         // info: 40%
//         track.peek(&log.track_value);
//         let _a1 = log.track_value.clone();
//         let _a2 = log.track_value.clone();

//         // warning: 80%
//         track.set_value(&log.track_value);
//         // info: 80%
//         track.peek(&log.track_value);
//         let _a3 = log.track_value.clone();

//         // error: passed the quota
//         track.set_value(&log.track_value);

//         for (i, v) in log.all_messages.into_inner().iter().enumerate() {
//             assert_eq!(v, correct[i]);
//         }
//     }
// }
