use crate::cursor::*;
use grid::*;
use itertools::Itertools;
use rand::prelude::*;
use std::convert::TryInto;
use std::io;

pub struct Tile {
  pub is_bomb: bool,
  pub been_revealed: bool,
  pub bombs_around: u8,
  pub display: String,
  pub name: usize,
}

#[derive(PartialEq)]
pub struct GameData {
  pub revealed_tiles: usize,
  pub tiles_marked: isize,
  pub revealed_tiles_to_win: usize,
}

pub struct GameConfig {
  pub grid_size: usize,
  pub bomb_count: isize,
  pub tiles: Grid<Tile>,
  pub game_data: GameData,
  pub cursor_data: Cursor,
}

pub enum GameActions {
  IncorrectInput,
  GameOver,
  Win,
  TileAlreadySelected,
  TileAlreadyMarked,
  SuccessfulRun,
}

impl GameData {
  pub fn win_conditions_met(&mut self) -> bool {
    if self.revealed_tiles == self.revealed_tiles_to_win {
      true
    } else {
      false
    }
  }
}

pub fn reveal_all_tiles(game_config: &mut GameConfig) {
  for coord_1 in 0..game_config.grid_size {
    for coord_2 in 0..game_config.grid_size {
      let selected_tile = game_config.tiles.get_mut(coord_1, coord_2).unwrap();

      if selected_tile.is_bomb {
        selected_tile.display = String::from("ó");
      } else {
        selected_tile.display = selected_tile.bombs_around.to_string();
      }
    }
  }
}

pub fn create_game_config() -> GameConfig {
  let (grid_size, bomb_count) = grid_data_input();
  let tiles = tile_creation(&grid_size, &bomb_count);

  let game_data = GameData {
    revealed_tiles: 0,
    tiles_marked: 0,
    revealed_tiles_to_win: grid_size.pow(2) - bomb_count,
  };

  let cursor = Cursor {
    currently_moving: Moves::Stay,
    position: [0, 0],
    action: CursorActions::Move,
  };

  let bomb_count = bomb_count.try_into().unwrap();

  GameConfig {
    grid_size,
    bomb_count,
    tiles,
    game_data,
    cursor_data: cursor,
  }
}

pub fn coords_around_input_coords(
  coords: &mut Vec<Vec<usize>>,
  grid_size: usize,
) -> Vec<Vec<usize>> {
  let mut coordinates_around = Vec::new();
  let grid_size: isize = grid_size.try_into().unwrap();

  for coord in coords {
    let nums: Vec<isize> = vec![coord[0].try_into().unwrap(), coord[1].try_into().unwrap()];
    let nums: Vec<Vec<isize>> = vec![
      vec![nums[0], nums[1] - 1],
      vec![nums[0], nums[1] + 1],
      vec![nums[0] - 1, nums[1]],
      vec![nums[0] + 1, nums[1]],
      vec![nums[0] - 1, nums[1] + 1],
      vec![nums[0] + 1, nums[1] - 1],
      vec![nums[0] - 1, nums[1] - 1],
      vec![nums[0] + 1, nums[1] + 1],
    ];

    for coordinates in nums {
      match coordinates[0] {
        -1 => continue,
        coord if coord == grid_size => continue,
        _ => (),
      }

      match coordinates[1] {
        -1 => continue,
        coord if coord == grid_size => continue,
        _ => (),
      }

      let swap_to_usize: Vec<usize> = vec![
        coordinates[0].try_into().unwrap(),
        coordinates[1].try_into().unwrap(),
      ];

      coordinates_around.push(swap_to_usize);
    }
  }

  coordinates_around
}

pub fn tile_names_to_coords(names: Vec<usize>, grid_size: usize) -> Vec<Vec<usize>> {
  let mut new_coords = Vec::new();

  for name in names {
    new_coords.push(vec![name / grid_size, name % grid_size]);
  }

  new_coords
}

pub fn reveal_tiles_return_zeros(
  coords: &mut Vec<Vec<usize>>,
  game_config: &mut GameConfig,
) -> Vec<Vec<usize>> {
  let mut new_zeros_vec = Vec::new();

  for coord in coords {
    let selected_coords = game_config.tiles.get_mut(coord[0], coord[1]).unwrap();

    if selected_coords.bombs_around == 0 {
      selected_coords.display = selected_coords.bombs_around.to_string();
      selected_coords.been_revealed = true;
      game_config.game_data.revealed_tiles += 1;

      new_zeros_vec.push(vec![coord[0], coord[1]]);
    } else {
      selected_coords.display = selected_coords.bombs_around.to_string();
      selected_coords.been_revealed = true;
      game_config.game_data.revealed_tiles += 1;
    }
  }

  new_zeros_vec
}

pub fn tile_creation(grid_size: &usize, bomb_count: &usize) -> Grid<Tile> {
  let mut tiles: Vec<Tile> = Vec::new();
  let mut bomb_names = Vec::new();

  for counter in 0..grid_size * grid_size {
    let new_tile = Tile {
      is_bomb: false,
      been_revealed: false,
      bombs_around: 0,
      display: String::from("▮"),
      name: counter,
    };

    tiles.push(new_tile);
  }

  loop {
    for _x in 0..bomb_count * 2 {
      let new_bomb_name = rand::thread_rng().gen_range(0..((grid_size * grid_size) - 1));

      bomb_names.push(new_bomb_name);
    }

    bomb_names = bomb_names.into_iter().unique().collect::<Vec<usize>>();

    if bomb_names.len() >= *bomb_count {
      bomb_names.truncate(*bomb_count);

      break;
    }
  }

  let mut bomb_coordinates = tile_names_to_coords(bomb_names, *grid_size);
  let mut grid_set = Grid::from_vec(tiles, *grid_size);
  let coords_around_bombs = coords_around_input_coords(&mut bomb_coordinates, *grid_size);

  for bomb_coord in bomb_coordinates {
    grid_set
      .get_mut(bomb_coord[0], bomb_coord[1])
      .unwrap()
      .is_bomb = true;
  }

  for coord in &coords_around_bombs {
    grid_set.get_mut(coord[0], coord[1]).unwrap().bombs_around += 1;
  }

  grid_set
}

pub fn print_grid(game_config: &GameConfig) {
  println!("");

  for coord_1 in 0..game_config.grid_size {
    for coord_2 in 0..game_config.grid_size {
      if game_config.cursor_data.position == [coord_1, coord_2] {
        print!(
          ">{}",
          game_config.tiles.get(coord_1, coord_2).unwrap().display
        );
      } else {
        print!(
          "|{}",
          game_config.tiles.get(coord_1, coord_2).unwrap().display
        );
      }
    }

    println!("");
  }
}

pub fn grid_data_input() -> (usize, usize) {
  let mut grid_size = String::new();
  let mut bomb_count = String::new();

  println!("Input the grid size, example - '10' is a 10 x 10 grid");
  io::stdin().read_line(&mut grid_size).unwrap();

  println!("Input the bomb count");
  io::stdin().read_line(&mut bomb_count).unwrap();

  let grid_size: usize = grid_size.trim().parse().unwrap();
  let bomb_count: usize = bomb_count.trim().parse().unwrap();

  if bomb_count >= grid_size * grid_size {
    panic!("bomb count > tile count");
  }

  (grid_size, bomb_count)
}
