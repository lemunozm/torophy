use super::math::bounding::{AxisAlignmentBoundingBox};

use std::slice::{Iter};
use std::collections::HashSet;

pub struct SpatialTable {
    width: u32,
    height: u32,
    inverse_cell_size: f32,
    cells: Vec<Vec<usize>>,
    pairs: Vec<(usize, usize)>,
    checked: HashSet<usize> // Stored for performance
}

impl SpatialTable {
    pub fn new(space_width: u32, space_height: u32, cell_size: f32) -> SpatialTable {
        let inverse_cell_size = 1.0 / cell_size;
        let width = (space_width as f32 * inverse_cell_size) as u32 + if space_width % cell_size as u32 > 0 { 1 } else { 0 };
        let height = (space_height as f32 * inverse_cell_size) as u32 + if space_height % cell_size as u32 > 0 { 1 } else { 0 };
        SpatialTable {
            width,
            height,
            inverse_cell_size,
            cells: (0..width * height).map(|_| Vec::new()).collect(),
            pairs: Vec::new(),
            checked: HashSet::new(),
        }
    }

    pub fn pairs(&self) -> Iter<(usize, usize)> {
        self.pairs.iter()
    }

    pub fn transform_coordinate(&self, coordinate: f32) -> u32 {
        (coordinate * self.inverse_cell_size) as u32
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }

        self.pairs.clear();
    }

    pub fn insert(&mut self, id: usize, aabb: &AxisAlignmentBoundingBox) {
        let left = self.transform_coordinate(aabb.left());
        let right = self.transform_coordinate(aabb.right());
        let top = self.transform_coordinate(aabb.top());
        let bottom = self.transform_coordinate(aabb.bottom());

        let x_length = 1 + if left <= right {right - left} else {right + self.width - left };
        let y_length = 1 + if top <= bottom {bottom - top} else {bottom + self.height - top };

        self.checked.clear();

        let mut x = left;
        for _ in 0..x_length {
            x = (x + 1) % self.width;
            let mut y = top;
            for _ in 0..y_length {
                y = (y + 1) % self.height;
                let cell = &mut self.cells[(y * self.width + x) as usize];
                for stored_id in cell.iter() {
                    if self.checked.insert(*stored_id) {
                        self.pairs.push((id, *stored_id));
                    }
                }
                cell.push(id);
            }
        }
    }
}
