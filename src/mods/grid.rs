use std::fmt;
use std::iter::Iterator;
use std::ops::{Index, IndexMut};

use serde_derive::Serialize;
use serdebug::SerDebug;

#[derive(Serialize, SerDebug)]
pub struct Grid<Item: Default + Clone> {
    width: usize,
    height: usize,
    #[serde(with = "super::ellipsis_serializer")]
    items: Vec<Item>,
}

impl<Item: Default + Clone> Grid<Item> {
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

impl<Item: Default + Clone + fmt::Display> fmt::Display for Grid<Item> {
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

impl<Item: Default + Clone> Index<(usize, usize)> for Grid<Item> {
    type Output = Item;
    fn index(&self, index: (usize, usize)) -> &Item {
        let i = self.inner_index(index);
        &self.items[i]
    }
}

impl<Item: Default + Clone> IndexMut<(usize, usize)> for Grid<Item> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Item {
        let i = self.inner_index(index);
        &mut self.items[i]
    }
}
