use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up, 
    Down, 
    Left, 
    Right,
}

impl Direction {
    pub fn _opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn as_move(&self, speed: i32) -> (i32, i32) {
        match *self {
            Direction::Up => (0, -speed),
            Direction::Down => (0, speed),
            Direction::Left => (-speed, 0),
            Direction::Right => (speed, 0),
        }
    }

    pub fn apply(&self, x: i32, y: i32, speed: i32) -> (i32, i32) {
        let (move_x, move_y) = self.as_move(speed);
        (x + move_x, y + move_y)
    }

    pub fn _to_string(&self) -> &str {
        match *self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

#[derive(Clone)]
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {
            x: x + 2,
            y,
        });
        body.push_back(Block {
            x: x + 1,
            y,
        });
        body.push_back(Block {
            x,
            y,
        });

        Snake {
            direction: Direction::Right,
            body,
            tail: None
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y) = self.head_position();
        let (move_x, move_y) = self.direction.as_move(1);

        self.body.push_front(Block {
            x: last_x + move_x, 
            y: last_y + move_y
        });


        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn _head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => ()
        }

        let (move_x, move_y) = moving_dir.as_move(1);
        
        (head_x + move_x, head_y + move_y)
    }

    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut body_without_head = self.body.clone();
        body_without_head.pop_front();

        for block in body_without_head {
            if x == block.x && y == block.y {
                return true;
            }
        }

        false
    }

    pub fn _get_body(&self) -> LinkedList<(i32, i32)> {
        let mut body: LinkedList<(i32, i32)> = LinkedList::new();

        for block in &self.body {
            body.push_back((block.x, block.y));
        }

        body
    }

}