extern crate sdl2;

extern crate bmp;

use std::path::Path;
use std::time::Instant;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use sdl2::ttf::Font;

mod game;
use game::GameOfLife;
use game::Board;

struct Options {
  window_width: u32,
  window_height: u32,
  window_title: &'static str,
  cell_size: u32
}

const OPTIONS: Options = Options {
  window_width:  1000,
  window_height: 1000,
  window_title: "Rusty GOL",
  cell_size: 1
};


fn init(sdl_context : &Sdl) -> Canvas<Window> {
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem.window(&OPTIONS.window_title, OPTIONS.window_width, OPTIONS.window_height)
      .position_centered()
      .build()
      .unwrap();

  return window.into_canvas().build().unwrap();
}

pub fn plot() {
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = init(&sdl_context);
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).expect("Error Loading ttf context!");

    let font = ttf_context.load_font(Path::new("D-DIN.ttf"), 128).expect("Error Loading font!");

    //let initial_board = Board::from_bmp(Path::new("scenarios/blinker.bmp"));
    let board_width = (OPTIONS.window_width / OPTIONS.cell_size) as usize;
    let board_height = (OPTIONS.window_height / OPTIONS.cell_size) as usize;
    
    let initial_board = Board::random(board_width, board_height, 0.3);
    let mut game = GameOfLife::new(initial_board);
    
    game.pause();

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'game_loop: loop {
      let frame_start_time = Instant::now();
      handle_events(&mut event_pump, &mut game);
      
      if game.is_finished() {
        break 'game_loop;
      }
      
      render(&mut canvas, &mut game, frame_start_time, &font, &texture_creator);
    }
}

fn render(canvas: &mut Canvas<Window>, game: &mut GameOfLife, frame_start_time: Instant, font: &Font, texture_creator: &TextureCreator<WindowContext>) {
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    
    let cells_to_analyze = game.get_analyzed_cell_count();

    if !game.is_paused() {
      game.update();
    }
    canvas.set_draw_color(Color::BLACK);
    
    let mut i = 0;
    while i < game.get_board().get_width() {
      let mut j = 0;
      while j < game.get_board().get_height() {
        if game.get_board().get(i, j) {
          canvas.fill_rect(Rect::new((i as u32 * OPTIONS.cell_size) as i32, (j as u32 * OPTIONS.cell_size) as i32, OPTIONS.cell_size, OPTIONS.cell_size)).expect("Could not Draw cell");
        }
    
        j += 1;
      }
      i += 1;
    }
    let frame_duration = frame_start_time.elapsed();
    let frame_duration_millis = frame_duration.as_millis();
    let mut fps = 0;

    if frame_duration_millis > 0 {
      fps = 1000 / frame_duration_millis;
    }

    let hud_text = format!("{}x{} Gen:{} FT: {:?} FPS: {} Processed Cells: {}", game.get_board().get_width(), game.get_board().get_height(), game.get_genereation(), frame_duration, fps, cells_to_analyze);
    
    render_hud(&font, hud_text, &texture_creator, canvas);
    
    canvas.present();
}

fn handle_events(event_pump: &mut EventPump, game: &mut GameOfLife) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                game.finish();
            },
            Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
              game.reset();
            }
            Event::KeyUp { keycode: Some(Keycode::N), .. } => {
              game.update();
            }
            Event::KeyUp { keycode: Some(Keycode::P), .. } => {
              game.toggle_pause();
            }
            _ => {}
        }
    }
}

fn render_hud(font: &Font, hud_text: String, texture_creator: &TextureCreator<WindowContext>, canvas: &mut Canvas<Window>) {
    let text = font.render(hud_text.as_str())
          .solid(Color::MAGENTA)
          .expect("Error creating font surface!");
    let texture = texture_creator.create_texture_from_surface(&text).expect("Error creating text texture");
    canvas.copy(&texture, None, Rect::new(0, 0, 600,  50)).unwrap();
}

fn main() {
  plot();
}