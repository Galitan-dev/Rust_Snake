use crate::snake::{Direction, Snake};
use std::collections::LinkedList;

pub struct AI {
}

impl AI {
    pub fn update(&mut self, food_x: i32, food_y: i32, snake: Snake) -> Option<Direction> {

        let (head_x, head_y) = snake.head_position();
        let dist_x = food_x - head_x;
        let dist_y = food_y - head_y;

        let dirs = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        let mut possible_dirs: LinkedList<Direction> = LinkedList::new();


        for dir in dirs {
            if self.is_dir_possible(dir, snake.clone()) {
                possible_dirs.push_back(dir);

                let (new_x, new_y) = dir.apply(head_x, head_y, 1);
                let new_dist_x = food_x - new_x;
                let new_dist_y = food_y - new_y;
                if new_dist_x.abs() < dist_x.abs() || new_dist_y.abs() < dist_y.abs() {
                    return Some(dir);
                }
            }
        }

        if dist_x == 0 {
            if possible_dirs.contains(&Direction::Left) {
                return Some(Direction::Left)
            }
            if possible_dirs.contains(&Direction::Right) {
                return Some(Direction::Right)
            }
        }

        if dist_y == 0 {
            if possible_dirs.contains(&Direction::Up) {
                return Some(Direction::Up)
            }
            if possible_dirs.contains(&Direction::Down) {
                return Some(Direction::Down);
            }
        }

        return None;
    }

    pub fn is_dir_possible(&self, dir: Direction, snake: Snake) -> bool {
        
        let (head_x, head_y) = snake.head_position();
        let (new_x, new_y) = dir.apply(head_x, head_y, 1);

        if snake.overlap_tail(new_x, new_y) || self.is_point_inside() {
            return false;
        }

        true
    }

    pub fn is_point_inside(&self) -> bool {
        false //TODO: Write this method
    }
}