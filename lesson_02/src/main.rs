//! Hello SDL : The first graphics window

extern crate sdl2;

mod error;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::image::LoadTexture;
use sdl2::image::{INIT_JPG, INIT_PNG};

use error::Error;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

struct SDLState {
   pub context: sdl2::Sdl,
   pub video: sdl2::VideoSubsystem,
   pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
   pub image: sdl2::image::Sdl2ImageContext,
}

impl SDLState {
   fn new() -> Result<SDLState, Error> {
      let context = sdl2::init()?;
      let video = context.video()?;
      let window = video.window("Hello SDL", SCREEN_WIDTH, SCREEN_HEIGHT)
                        .build()?;
      let canvas: Canvas<Window> = window.into_canvas()
                                         .present_vsync()
                                         .build()?;
      let image = sdl2::image::init(INIT_PNG | INIT_JPG)?;
      let state = SDLState {
          context: context,
          video: video,
          canvas: canvas,
          image: image,
      };
      Ok(state)
   }
}

fn main() {

   let mut sdl = match SDLState::new() {
      Ok(state) => state,
      Err(e) => panic!(e),
   };

   let tex = sdl.canvas.texture_creator();

   sdl.canvas.set_draw_color(Color::RGB(0,0,0));
   sdl.canvas.clear();

   let img_path = std::path::Path::new("./resources/preview.png");
   let img = match tex.load_texture(img_path) {
      Ok(t) => t,
      Err(e) => panic!(e),
   };

   sdl.canvas.set_draw_color(Color::RGB(255,210,0));
   sdl.canvas.fill_rect(Rect::new(10,10,40,40)).unwrap();

   sdl.canvas.copy(&img, None, Some(Rect::new(50,50,100,100))).unwrap();

   sdl.canvas.present();

   std::thread::sleep(std::time::Duration::new(1,0));
}
