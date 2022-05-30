use crate::cursor::*;
use crate::gamedata::*;

pub fn print_instructions() {
  println!(
    "
Your currently selected tile will be marked with the cursor. | > |
To move, type out the amount you want to move up, down, left, and/or right.
Example for moving up 10 tiles. - u10

It's also possible to use a vertical and horizontal movement at the same time.
Example of moving vertically and horizontally. - u10 r5

If you wish to mark/unmark a tile, type | ! | and it'll mark/unmark the current one you're on.
If you wish to select a tile, type | y | and it'll select the tile your cursor is on.

You can also do these in sequence.
Example of moving and choosing the new tile. - u0 r5 y
Example of moving and marking the new tile. - u0 r5 !
"
  );
}

pub fn run_gameplay() {
  let mut game_config = create_game_config();

  print_instructions();

  loop {
    println!(
      "
----------------------------------------
----------------------------------------
----------------------------------------
    "
    );
    println!(
      "{} bombs left",
      game_config.bomb_count - game_config.game_data.tiles_marked
    );

    print_grid(&game_config);

    match gameplay(&mut game_config) {
      GameActions::IncorrectInput => println!("incorrect input"),
      GameActions::TileAlreadySelected => println!("tile already selected"),
      GameActions::TileAlreadyMarked => println!("tile is marked"),
      GameActions::SuccessfulRun => println!(),
      GameActions::Win => {
        reveal_all_tiles(&mut game_config);
        print_grid(&game_config);

        println!("\nBoard Cleared!");

        return;
      }
      GameActions::GameOver => {
        reveal_all_tiles(&mut game_config);
        print_grid(&game_config);

        println!("\nGame Over");

        return;
      }
    }

    game_config.cursor_data.reset_cursor_non_position();
  }
}

pub fn gameplay(game_config: &mut GameConfig) -> GameActions {
  if game_config.game_data.win_conditions_met() {
    return GameActions::Win;
  }

  game_config.cursor_data.cursor_input(game_config.grid_size);

  let coords = game_config.cursor_data.position;
  let tile_at_cursor = game_config.tiles.get_mut(coords[0], coords[1]).unwrap();

  match game_config.cursor_data.action {
    CursorActions::FailedInput => {
      return GameActions::IncorrectInput;
    }

    CursorActions::Mark => {
      if tile_at_cursor.display == "!" {
        if tile_at_cursor.been_revealed {
          tile_at_cursor.display = tile_at_cursor.bombs_around.to_string();
        } else {
          tile_at_cursor.display = "▮".to_string();
        }

        game_config.game_data.tiles_marked -= 1;
      } else {
        tile_at_cursor.display = "!".to_string();

        game_config.game_data.tiles_marked += 1;
      }

      return GameActions::SuccessfulRun;
    }

    CursorActions::Select => {
      if tile_at_cursor.display != *"▮".to_string() {
        if tile_at_cursor.display == *"!".to_string() {
          return GameActions::TileAlreadyMarked;
        } else {
          return GameActions::TileAlreadySelected;
        }
      } else if tile_at_cursor.is_bomb {
        return GameActions::GameOver;
      }

      if tile_at_cursor.bombs_around == 0 {
        reveal_all_zeros_around_cursor_position(game_config);
      } else {
        tile_at_cursor.display = tile_at_cursor.bombs_around.to_string();
        tile_at_cursor.been_revealed = true;
        game_config.game_data.revealed_tiles += 1;
      }
    }

    _ => (),
  }

  GameActions::SuccessfulRun
}
