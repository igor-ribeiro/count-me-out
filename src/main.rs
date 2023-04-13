use std::io;

enum Hand {
    Left,
    Right,
}

struct Player {
    name: String,
    hands: (u32, u32),
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            hands: (1, 1),
        }
    }

    fn add(&mut self, hand: Hand, value: u32) {
        let (left, right) = self.hands;

        match hand {
            Hand::Left => {
                self.hands.0 = self.safe_value(left + value);
            }
            Hand::Right => {
                self.hands.1 = self.safe_value(right + value);
            }
        }
    }

    fn safe_value(&self, value: u32) -> u32 {
        value % 5
    }

    fn split(&mut self) -> bool {
        let value = match self.hands {
            (0, 0) => return false,
            (0, value) if value % 2 == 0 => value,
            (value, 0) if value % 2 == 0 => value,
            (_, _) => return false,
        };

        let value = value / 2;

        self.hands = (value, value);

        true
    }

    fn display(&self, is_turn: bool) {
        println!(
            "----- {} ({}) -----",
            self.name,
            if is_turn { "x" } else { " " }
        );
        println!("L: {}    R: {}", self.hands.0, self.hands.1);
    }
}

struct Game<'a> {
    players: [&'a mut Player; 2],
    turn: usize,
}

impl<'a> Game<'a> {
    fn new(players: [&'a mut Player; 2]) -> Game {
        Game { players, turn: 0 }
    }

    fn display(&self) {
        clear_display();

        for (i, player) in self.players.iter().enumerate() {
            player.display(i == self.turn);
            println!("");
            println!("");
            println!("");
        }
    }

    fn act(&mut self, from: Hand, to: Hand) {
        let next_player_index = self.get_next_player();
        let points_to_transfer = match from {
            Hand::Left => self.players[self.turn].hands.0,
            Hand::Right => self.players[self.turn].hands.1,
        };

        match to {
            Hand::Left => {
                self.players[next_player_index].add(Hand::Left, points_to_transfer);
            }
            Hand::Right => {
                self.players[next_player_index].add(Hand::Right, points_to_transfer);
            }
        }

        self.set_next_player();
    }

    fn get_winner(&self) -> Option<String> {
        match (self.players[0].hands, self.players[1].hands) {
            ((0, 0), (l, r)) if l > 0 || r > 0 => Some(self.players[1].name.to_string()),
            ((l, r), (0, 0)) if l > 0 || r > 0 => Some(self.players[0].name.to_string()),
            ((_, _), (_, _)) => None,
        }
    }

    fn split(&mut self) {
        if self.players[self.turn].split() {
            self.set_next_player();
        }
    }

    fn get_next_player(&self) -> usize {
        (self.turn + 1) % self.players.len()
    }

    fn set_next_player(&mut self) {
        self.turn = self.get_next_player();
    }
}

fn clear_display() {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_input(label: String) -> String {
    println!("{}", label);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");

    input.trim().to_string()
}

fn main() {
    let mut player_a = Player::new("Player A".to_string());
    let mut player_b = Player::new("Player B".to_string());
    let mut game = Game::new([&mut player_a, &mut player_b]);

    loop {
        game.display();

        let winner = game.get_winner();

        if winner.is_some() {
            println!("{} is the winner", winner.unwrap());
            break;
        }

        let from = match get_input("From: ".to_string()).as_ref() {
            "l" => Hand::Left,
            "r" => Hand::Right,
            "s" | "split" => {
                game.split();

                continue;
            }
            "quit" => break,
            _ => continue,
        };
        let to = match get_input("To: ".to_string()).as_ref() {
            "l" => Hand::Left,
            "r" => Hand::Right,
            "quit" => break,
            _ => continue,
        };

        game.act(from, to);
    }
}
