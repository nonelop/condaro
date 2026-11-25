//! # Triangles and its methods

use wisk::vector::Vector2;

/// 2D Triangle object
pub struct Triangle {
    /// First triangle vertex
    pub a: Vector2,
    /// Second triangle vertex
    pub b: Vector2,
    /// Third triangle vertex
    pub c: Vector2,
}

impl Triangle {
    /// Takes 3 vertexes, returns triangle with it
    pub fn new(a: Vector2, b: Vector2, c: Vector2) -> Self {
        Self { a, b, c }
    }

    /// Returns perimetr of the triangle
    pub fn perimetr(&self) -> f64 {
        self.a.distance_to(self.b) + self.b.distance_to(self.c) + self.c.distance_to(self.a)
    }
}
