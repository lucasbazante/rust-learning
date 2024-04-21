pub mod game {
    pub enum Player {
        X,
        O,
    }

    impl Player {
        pub fn next_player(&mut self) {
            *self = match *self {
                Player::X => Player::O,
                Player::O => Player::X,
            }
        }

        pub fn to_char(&self) -> char {
            match *self {
                Player::X => 'X',
                Player::O => 'O',
            }
        }
    }

    pub struct Table {
        t: [[char; 3]; 3],
    }

    impl Table {
        pub fn new() -> Table {
            let mut table = [[' '; 3]; 3];
            let mut index = 1;

            for row in &mut table {
                for col in row {
                    *col = std::char::from_digit(index, 10).unwrap();
                    index += 1
                }
            }

            Table { t: table }
        }

        pub fn display(&self) {
            for row in self.t {
                for j in 0..3 {
                    if j == 2 {
                        println!(" {} ", row[j]);
                    } else {
                        print!(" {} |", row[j])
                    }
                }
            }
        }

        pub fn is_taken(&self, pos: (usize, usize)) -> bool {
            match self.t[pos.0][pos.1] {
                'X' => true,
                'O' => true,
                _ => false,
            }
        }

        pub fn update(&mut self, pos: (usize, usize), player: &Player) {
            self.t[pos.0][pos.1] = player.to_char();
        }

        pub fn is_tie(&self) -> bool {
            for row in self.t {
                for col in row {
                    if col != 'X' || col != 'O' {
                        return true;
                    }
                }
            }
            false
        }

        pub fn is_win(&self, player: &Player) -> bool {
            for row in self.t {
                if row.iter().all(|p| *p == player.to_char()) {
                    return true;
                }
            }

            for col in 0..3 {
                if (0..3).all(|row| self.t[row][col] == player.to_char()) {
                    return true;
                }
            }

            if (0..3).all(|i| self.t[i][i] == player.to_char())
                || (0..3).all(|i| self.t[i][2 - i] == player.to_char())
            {
                return true;
            }

            false
        }
    }
}
