use std::vec;

use crate::binary_tree::*;
use nannou::prelude::*;
use rand::random;
use rand::seq::SliceRandom;

#[derive(Clone, Copy)]
pub enum Direction {
    X,
    Y,
}

#[derive(Clone, Copy)]
pub struct Split {
    pub direction: Direction,
    pub ratio: f32,
}

impl Split {
    #[allow(unused)]
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            direction: [Direction::X, Direction::Y]
                .choose(&mut rng)
                .unwrap()
                .to_owned(),
            ratio: random::<f32>(),
        }
    }

    #[allow(unused)]
    fn alternating(i: i32) -> Self {
        Self {
            direction: if i % 2 == 0 {
                Direction::X
            } else {
                Direction::Y
            },
            ratio: random::<f32>(),
        }
    }

    fn apply(&self, rect: &Rect) -> (Rect, Rect) {
        match self.direction {
            Direction::X => (
                rect.pad_right(rect.w() * self.ratio),
                rect.pad_left(rect.w() * (1.0 - self.ratio)),
            ),
            Direction::Y => (
                rect.pad_bottom(rect.h() * self.ratio),
                rect.pad_top(rect.h() * (1.0 - self.ratio)),
            ),
        }
    }
}

use BinaryTree::{Leaf, Node};
pub type SplitTree = BinaryTree<Split, ()>;

impl SplitTree {
    pub fn random(depth: i32) -> SplitTree {
        if depth > 0 {
            Node {
                value: Split::random(),
                a: Box::new(Self::random(depth - 1)),
                b: Box::new(Self::random(depth - 1)),
            }
        } else {
            Leaf { value: () }
        }
    }

    pub fn rectangles(&self, bounds: Rect) -> Vec<Rect> {
        match self {
            Node { value, a, b } => {
                let (a_bounds, b_bounds) = value.apply(&bounds);
                a.rectangles(a_bounds)
                    .into_iter()
                    .chain(b.rectangles(b_bounds).into_iter())
                    .collect()
            }
            Leaf { .. } => vec![bounds],
        }
    }
}
