use std::fmt::Debug;
use std::ops::IndexMut;

use super::grid::Grid;

use serde::ser::Serialize;
use serde_derive::Serialize;
use serdebug::SerDebug;

use log::{debug, error, info, log, trace, warn, Log};

#[derive(Default, Clone)]
pub struct NodeInfo {
    pub visited: bool,
}

#[derive(Serialize, SerDebug)]
pub struct Walker<IndexType, GridType>
where
    IndexType: Copy + Debug + PartialEq + Serialize,
    GridType: Grid<IndexType> + IndexMut<IndexType, Output = NodeInfo> + Debug + Serialize,
{
    grid: GridType,
    end_point: IndexType,

    #[serde(with = "super::ellipsis_serializer")]
    current_path: Vec<IndexType>,

    #[serde(with = "super::ellipsis_serializer")]
    strategy: fn(neighbours: Vec<IndexType>, target: IndexType, grid: &GridType) -> Vec<IndexType>,
}

impl<IndexType, GridType> Walker<IndexType, GridType>
where
    IndexType: Copy + Debug + PartialEq + Serialize,
    GridType: Grid<IndexType> + IndexMut<IndexType, Output = NodeInfo> + Debug + Serialize,
{
    pub fn new(
        mut grid: GridType,
        start_point: IndexType,
        end_point: IndexType,
        strategy: fn(neighbours: Vec<IndexType>, target: IndexType, grid: &GridType)
            -> Vec<IndexType>,
    ) -> Self {
        grid[start_point].visited = true;
        let current_path = vec![start_point];
        let s = Self {
            grid,
            end_point,
            current_path,
            strategy,
        };
        info!("Initialized {:?}.", s);
        s
    }

    /// Advances the pathfinding by a single step.
    /// Returns true iff we're still running, else subsequent calls will have no effect.
    pub fn step(&mut self, rng: &mut impl rand::Rng) -> bool {
        if self.current_path.len() == 0 {
            return false;
        }

        let head = self.current_path[self.current_path.len() - 1];

        if head == self.end_point {
            return false;
        }

        let neighbours = self.grid.neighbours(head);
        let unvisited = neighbours
            .into_iter()
            .filter(|neighbour| self.grid[*neighbour].visited == false)
            .collect();

        let preferred = (self.strategy)(unvisited, self.end_point, &self.grid);
        if preferred.len() > 0 {
            let chosen = rng.choose(&preferred).unwrap();
            self.current_path.push(*chosen);
            self.grid[*chosen].visited = true;
        } else {
            self.current_path.pop();
        }

        head != self.end_point
    }

    pub fn current_path(&self) -> Vec<IndexType> {
        self.current_path.clone()
    }
}
