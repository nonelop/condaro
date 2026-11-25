//! Trait CollidableShape for shapes.

use wisk::vector::Vector3;

use crate::shape::sphere::Sphere;

pub struct CollisionInfo {
    pub depth: f64,
    pub normal: Vector3,
}

/// Trait with collision methods
pub trait CollidableShape {
    /// Takes other shape, returns info of collision with it.
    /// This method is just router, which calls other get_collision_* methods and returns their results
    fn get_collision(&self, other: &dyn CollidableShape) -> CollisionInfo;

    /// Takes other sphere, returns info of collision with it.
    fn get_collision_sphere(&self, other: &Sphere) -> CollisionInfo;
}
