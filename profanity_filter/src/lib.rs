pub struct Message {
    content: String,
    user: String,
}

impl Message {

    pub fn new(ms: String, u: String) -> Message {
        Message{content: ms, user: u}
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.contains("stupid") || self.content.is_empty() {
            return None;
        }
        return Some(&self.content);
    }

}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    if ms.send_ms().is_some() {
        return (true, &ms.content);
    } else {
        return (false, "ERROR: illegal");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_ms() {
        let v = vec![
            Message::new("".to_string(), "toby".to_string()),
            Message::new("stupid".to_string(), "jack".to_string()),
            Message::new("you are stupid".to_string(), "jacob".to_string()),
        ];
        for value in v {
            let (t, _) = check_ms(&value);
            assert!(!t);
        }
    }

    #[test]
    fn test_ok_ms() {
        let v = vec![
            Message::new("get out of the car".to_string(), "police".to_string()),
            Message::new("no!".to_string(), "thief".to_string()),
            Message::new("get the werewolf".to_string(), "police".to_string()),
            Message::new("wait the wha...".to_string(), "thief".to_string()),
        ];
        for value in v {
            let (t, _) = check_ms(&value);
            assert!(t);
        }
    }
}
