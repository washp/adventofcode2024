#![allow(dead_code)]
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: std::ops::Add<Output = T>> Add for Vector2D<T> {
    type Output = Self;
    fn add(self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: std::ops::Sub<Output = T>> Sub for Vector2D<T> {
    type Output = Self;
    fn sub(self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Vector2D<T> {
    pub const fn new(x: T, y: T) -> Vector2D<T> {
        Vector2D { x, y }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: std::ops::Add<Output = T>> Add for Vector3D<T> {
    type Output = Self;
    fn add(self, other: Vector3D<T>) -> Vector3D<T> {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: std::ops::Sub<Output = T>> Sub for Vector3D<T> {
    type Output = Self;
    fn sub(self, other: Vector3D<T>) -> Vector3D<T> {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Vector3D<T> {
    pub const fn new(x: T, y: T, z: T) -> Vector3D<T> {
        Vector3D { x, y, z }
    }
}
