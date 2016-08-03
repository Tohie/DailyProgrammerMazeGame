extern crate sdl2;
extern crate rand;

use std::fs::File;
use std::io::{BufReader, Error, BufRead, ErrorKind};
use sdl2::render::Renderer;
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use rand::distributions::{IndependentSample, Range};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

fn get_dxdy(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Up => (0, -1),
        Direction::Down => (0, 1), 
    }
}

pub enum Piece {
    Empty,
    Player(Direction),
    Boulder,
    Exit,
}

pub enum GameState {
    Won,
    InvalidMove,
    Moved,
}

fn render_player(renderer: &mut Renderer<'static>, dir: &Direction, x: i32, y: i32, width: i32, height: i32) {
    let points = match dir {
        &Direction::Left => vec!(Point::new(x+(width/2), y+height), Point::new(x, y+(height/2)), Point::new(x+(width/2), y)),
        &Direction::Right => vec!(Point::new(x+(width/2), y+height), Point::new(x+width, y+(height/2)), Point::new(x+(width/2), y)),
        &Direction::Up => vec!(Point::new(x, y+(height/2)), Point::new(x+(width/2), y), Point::new(x+width, (y+height/2))),
        &Direction::Down => vec!(Point::new(x, y+(height/2)), Point::new(x+(width/2), y+height), Point::new(x+width, (y+height/2))),
    };

    renderer.draw_lines(&points).unwrap();
}

pub struct Maze {
    pub rows: u32,
    pub cols: u32,
    pub pieces: Vec<Vec<Piece>>,
}

impl Maze {
    pub fn from_file(fname: &str) -> Result<Maze, Error> {
        let f = try!(File::open(fname));
        let f = BufReader::new(f);

        let mut rows: Vec<Vec<Piece>> = Vec::new();

        for line in f.lines() {
            let line = try!(line);
            let mut col: Vec<Piece> = Vec::new();
            
            for sym in line.chars() {
                let piece = match sym {
                    '#' => Piece::Boulder,
                    'X' => Piece::Exit,
                    ' ' => Piece::Empty,
                    _ => return Err(Error::new(ErrorKind::Other, "invalid map")) ,
                };
                col.push(piece);
            }

            rows.push(col);
        };

        let r = rows.len() as u32;
        let c = rows[0].len() as u32;

        Ok(Maze {
            rows: r,
            cols: c,
            pieces: rows,
        })
    }

    pub fn render(&self, renderer: &mut Renderer<'static>) {
        let height = 10;
        let width = 10;

        let grey = Color::RGB(128, 128, 128);
        let white = Color::RGB(255, 255, 255);
        let yellow = Color::RGB(255, 255, 0);
        let brown = Color::RGB(139, 69, 19);

        for (y, row) in self.pieces.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                let x_loc = (x as i32) * (width as i32);
                let y_loc = (y as i32) * (height as i32);  
                match piece {
                    &Piece::Boulder => renderer.set_draw_color(grey),
                    &Piece::Empty => renderer.set_draw_color(white),
                    &Piece::Exit => renderer.set_draw_color(yellow),
                    &Piece::Player(ref dir) =>  {
                        renderer.set_draw_color(brown);
                        render_player(renderer, dir, x_loc, y_loc, width as i32, height as i32);
                        continue;
                    }
                }
                let rect = Rect::new(x_loc, y_loc, width, height);

                renderer.fill_rect(rect).unwrap();
            }
        }
    }

    pub fn add_player(&mut self) {
        let mut rng = rand::thread_rng();

        let row_range = Range::new(1, self.rows);
        let col_range = Range::new(1, self.cols);

        loop {
            let x = row_range.ind_sample(&mut rng);
            let y = col_range.ind_sample(&mut rng);

            match self.pieces[x as usize][y as usize] {
                Piece::Empty => {
                    self.pieces[x as usize][y as usize] = Piece::Player(Direction::Up);
                    break;
                },
                _ => continue,
            }
        };
    }

    fn find_player(&self) -> Option<(usize, usize, Direction)> {
        for (y, row) in self.pieces.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                match piece {
                    &Piece::Player(dir) => return Some((x, y, dir)),
                    _ => continue,
                };
            }
        }

        None
    }

    fn change_player_dir(&mut self, new_dir: Direction) -> GameState {
        let (x, y, _) = self.find_player().unwrap();

        self.pieces[y][x] = Piece::Player(new_dir);

        GameState::Moved
    }

    fn move_player_forward(&mut self) -> GameState {
        let (x, y, dir) = self.find_player().unwrap();
        let (dx, dy) = get_dxdy(dir);
        let (new_x, new_y) = (((x as i32)+dx) as usize, ((y as i32)+dy) as usize);

        if new_x < (self.cols as usize) && new_y < (self.rows as usize) {
            match self.pieces[new_y][new_x] {
                Piece::Empty => {
                    self.pieces[new_y][new_x] = Piece::Player(dir);
                    self.pieces[y][x] = Piece::Empty;
                },
                Piece::Boulder => {
                    let (x_1, y_1) = (((new_x as i32)+dx) as usize, ((new_y as i32)+dy) as usize);
                    match self.pieces[y_1][x_1] {
                        Piece::Empty => {
                            self.pieces[y_1][x_1] = Piece::Boulder;
                            self.pieces[new_y][new_x] = Piece::Player(dir);
                            self.pieces[y][x] = Piece::Empty;
                        }
                        _ => return GameState::InvalidMove,
                    }
                }
                Piece::Exit => return GameState::Won,
                _ => return GameState::InvalidMove
            }
        }

        GameState::Moved
    }

    pub fn move_player(&mut self, req_dir: Direction) -> GameState {
        let (_, _, dir) = self.find_player().unwrap();

        if dir == req_dir {
            self.move_player_forward()
        } else {
            self.change_player_dir(req_dir)
        }
    }
}