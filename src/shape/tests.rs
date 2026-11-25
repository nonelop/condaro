use crate::collision::CollidableShape;
use wisk::vector::{Vector2, Vector3};

use super::sphere::Sphere;
use super::triangle::Triangle;

fn test_triangle() -> Triangle {
    Triangle::new(Vector2::UP, Vector2::RIGHT, Vector2::ZERO)
}

#[test]
fn triangle_perimetr() {
    let triangle = test_triangle();

    assert_eq!(triangle.perimetr(), 3.414213562373095)
}

#[test]
fn sphere_collision_info() {
    let left = Sphere {
        radius: 1.0,
        center: Vector3::ZERO,
    };
    let right = Sphere {
        radius: 1.0,
        center: Vector3::RIGHT,
    };

    let info = left.get_collision(&right);

    assert_eq!(info.depth, -1.0)
}
