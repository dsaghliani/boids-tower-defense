use bevy::prelude::Vec2;
use itertools::Itertools;
use std::collections::HashMap;

type Region = (i32, i32);

pub struct SpatialHashMap2D<Value: Clone> {
    cell_size: f32,
    map: HashMap<Region, Vec<Value>>,
}

impl<Value> SpatialHashMap2D<Value>
where
    Value: Clone,
{
    #[must_use]
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            map: HashMap::new(),
        }
    }

    /// Get the values in own and adjacent regions.
    ///
    /// ```
    /// use bevy::prelude::Vec2;
    /// use boids_tower_defense::SpatialHashMap2D;
    ///
    /// #[derive(Clone)]
    /// struct DummyValue;
    ///
    /// // Create a map with the cell size of 1.0.
    /// let mut map = SpatialHashMap2D::new(1.0);
    ///
    /// // Store a point in each cell of a 3x3 "matrix".
    /// for y in 0..3 {
    ///     for x in 0..3 {
    ///         // Create a point in the middle of each cell.
    ///         let x = x as f32 + 0.5;
    ///         let y = y as f32 + 0.5;
    ///
    ///         // Store it in the spatial hash map.
    ///         map.add(Vec2::new(x, y), DummyValue);
    ///     }
    /// }
    ///
    /// let centermost_point = Vec2::new(1.5, 1.5);
    ///
    /// // Assert the method returns a point from each of the cells.
    /// assert_eq!(map.neighbors(centermost_point).len(), 9);
    /// ```
    #[must_use]
    pub fn neighbors(&mut self, position: Vec2) -> Vec<Value> {
        let (x, y) = self.vec2_to_region(position);

        (y - 1..=y + 1)
            .cartesian_product(x - 1..=x + 1)
            .flat_map(|(y, x)| {
                self.map.entry((x, y)).or_insert_with(Vec::new).clone()
            })
            .collect()
    }

    /// Store a value with the given position in the map.
    ///
    /// ```
    /// use bevy::prelude::Vec2;
    /// use boids_tower_defense::SpatialHashMap2D;
    ///
    /// #[derive(Clone)]
    /// struct DummyValue;
    ///
    /// // Create a map with the cell size of 1.0 and store a value in the cell (0,0).
    /// let mut map = SpatialHashMap2D::new(1.0);
    /// let point = Vec2::new(0.5, 0.5);
    /// map.add(point, DummyValue);
    ///
    /// // Assert the value was added.
    /// assert_eq!(map.neighbors(point).len(), 1);
    /// ```
    pub fn add(&mut self, position: Vec2, value: Value) {
        let region = self.vec2_to_region(position);
        self.map.entry(region).or_insert_with(Vec::new).push(value);
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn vec2_to_region(&self, position: Vec2) -> Region {
        (
            (position.x / self.cell_size) as i32,
            (position.y / self.cell_size) as i32,
        )
    }
}
