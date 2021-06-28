use engine::Engine;
use engine::Options;
use game::Board;
use game::GameOfLife;

mod engine;
mod game;


const OPTIONS: Options = Options {
  window_width:  1000,
  window_height: 1000,
  window_title: "Rusty GOL",
  cell_size: 5
};

fn main() {
  let board_width = (OPTIONS.window_width / OPTIONS.cell_size) as usize;
  let board_height = (OPTIONS.window_height / OPTIONS.cell_size) as usize;
  
  //let initial_board = Board::from_bmp(Path::new("scenarios/blinker.bmp"));
  let initial_board = Board::random(board_width, board_height, 0.3);
  let mut game = GameOfLife::new(initial_board);
  
  Engine::new(OPTIONS, &mut game).run();
}