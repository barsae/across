extern crate core;

use std::fmt::{Display, Formatter, Error};
use ::SquareState::{BLOCK, PLAYABLE};
use ::SquareState::EMPTY;
use ::SquareState::FULL;
use core::fmt::Write;

#[derive(Copy, Clone, Debug, PartialEq)]
enum SquareState {
    BLOCK,
    EMPTY,
    FULL,
    PLAYABLE,
}

#[derive(Debug)]
struct Square {
    state: SquareState,
    count: u8,
}

#[derive(Debug)]
struct Play {
    x: i8,
    y: i8,
    dx: i8,
    dy: i8,
    count: u8,
}

fn from_hex(c: char) -> u8 {
    if c >= '1' && c <= '9' {
        c as u8 - '1' as u8 + 1
    } else if c >= 'a' && c <= 'f' {
        c as u8 - 'a' as u8 + 10
    } else {
        panic!("Can't to_hex numbers outside 1-9 and a-f")
    }
}

fn to_hex(i: u8) -> char {
    if i >= 1 && i <= 9 {
        ('0' as u8 + i) as char
    } else if i >= 10 && i <= 16 {
        ('a' as u8 + i - 10) as char
    } else {
        panic!("Can't to_hex numbers outside 1-16")
    }
}

impl Square {
    fn parse(c: char) -> Result<Square, String> {
        if c >= '1' && c<= '9' {
            Ok(Square {
                state: PLAYABLE,
                count: from_hex(c),
            })
        } else if c == ' ' {
            Ok(Square {
                state: BLOCK,
                count: 0,
            })
        } else if c == '.' {
            Ok(Square {
                state: EMPTY,
                count: 0,
            })
        } else {
            Err(format!("Couldn't parse {} as a Square", c))
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.state {
            PLAYABLE => f.write_char(to_hex(self.count))?,
            FULL  => f.write_char('X')?,
            EMPTY => f.write_char('.')?,
            BLOCK => f.write_char(' ')?,
        };
        Ok(())
    }
}

struct Map {
    grid: Vec<Vec<Square>>,
}

impl Map {
    fn parse(s: &str) -> Result<Map, String> {
        let mut lines = vec!();
        let mut line = vec!();

        for c in s.chars() {
            if c.is_ascii_hexdigit() || c == ' ' || c == '.' {
                line.push(Square::parse(c)?);
            } else if c == '\n' {
                lines.push(line);
                line = vec!();
            }
        }

        lines.push(line);

        Ok(Map{
            grid: lines
        })
    }

    fn get_plays(&self) -> Vec<Play> {
        let mut plays = vec!();

        for (y, line) in self.grid.iter().enumerate() {
            for (x, square) in line.iter().enumerate() {
                if square.state == PLAYABLE {
                    let play = Play { x: x as i8, y: y as i8, dx: 1, dy: 0, count: square.count };
                    if self.is_legal_play(&play) {
                        plays.push(play)
                    }
                }
            }
        }

        return plays;
    }

    fn is_legal_play(&self, play: &Play) -> bool {
        let mut x = play.x + play.dx;
        let mut y = play.y + play.dy;
        let mut empties = 0;

        while empties < play.count {
            println!("x, y: {}, {}", x, y);
            match self.get_square(x, y).map(|square| square.state) {
                Some(EMPTY) => empties += 1,
                Some(BLOCK) | None => return false,
                _ => {
                    x += play.dx;
                    y += play.dy;
                }
            }
        }

        return true;
    }

    fn get_square(&self, x: i8, y: i8) -> Option<&Square> {
        self.grid.get(y as usize).map(|line| line.get(x as usize)).unwrap_or_default()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for line in self.grid.iter() {
            for square in line.iter() {
                write!(f, "{}", square)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let map = Map::parse("1.\n1.\n1.")?;
    println!("map");
    println!("{}", map);
    let plays = map.get_plays();
    println!("{:?}", plays);
    Ok(())
}


