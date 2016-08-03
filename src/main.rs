extern crate sdl2;
extern crate rand;

mod maze;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use maze::Direction;
use maze::GameState;

pub fn main() {
    let mut map = make_new_game();
    let width = 10*map.cols;
    let height = 10*map.rows;
    
    println!("width: {}, height: {}", width, height);

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
                Event::KeyDown { keycode: Some(kc), .. } => {
                    let dir = match kc {
                        Keycode::Left => Direction::Left,
                        Keycode::Right => Direction::Right,
                        Keycode::Up => Direction::Up,
                        Keycode::Down => Direction::Down,
                        _ => {
                            println!("Invalid key");
                            continue;
                        },
                    };

                    match map.move_player(dir) {
                        GameState::Won => {
                            println!("You win!");
                            map = make_new_game();
                        },
                        GameState::Dead => {
                            println!("You were killed. RIP :(");
                            map = make_new_game();
                        }
                        _ => continue,
                    };
                }
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

fn make_new_game() -> maze::Maze {
    let mut map = maze::Maze::from_file("./res/map.txt").unwrap();
    map.add_player();
    map.add_trolls(3);

    map
}