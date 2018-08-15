use std::convert::TryFrom;
use std::fmt;
use std::iter::Iterator;
use std::ops::{Index, IndexMut};

use serde_derive::Serialize;
use serdebug::SerDebug;

use super::grid::Grid;

#[derive(Serialize, SerDebug)]
pub struct SquareGrid<Item: Default + Clone> {
    width: usize,
    height: usize,
    #[serde(with = "super::ellipsis_serializer")]
    items: Vec<Item>,
}

impl<Item: Default + Clone> Grid<(usize, usize)> for SquareGrid<Item> {
    fn neighbours(&self, index: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = index;
        let mut v = Vec::new();
        if x > 0 {
            if y > 0 {
                v.push((x - 1, y - 1))
            }
            v.push((x - 1, y));
            if y + 1 < self.height {
                v.push((x - 1, y + 1))
            }
        }
        if y > 0 {
            v.push((x, y - 1))
        }
        v.push((x, y));
        if y + 1 < self.height {
            v.push((x, y + 1))
        }
        if x + 1 < self.width {
            if y > 0 {
                v.push((x + 1, y - 1))
            }
            v.push((x + 1, y));
            if y + 1 < self.height {
                v.push((x + 1, y + 1))
            }
        }
        v
    }

    fn distance(&self, a: (usize, usize), b: (usize, usize)) -> u32 {
        let (a_x, a_y) = a;
        let (b_x, b_y) = b;
        let x_distance =
            u32::try_from((i64::try_from(a_x).unwrap() - i64::try_from(b_x).unwrap()).abs())
                .unwrap();
        let y_distance =
            u32::try_from((i64::try_from(a_y).unwrap() - i64::try_from(b_y).unwrap()).abs())
                .unwrap();
        x_distance + y_distance - x_distance.min(y_distance)
    }
}

impl<Item: Default + Clone> SquareGrid<Item> {
    pub fn new(width: usize, height: usize) -> Self {
        let items = vec![Item::default(); width * height];
        Self {
            width,
            height,
            items,
        }
    }

    pub fn len(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn inner_index(&self, index: (usize, usize)) -> usize {
        let (x, y) = index;
        if x >= self.width || y >= self.height {
            panic!(
                "index ({}, {}) out of bounds ({}, {})",
                x, y, self.width, self.height
            );
        }

        y * self.width + x
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &Item)> {
        let height = self.height;
        let width = self.width;
        (0..height)
            .flat_map(move |y| (0..width).map(move |x| (x, y)))
            .zip(self.items.iter())
    }
}

impl<Item: Default + Clone + fmt::Display> fmt::Display for SquareGrid<Item> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ((x, y), item) in self.iter() {
            if x == 0 && y > 0 {
                write!(f, "\n")?;
            }

            write!(f, "{:5} ", item)?;
        }

        Ok(())
    }
}

impl<Item: Default + Clone> Index<(usize, usize)> for SquareGrid<Item> {
    type Output = Item;
    fn index(&self, index: (usize, usize)) -> &Item {
        let i = self.inner_index(index);
        &self.items[i]
    }
}

impl<Item: Default + Clone> IndexMut<(usize, usize)> for SquareGrid<Item> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Item {
        let i = self.inner_index(index);
        &mut self.items[i]
    }
}
