use std::io;

pub struct Cursor {
  pub currently_moving: Moves,
  pub position: [usize; 2],
  pub action: CursorActions,
}

#[derive(PartialEq)]
pub enum CursorActions {
  Select,
  Mark,
  FailedInput,
  Move,
}

#[derive(PartialEq)]
pub enum Moves {
  Up,
  Down,
  Left,
  Right,
  Stay,
}

impl Cursor {
  pub fn reset_cursor_non_position(&mut self) {
    self.currently_moving = Moves::Stay;
    self.action = CursorActions::Move;
  }

  pub fn move_cursor(&mut self, move_amount: usize, grid_size: usize) {
    match self.currently_moving {
      Moves::Down => {
        if move_amount + self.position[0] > grid_size - 1 {
          self.position[0] = grid_size - 1;
        } else {
          self.position[0] += move_amount;
        }
      }
      Moves::Up => {
        if move_amount > self.position[0] {
          self.position[0] = 0;
        } else {
          self.position[0] -= move_amount;
        }
      }
      Moves::Right => {
        if move_amount + self.position[1] > grid_size - 1 {
          self.position[1] = grid_size - 1;
        } else {
          self.position[1] += move_amount;
        }
      }
      Moves::Left => {
        if move_amount > self.position[1] {
          self.position[1] = 0;
        } else {
          self.position[1] -= move_amount;
        }
      }
      _ => (),
    }
  }

  pub fn match_cursor_input(&mut self, user_input: &str) {
    match &user_input[0..1] {
      "u" => self.currently_moving = Moves::Up,
      "U" => self.currently_moving = Moves::Up,
      "d" => self.currently_moving = Moves::Down,
      "D" => self.currently_moving = Moves::Down,
      "l" => self.currently_moving = Moves::Left,
      "L" => self.currently_moving = Moves::Left,
      "r" => self.currently_moving = Moves::Right,
      "R" => self.currently_moving = Moves::Right,
      "y" => self.action = CursorActions::Select,
      "Y" => self.action = CursorActions::Select,
      "!" => self.action = CursorActions::Mark,
      _ => self.action = CursorActions::FailedInput,
    }
  }

  pub fn cursor_input(&mut self, grid_size: usize) {
    let mut user_input = String::new();

    println!("Choose an action");
    io::stdin().read_line(&mut user_input).unwrap();

    let mut user_input = user_input.split_whitespace();

    while let Some(action) = user_input.next() {
      self.match_cursor_input(action);

      match self.action {
        CursorActions::Select => continue,
        CursorActions::Mark => continue,
        CursorActions::FailedInput => break,
        _ => (),
      }

      let move_amount: usize = match action[1..action.len()].parse() {
        Ok(x) => x,
        Err(_) => {
          self.action = CursorActions::FailedInput;
          break;
        }
      };

      self.move_cursor(move_amount, grid_size);
    }
  }
}
