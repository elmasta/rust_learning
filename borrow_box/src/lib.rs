#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession{id: id, p1: (p1_name, 0), p2: (p2_name, 0), nb_games: nb_games})
    }

    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 == self.p2.1 {
            (String::from("Same score! tied"), self.p1.1)
        } else if self.p1.1 > self.p2.1 {
            self.p1.clone()
        } else {
            self.p2.clone()
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        if self.nb_games > 0 && self.p1.1 != 3 && self.p2.1 != 3 {
            if self.p1.0 == user_name {
                self.p1.1 += 1;
                self.nb_games -= 1
            } else if self.p2.0 == user_name {
                self.p2.1 += 1;
                self.nb_games -= 1
            }
        }
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_games() -> Vec<Box<GameSession>> {
        vec![
            GameSession::new(0, String::from("player1"), String::from("player2"), 1),
            GameSession::new(1, String::from("Alice"), String::from("Mark"), 3),
            GameSession::new(2, String::from("Jack"), String::from("Miller"), 5),
        ]
    }

    #[test]
    fn test_create() {
        let games = create_games();
        assert_eq!(
            *games[0],
            GameSession {
                id: 0,
                p1: (String::from("player1"), 0),
                p2: (String::from("player2"), 0),
                nb_games: 1
            }
        );
        assert_eq!(
            *games[1],
            GameSession {
                id: 1,
                p1: (String::from("Alice"), 0),
                p2: (String::from("Mark"), 0),
                nb_games: 3
            }
        );
        assert_eq!(
            *games[2],
            GameSession {
                id: 2,
                p1: (String::from("Jack"), 0),
                p2: (String::from("Miller"), 0),
                nb_games: 5
            }
        );
    }

    #[test]
    fn test_update_and_read() {
        let mut games = create_games();
        games[0].update_score(String::from("player1"));
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));

        games[0].update_score(String::from("player2"));
        // this will stay the same because the nb_games is 1 so if one
        // of the players wins just once it will no longer increment the score
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));

        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Miller"));
        games[2].update_score(String::from("Miller"));
        // tie between players
        assert_eq!(
            games[2].read_winner(),
            (String::from("Same score! tied"), 2)
        );

        games[2].update_score(String::from("Jack"));
        assert_eq!(games[2].read_winner(), (String::from("Jack"), 3));
    }

    #[test]
    fn test_stop_updating() {
        let mut games = create_games();
        games[0].update_score(String::from("player1"));
        games[0].update_score(String::from("player1"));
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));

        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        assert_eq!(games[2].read_winner(), (String::from("Jack"), 3));
    }

    #[test]
    fn test_delete() {
        let game = GameSession::new(0, String::from("Alice"), String::from("Mark"), 3);
        let game1 = GameSession::new(23, String::from("Jack"), String::from("Miller"), 1);

        assert_eq!(game.delete(), String::from("game deleted: id -> 0"));
        assert_eq!(game1.delete(), String::from("game deleted: id -> 23"));
    }

    #[test]
    fn test_different_name() {
        let mut game = GameSession::new(0, String::from("Alice"), String::from("Mark"), 3);

        game.update_score(String::from("Mark"));
        assert_eq!(game.read_winner(), (String::from("Mark"), 1));

        game.update_score(String::from("Miller"));
        assert_eq!(game.read_winner(), (String::from("Mark"), 1));
    }
}
