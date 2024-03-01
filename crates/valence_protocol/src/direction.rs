use bevy_ecs::prelude::*;

use crate::{Decode, Encode};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, Component)]
pub enum Direction {
    /// -Y
    Down,
    /// +Y
    Up,
    /// -Z
    North,
    /// +Z
    South,
    /// -X
    West,
    /// +X
    East,
}
impl Direction {
    pub fn rotate(self, num: u8) -> Self {
        match num  {
            1 => match self {
                Direction::East => Direction::North,
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                _ => self,
            },
            2 => match self {
                Direction::East => Direction::West,
                Direction::North => Direction::South,
                Direction::West => Direction::East,
                Direction::South => Direction::North,
                _ => self,
            },
            3 => match self {
                Direction::East => Direction::South,
                Direction::North => Direction::East,
                Direction::West => Direction::North,
                Direction::South => Direction::West,
                _ => self,
            }
            _=> self,
        }
    }
}
