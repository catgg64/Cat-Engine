#[derive(Debug, Clone)]
pub struct Coordinate2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Coordinate3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Debug)]
pub enum Range<T> {
    Range(std::ops::Range<T>),
    Full,
}

