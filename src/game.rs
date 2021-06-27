
extern crate bmp;

use std::{collections::HashSet, path::Path};
use rand::Rng;

const DIRECTIONS: [(i32, i32); 8] = [
  (-1,-1), (-1, 0), (-1, 1),
  (0, -1),          (0,  1),
  (1, -1), (1,  0), (1,  1),
];

pub struct GameOfLife {
  board: Board,
  paused: bool,
  finished: bool,
  generation: u32,
  cells_to_analyze: HashSet<(i32, i32)>
}


impl GameOfLife {
  pub fn new(board: Board) -> Self {
    let mut cells_to_analyze: HashSet<(i32, i32)> = HashSet::new();

    initial_analysis(&board, &mut cells_to_analyze);

    return GameOfLife {
      board: board,
      paused: false,
      finished: false,
      generation: 0,
      cells_to_analyze: cells_to_analyze
    }
  }

  pub fn update(&mut self) {
    let width = self.board.get_width();
    let height = self.board.get_height();
    
    let mut new_board = Board::new(width, height, false);
    let mut new_cells_to_analyze: HashSet< (i32, i32) > = HashSet::new();

    for cell in &self.cells_to_analyze {
      let mut ni = cell.0;
      let mut nj = cell.1;

      if ni <= 0 {
        ni = 0;
      }

      if nj <= 0 {
        nj = 0;
      }

      let i = ni as usize % self.board.get_width();
      let j = nj as usize % self.board.get_height();
      
      let cell_alive = self.board.get(i, j);
      let live_neighbours = self.count_neighbours(i as i32, j as i32);

      if cell_alive {
        if live_neighbours < 2 || live_neighbours > 3 {
          new_board.set(i, j, false);
        } else {
          new_board.set(i, j, true);
          insert_me_and_neighbours(&mut new_cells_to_analyze, i, j);
        }
      } else {
        if live_neighbours == 3 {
          new_board.set(i, j, true);
          insert_me_and_neighbours(&mut new_cells_to_analyze, i, j);
        }
      }
    }

    self.generation += 1;
    self.board = new_board;
    self.cells_to_analyze = new_cells_to_analyze;
  }

  fn count_neighbours(&self, x: i32, y: i32) -> u32 {
    let mut neighbours = 0;
    
    let width = self.board.get_width() as u32;
    let height = self.board.get_height() as u32;
    
    for direction in DIRECTIONS {

      let nx = (x + direction.0) as u32 % width;
      let ny = (y + direction.1) as u32 % height;

      if self.board.data[nx as usize][ny as usize] {
        neighbours += 1;
      }
    }

    return neighbours;
  }

  pub fn reset(&mut self) {
    let new_board = Board::random(self.board.width, self.board.height, 0.5);
    let mut new_cells_to_analyze: HashSet<(i32, i32)> = HashSet::new();
    
    initial_analysis(&new_board, &mut new_cells_to_analyze);
    
    self.board = new_board;
    self.cells_to_analyze = new_cells_to_analyze;
    self.generation = 0;
  }

  pub fn get_board(&self) -> &Board {
    return &self.board;
  }

  pub fn is_paused(&self) -> bool {
    return self.paused;
  }

  pub fn pause(&mut self) {
    self.paused = true;
  }

  pub fn unpause(&mut self) {
    self.paused = false;
  }

  pub fn toggle_pause(&mut self) {
    self.paused = !self.paused;
  }

  pub fn get_genereation(&self) -> u32 {
    return self.generation;
  }

  pub fn finish(&mut self) {
    self.finished = true;
  }

  pub fn is_finished(&self) -> bool {
    return self.finished;
  }

  pub fn get_analyzed_cell_count(&self) -> usize {
    return self.cells_to_analyze.len();
  }
}

fn initial_analysis(board: &Board, cells_to_analyze: &mut HashSet<(i32, i32)>) {
    let mut i = 0;
    while i < board.get_width() {
      let mut j = 0;
      while j < board.get_height() {
        if board.get(i, j) {
          insert_me_and_neighbours(cells_to_analyze, i, j);
        }
        j += 1;
      }
      i += 1;
    }
}

fn insert_me_and_neighbours(cells_to_analyze: &mut HashSet<(i32, i32)>, i: usize, j: usize) {
    cells_to_analyze.insert( (i as i32, j as i32) );
    
    for dir in DIRECTIONS {
      let nx = i as i32 + dir.0;
      let nj = j as i32 + dir.1;

      cells_to_analyze.insert( (nx, nj) );
    }
}

pub struct Board {
  width: usize,
  height: usize,
  data: Vec<Vec<bool>>
}

impl Board {
  pub fn new(width: usize, height: usize, default: bool) -> Self {
    let vec = vec![vec![default; width]; height];
    return Board {
      width,
      height,
      data: vec
    };
  }

  pub fn random(width: usize, height: usize, prob: f64) -> Self {
    let mut vec = vec![vec![false; width]; height];

    let mut rng = rand::thread_rng();

    let mut i = 0;
    while i < width {
      let mut j = 0;
      while j < height {
        let rand_float = rng.gen::<f64>();
        if rand_float < prob {
          vec[i][j] = true;
        }
        j += 1;
      }
      i += 1;
    }

    return Board {
      width,
      height,
      data: vec
    };
  }

  pub fn from_bmp(file_path: &Path) -> Self {
    let scenario = bmp::open(file_path).expect("Could not load bitmap");

    let width  = scenario.get_width() as usize;
    let height = scenario.get_height() as usize;
    
    let mut vec = vec![vec![false; width]; height];

    let mut i = 0;
    while i < scenario.get_width() {
      let mut j = 0;
      while j < scenario.get_height() {
        let pixel = scenario.get_pixel(i, j);
        if pixel.r == 0 && pixel.g == 0 && pixel.b == 0 {
          vec[i as usize][j as usize] = true;
        }
        
        j += 1;
      }
      i += 1;
    }

    return Board {
      width,
      height,
      data: vec
    }
  }

  pub fn get_height(&self) -> usize {
    return self.height;
  }

  pub fn get_width(&self) -> usize {
    return self.width;
  }

  pub fn get(&self, x: usize, y: usize) -> bool {
    return self.data[x][y];
  }

  pub fn set(&mut self, x: usize, y: usize, value: bool) {
    self.data[x][y] = value;
  }
}

