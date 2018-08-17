use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    collections::BinaryHeap,
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};

use log::{debug, error, info, log, trace, warn, Log};

/// The value used for each axis of a Position.
pub type Distance = usize;
/// A 2D index/position for our grids.
pub type Position = (Distance, Distance);

/// An fixed-size array of values indexed by an (x, y) tuple.
pub struct Array2D<Value>
where
    Value: Clone + Default,
{
    width: Distance,
    height: Distance,
    values: Vec<Value>,
}

impl<Value> Array2D<Value>
where
    Value: Clone + Default,
{
    /// Constructs an Array2D with the given dimension filled with [Value::default].
    fn new(width: Distance, height: Distance) -> Self {
        Self {
            width,
            height,
            values: vec![Value::default(); width * height],
        }
    }

    pub fn len(&self) -> Position {
        (self.width, self.height)
    }

    fn inner_index(&self, index: Position) -> usize {
        let (x, y) = index;
        if x >= self.width || y >= self.height {
            panic!(
                "index ({}, {}) out of bounds ({}, {})",
                x, y, self.width, self.height
            );
        }

        y * self.width + x
    }

    pub fn iter(&self) -> impl Iterator<Item = (Position, &Value)> {
        let height = self.height;
        let width = self.width;
        (0..height)
            .flat_map(move |y| (0..width).map(move |x| (x, y)))
            .zip(self.values.iter())
    }
}

impl<Value> Index<Position> for Array2D<Value>
where
    Value: Clone + Default,
{
    type Output = Value;
    fn index(&self, index: Position) -> &Value {
        let i = self.inner_index(index);
        &self.values[i]
    }
}

impl<Value> IndexMut<Position> for Array2D<Value>
where
    Value: Clone + Default,
{
    fn index_mut(&mut self, index: Position) -> &mut Value {
        let i = self.inner_index(index);
        &mut self.values[i]
    }
}

#[derive(Clone, Default, Copy)]
pub struct AStarCell {
    state: AStarCellState,
}

impl AStarCell {
    pub fn state(&self) -> AStarCellState {
        self.state
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum AStarCellState {
    Free,
    Blocked,
    VisitedFrom(Position),
}

impl Default for AStarCellState {
    fn default() -> Self {
        AStarCellState::Free
    }
}

/// A potential path.
#[derive(Hash, Eq, PartialEq)]
pub struct AStarPath {
    /// The position at the end of this path.
    head: Position,
    /// The cost that this path has taken so far.
    cost_from_origin: Distance,
    /// Our heuristic's lower bound on the cost to the target.
    min_cost_to_target: Distance,

    diagonless_distance_to_target: Distance,
}

impl AStarPath {
    fn min_cost(&self) -> Distance {
        // Exclude cost_from_origin and you get dijkstra’s (prioritize options that seem closer to target).
        // Exclude min_cost_to_target and you get breadth-first-search (prioritize options close to source).
        // Include both and we have A*.
        let a_star = self.cost_from_origin + self.min_cost_to_target;
        let dijkstras = self.min_cost_to_target;
        let bfs = self.cost_from_origin;
        let dfs = Distance::max_value() / 2 - self.cost_from_origin;

        a_star
    }

    fn default_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

/// The priority of exploring this path relative to other potential paths.
impl Ord for AStarPath {
    fn cmp(&self, other: &Self) -> Ordering {
        let cost_ordering = self.min_cost().cmp(&other.min_cost()).reverse();

        let min_cost_to_target_ordering = self
            .min_cost_to_target
            .cmp(&other.min_cost_to_target)
            .reverse();

        let min_diagonless_distance_to_target_ordering = self
            .diagonless_distance_to_target
            .cmp(&other.diagonless_distance_to_target)
            .reverse();

        let arbitrary_stable_ordering = self.default_hash().cmp(&other.default_hash());

        cost_ordering
            .then(min_cost_to_target_ordering)
            .then(min_diagonless_distance_to_target_ordering)
            .then(arbitrary_stable_ordering)
    }
}

impl PartialOrd for AStarPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AStarPathfinder {
    /// Whether this is still running.
    working: bool,
    /// The width of the grid in cells.
    width: Distance,
    /// The height of the grid in cells.
    height: Distance,
    /// The point at which our paths start.
    origin: Position,
    /// The point our paths are trying to find.
    target: Position,
    /// Pathfinding data associated with each cell of the grid.
    data: Array2D<AStarCell>,
    /// Paths that we're still exploring.
    frontier: BinaryHeap<AStarPath>,
}

impl AStarPathfinder {
    pub fn working(&self) -> bool {
        self.working
    }

    pub fn data(&self) -> &Array2D<AStarCell> {
        &self.data
    }

    pub fn get_path(&mut self) -> Option<Vec<Position>> {
        while self.working {
            self.step();
        }

        self.peek_path()
    }

    pub fn peek_path(&mut self) -> Option<Vec<Position>> {
        if let Some(path) = self.frontier.peek() {
            let mut full_path = Vec::new();
            let mut current = path.head;

            loop {
                full_path.push(current);
                if let AStarCellState::VisitedFrom(neighbour) = self.data[current].state {
                    current = neighbour;
                } else {
                    break;
                }
            }

            full_path.reverse();

            Some(full_path)
        } else {
            None
        }
    }

    /// Advances the pathfinding by one step.
    pub fn step(&mut self) {
        if !self.working {
            return;
        }

        if let Some(path) = { self.frontier.pop() } {
            if path.head == self.target {
                debug!("Found path to target.");
                self.working = false;
                // Put it back in front.
                self.frontier.push(path);
            } else {
                let open_neighbours: Vec<Position> = self
                    .neighbours(path.head)
                    .into_iter()
                    .filter(|position| self.data[*position].state == AStarCellState::Free)
                    .collect();

                debug!(
                    "Exploring {:?} new neighbours of {:?}.",
                    open_neighbours.len(),
                    path.head
                );

                for neighbour in open_neighbours.iter() {
                    self.data[*neighbour].state = AStarCellState::VisitedFrom(path.head);
                }

                let new_frontier: Vec<AStarPath> = open_neighbours
                    .into_iter()
                    .map(|position| AStarPath {
                        head: position,
                        cost_from_origin: path.cost_from_origin + 1,
                        min_cost_to_target: Self::min_distance(position, self.target),
                        diagonless_distance_to_target: Self::diagonless_distance(
                            position,
                            self.target,
                        ),
                    }).collect();

                self.frontier.extend(new_frontier);
            }
        } else {
            debug!("Frontier exhausted without finding end point.");
            self.working = false;
        }
    }

    fn neighbours(&self, position: Position) -> Vec<Position> {
        let (x, y) = position;
        let mut vec = Vec::with_capacity(8);
        if x > 0 {
            if y > 0 {
                vec.push((x - 1, y - 1))
            }
            vec.push((x - 1, y));
            if y + 1 < self.height {
                vec.push((x - 1, y + 1))
            }
        }
        if y > 0 {
            vec.push((x, y - 1))
        }
        vec.push((x, y));
        if y + 1 < self.height {
            vec.push((x, y + 1))
        }
        if x + 1 < self.width {
            if y > 0 {
                vec.push((x + 1, y - 1))
            }
            vec.push((x + 1, y));
            if y + 1 < self.height {
                vec.push((x + 1, y + 1))
            }
        }
        vec
    }

    /// Minimum distnace between two points with 8-way movement allowed.
    fn min_distance(a: Position, b: Position) -> Distance {
        (a.0.max(b.0) - a.0.min(b.0)).max(a.1.max(b.1) - a.1.min(b.1))
    }

    fn diagonless_distance(a: Position, b: Position) -> Distance {
        (a.0.max(b.0) - a.0.min(b.0)) + (a.1.max(b.1) - a.1.min(b.1))
    }

    pub fn demo(demo_iteration: usize) -> Self {
        let mut self_ = Self::default();

        let ref mut array = &mut self_.data;
        array[self_.origin].state = AStarCellState::Blocked;

        for x in 0..self_.width {
            for y in 0..self_.height {
                if x + y < 5 {
                    continue;
                }

                if (self_.width - x) + (self_.height - y) < 5 {
                    continue;
                }

                if ((x) ^ ((y + 3) % 2) ^ demo_iteration) % (1 + x / 2 + y / 5) <= 0 {
                    array[(x, y)].state = AStarCellState::Blocked;
                }

                if ((x / 8) + ((y * 3) / 8)) % 5 == 0 {
                    array[(x, y)].state = AStarCellState::Blocked;
                }
            }
        }

        self_
    }
}

impl Default for AStarPathfinder {
    fn default() -> Self {
        let width = 64;
        let height = 64;
        let origin = (1, 1);
        let target = (width - 2, height - 2);
        Self {
            width,
            height,
            origin,
            target,
            data: Array2D::<AStarCell>::new(width, height),
            frontier: BinaryHeap::from(vec![AStarPath {
                head: origin,
                cost_from_origin: 0,
                diagonless_distance_to_target: Self::diagonless_distance(origin, target),
                min_cost_to_target: Self::min_distance(origin, target),
            }]),
            working: true,
        }
    }
}
