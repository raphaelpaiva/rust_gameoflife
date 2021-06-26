extern crate bmp;

use std::path::Path;
use rand::Rng;

#[derive(Debug)]
pub struct Board {
  width: usize,
  height: usize,
  data: Vec<Vec<bool>>
}

impl Board {
  pub fn new(width: usize, height: usize, default: bool) -> Board {
    let vec = vec![vec![default; width]; height];
    return Board {
      width,
      height,
      data: vec
    };
  }

  pub fn random(width: usize, height: usize, prob: f64) -> Board {
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

  pub fn from_bmp(file_path: &Path) -> Board {
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

