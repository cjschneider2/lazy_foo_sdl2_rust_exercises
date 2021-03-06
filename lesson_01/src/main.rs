//! Hello SDL : The first graphics window

extern crate sdl2;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() {

    let sdl_ctx = sdl2::init().unwrap();
    let vid_ctx = sdl_ctx.video().unwrap();
    let window = vid_ctx.window("Hello SDL", SCREEN_WIDTH, SCREEN_HEIGHT)
        .build().unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas()
        .present_vsync()
        .build().unwrap();

    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255,210,0));
    canvas.fill_rect(Rect::new(10,10,40,40)).unwrap();

    canvas.present();

    std::thread::sleep(std::time::Duration::new(2,0));
}
