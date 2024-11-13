use std::fmt;
use Event::*;

use chrono::Duration;
use colored::*;

// Remainder(text):
// size: 50
// color: (50, 50, 50)
// position: Bottom
// content: the text associated to the enum variant
// Registration(chrono::Duration):
// size: 30
// color: (255, 2, 22)
// position: Top
// content: "You have {duration} left before the registration ends"
// Appointment(text):
// size: 100
// color: (200, 200, 3)
// position: Center
// content: text associated to the value
// Holiday:
// size: 25
// color: (0, 255, 0)
// position: Top
// content: "Enjoy your holiday"

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}

#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {}, {})", self.position, self.size, self.content.truecolor(self.color.0, self.color.1, self.color.2))
    }
}

fn format_duration(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    format!("{:02}H:{:02}M:{:02}S", hours, minutes, seconds)
}

impl Event<'_> {
	pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(message) => {
                println!("remainder"); 
                let new_notif = Notification {
                    size: 50,
                    color: (50, 50, 50),
                    position: Position::Bottom,
                    content: message.to_string(),
                }; 
                new_notif
            },
            Event::Registration(duration) => {
                println!("registration");
                let template = "You have {duration} left before the registration ends";
                let formatted_string = template.replace("{duration}", &format_duration(&duration));
                let new_notif = Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: formatted_string,
                }; 
                new_notif
            },
            Event::Appointment(message) => {
                println!("registration");
                let new_notif = Notification {
                    size: 100,
                    color: (200, 200, 3),
                    position: Position::Center,
                    content: message.to_string(),
                }; 
                new_notif
            },
            Event::Holiday => {
                println!("registration");
                let new_notif = Notification {
                    size: 25,
                    color: (0, 255, 0),
                    position: Position::Top,
                    content: "Enjoy your holiday".to_string(),
                }; 
                new_notif
            },
        }
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
