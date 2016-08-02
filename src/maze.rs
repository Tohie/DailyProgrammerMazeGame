use std::fs::File;
use std::io::{BufReader, Error, BufRead, ErrorKind};
use std::fmt;

pub enum Piece {
    Empty,
    Player,
    Boulder,
    Exit,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            &Piece::Empty => ' ',
            &Piece::Player => 'O',
            &Piece::Boulder => '#',
            &Piece::Exit => 'X',
        };

        write!(f, "{}", repr)
    }
}

pub struct Maze {
    rows: u16,
    cols: u16,
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
                    'O' => Piece::Player,
                    ' ' => Piece::Empty,
                    _ => return Err(Error::new(ErrorKind::Other, "invalid map")) ,
                };
                col.push(piece);
            }

            rows.push(col);
        };

        let r = rows.len() as u16;
        let c = rows[0].len() as u16;

        Ok(Maze {
            rows: r,
            cols: c,
            pieces: rows,
        })
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.pieces.iter() {
            for col in row.iter() {
                write!(f, "{}", col);
            }
            write!(f, "\r\n");
        }
        write!(f, "\r\n")
    }
}