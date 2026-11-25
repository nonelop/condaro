//! # Sphere and its methods

use wisk::vector::Vector3;

use crate::collision::{CollidableShape, CollisionInfo};

/// 3D Sphere object
pub struct Sphere {
    /// Center of sphere in space
    pub center: Vector3,
    /// Radius of sphere
    pub radius: f64,
}

impl CollidableShape for Sphere {
    fn get_collision(&self, other: &dyn CollidableShape) -> CollisionInfo {
        other.get_collision_sphere(self)
    }
    fn get_collision_sphere(&self, other: &Sphere) -> CollisionInfo {
        CollisionInfo {
            depth: self.center.distance_to(other.center) - (self.radius + other.radius),
            normal: Vector3::ZERO,
        }
    }
}
