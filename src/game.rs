
extern crate bmp;

use std::{collections::HashSet, hash::Hash, path::Path};
use rand::Rng;

const DIRECTIONS: [(i32, i32); 8] = [
  (-1,-1), (-1, 0), (-1, 1),
  (0, -1),          (0,  1),
  (1, -1), (1,  0), (1,  1),
];

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point2D {
  x: i32,
  y: i32
}

impl Point2D {
  pub fn new(x: i32, y: i32) -> Self {
    return Point2D {x, y};
  }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Cell {
  location: Point2D,
  state: bool
}

impl Cell {
  pub fn new(location: Point2D, alive: bool) -> Self {
    return Cell {
      location: location,
      state: alive
    }
  }

  pub fn new_alive(location: Point2D) -> Self {
    return Cell {
      location: location,
      state: true
    }
  }

  pub fn new_dead(location: Point2D) -> Self {
    return Cell {
      location: location,
      state: false
    }
  }

  pub fn is_alive(&self) -> bool {
    return self.state;
  }

  pub fn set_state(&mut self, state: bool) {
    self.state = state;
  }
}

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
      
      let cell = self.board.get(i, j);
      let live_neighbours = self.count_neighbours(i as i32, j as i32);

      if cell.is_alive() {
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

      if self.board.get(nx as usize, ny as usize).is_alive() {
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
        if board.get(i, j).is_alive() {
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
  data: Vec<Cell>
}

impl Board {
  pub fn new(width: usize, height: usize, default: bool) -> Self {
    let mut cells: Vec<Cell> = Vec::new();

    let mut i = 0;
    while i < height {
      let mut j = 0;
      
      while j < width {
        cells.push(Cell {
          location: Point2D { x: j as i32, y: i as i32 },
          state: default
        });
        
        j += 1;
      }

      i += 1;
    }

    return Board {
      width,
      height,
      data: cells
    };
  }

  pub fn random(width: usize, height: usize, prob: f64) -> Self {
    let mut cells: Vec<Cell> = Vec::new();
    let mut rng = rand::thread_rng();

    let mut i = 0;
    while i < height {
      let mut j = 0;
      
      while j < width {
        let rand_float = rng.gen::<f64>();
        let is_alive = rand_float < prob;
        
        cells.push(Cell {
          location: Point2D { x: j as i32, y: i as i32},
          state: is_alive
        });
        
        j += 1;
      }

      i += 1;
    }

    return Board {
      width,
      height,
      data: cells
    };
  }

  pub fn from_bmp(file_path: &Path) -> Self {
    let scenario = bmp::open(file_path).expect("Could not load bitmap");

    let width  = scenario.get_width() as usize;
    let height = scenario.get_height() as usize;
    
    let mut cells: Vec<Cell> = Vec::new();

    let mut i = 0;
    while i < scenario.get_height() {
      let mut j = 0;
      
      while j < scenario.get_width() {
        let pixel = scenario.get_pixel(j, i);
        
        if pixel.r == 0 && pixel.g == 0 && pixel.b == 0 {
          cells.push(
            Cell::new_alive(Point2D { x: j as i32, y: i as i32 })
          );
        } else {
          cells.push(
            Cell::new_dead(Point2D { x: j as i32, y: i as i32 })
          );
        }
        
        j += 1;
      }

      i += 1;
    }

    return Board {
      width,
      height,
      data: cells
    }
  }

  pub fn get_height(&self) -> usize {
    return self.height;
  }

  pub fn get_width(&self) -> usize {
    return self.width;
  }

  pub fn get(&self, x: usize, y: usize) -> Cell {
    return self.data[self.width * y + x];
  }

  pub fn set(&mut self, x: usize, y: usize, state: bool) {
    self.data[self.width * y + x].set_state(state);
  }
}

mod test {
    use crate::game::{Cell, Point2D};

  #[test]
  fn the_worst_test_code_ever() {
    println!("Point: {:?}", Point2D::new(0, 0));
    assert_eq!(Point2D::new(0, 0), Point2D::new(0, 0));
    assert_ne!(Point2D::new(10, 0), Point2D::new(230, 0));

    let center_alive = Cell::new_alive(Point2D::new(0, 0));

    println!("Cell: {:?}", center_alive);
    assert_eq!(center_alive, Cell::new_alive(Point2D::new(0, 0)));
    assert_ne!(center_alive, Cell::new_dead(Point2D::new(0, 0)));
    assert_ne!(center_alive, Cell::new_alive(Point2D::new(10, 0)));
  }
}