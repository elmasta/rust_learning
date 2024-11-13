pub use std::rc::Rc;
pub use std::cell::RefCell;

pub trait Logger {
	fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: usize,
    pub max: usize,
}

impl<'a> Tracker<'a>{
    pub fn new(logger: &'a dyn Logger, max: usize) -> Tracker<'a> {
        Tracker{ logger, value: 0, max }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        
        let percentage = Rc::strong_count(value) as f64 / self.max as f64 * 100.0;
        println!("{:?}, {:?}", Rc::strong_count(value), self.max);
        println!("{:?}", percentage);
        println!("??????????????????");
        if percentage >= 100.0 {
            println!("error");
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70.0 && percentage < 100.0 {
            println!("war");
            self.logger.warning(&format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", percentage.floor()));
        } else {
            self.logger.info(&format!("Info: you are using up to {}% of your quota", percentage.floor()));
        }
    }

    pub fn peek(&self, value: &Rc<usize>) {
        println!("test");
        println!("{:?}, {:?}", Rc::strong_count(value), self.max);
        let percentage = Rc::strong_count(value) as f64 / self.max as f64 * 100.0;
        println!("{:?}", percentage);
        println!("#######");
        self.logger.info(&format!("Info: you are using up to {}% of your quota", percentage.floor()));
    }
}