use super::Vec2;

pub struct AxisAlignmentBoundingBox {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl AxisAlignmentBoundingBox {
    pub fn new(position: Vec2, half_size: Vec2) -> AxisAlignmentBoundingBox {
        AxisAlignmentBoundingBox {
            left: position.x - half_size.x,
            right: position.x + half_size.x,
            top: position.y - half_size.y,
            bottom: position.y + half_size.y,
        }
    }

    pub fn from_bounds(left: f32, right: f32, top: f32, bottom: f32) -> AxisAlignmentBoundingBox {
        AxisAlignmentBoundingBox { left, right, top, bottom }
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn right(&self) -> f32 {
        self.right
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    pub fn position(&self) -> Vec2 {
        Vec2::xy((self.right + self.left) / 2.0, (self.bottom + self.top) / 2.0)
    }

    pub fn half_dimension(&self) -> Vec2 {
        Vec2::xy((self.right - self.left) / 2.0, (self.bottom - self.top) / 2.0)
    }

    pub fn dimension(&self) -> Vec2 {
        Vec2::xy(self.right - self.left, self.bottom - self.top)
    }
}
