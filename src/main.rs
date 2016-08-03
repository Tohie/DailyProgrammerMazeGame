extern crate sdl2;
extern crate rand;

mod maze;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use maze::Direction;

pub fn main() {
    let mut map = maze::Maze::from_file("./res/map.txt").unwrap();
    let width = 10*map.cols;
    let height = 10*map.rows;
    
    println!("width: {}, height: {}", width, height);
    
    map.add_player();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 maze", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    renderer.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => map.move_player(Direction::Left),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => map.move_player(Direction::Right),
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => map.move_player(Direction::Up),
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => map.move_player(Direction::Down),
                _ => {}
            }
        }

        renderer.clear();
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.fill_rect(Rect::new(0, 0, height, width)).unwrap();
        map.render(&mut renderer);
        renderer.present();
    }
}