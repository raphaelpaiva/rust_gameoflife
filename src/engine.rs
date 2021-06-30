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
use sdl2::ttf::{Font};

use crate::game::{GameOfLife};

pub struct Options {
  pub window_width: u32,
  pub window_height: u32,
  pub window_title: &'static str,
  pub cell_size: u32
}

pub struct Engine<'a> {
  options: Options,
  sdl_context: Sdl,
  canvas: Canvas<Window>,
  game: &'a mut GameOfLife
}

impl<'a> Engine<'a> {
  pub fn new(options: Options, game: &'a mut GameOfLife) -> Self {
    let sdl_context = sdl2::init().map_err(|e| e.to_string()).expect("Error initializing SDL");
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(options.window_title, options.window_width, options.window_height)
        .position_centered()
        .build()
        .unwrap();
    
    let canvas = window.into_canvas().build().unwrap();

    return Engine {
      options: options,
      sdl_context: sdl_context,
      canvas: canvas,
      game: game
    }
  }

  pub fn run(&mut self) {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).expect("Error Loading ttf context!");
    let font = ttf_context.load_font(Path::new("D-DIN.ttf"), 128).expect("Error Loading font");

    self.canvas.set_draw_color(Color::WHITE);
    self.canvas.clear();
    
    'game_loop: loop {
      let cells_to_analyze = self.game.get_analyzed_cell_count();
      let frame_start_time = Instant::now();
      
      self.handle_events();
      
      if self.game.is_finished() {
        break 'game_loop;
      }
            
      self.render();
      
      let frame_duration = frame_start_time.elapsed();
      let frame_duration_millis = frame_duration.as_millis();
      let mut fps = 0;

      if frame_duration_millis > 0 {
        fps = 1000 / frame_duration_millis;
      }

      let hud_text = format!(
        "{}x{} Gen:{} FT: {:?} FPS: {} Processed Cells: {}",
        self.game.get_board().get_width(),
        self.game.get_board().get_height(),
        self.game.get_genereation(),
        frame_duration,
        fps,
        cells_to_analyze
      );
      
      //println!("{}", hud_text);
      self.render_hud(&font, hud_text);
      self.canvas.present();
    }
  }

  pub fn render(&mut self) {
    self.canvas.set_draw_color(Color::WHITE);
    self.canvas.clear();

    if !self.game.is_paused() {
      self.game.update();
    }
    self.canvas.set_draw_color(Color::BLACK);
    
    let mut i = 0;
    while i < self.game.get_board().get_width() {
      let mut j = 0;
      while j < self.game.get_board().get_height() {
        if self.game.get_board().get(i, j).is_alive() {
          self.canvas.fill_rect(Rect::new((i as u32 * self.options.cell_size) as i32, (j as u32 * self.options.cell_size) as i32, self.options.cell_size, self.options.cell_size)).expect("Could not Draw cell");
        }
    
        j += 1;
      }
      i += 1;
    }
  }

  fn handle_events(&mut self) {
    let mut event_pump = self.sdl_context.event_pump().unwrap();
    
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.game.finish();
            },
            Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
              self.game.reset();
            }
            Event::KeyUp { keycode: Some(Keycode::N), .. } => {
              self.game.update();
            }
            Event::KeyUp { keycode: Some(Keycode::P), .. } => {
              self.game.toggle_pause();
            }
            _ => {}
        }
    }
}

  fn render_hud(&mut self, font: &Font, hud_text: String) {
    let texture_creator = self.canvas.texture_creator();
    
    let text = font.render(hud_text.as_str())
          .solid(Color::MAGENTA)
          .expect("Error creating font surface!");
    
    let texture = texture_creator.create_texture_from_surface(&text).expect("Error creating text texture");
    
    self.canvas.copy(&texture, None, Rect::new(0, 0, 600,  50)).unwrap();
  }
}


