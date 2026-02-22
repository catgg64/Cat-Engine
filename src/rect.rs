pub struct Rect {
    x: u64,
    y: u64,
    size_x: u64,
    size_y: u64,
}

impl Rect {
    pub fn colliderect(&self, rect: Rect) -> bool {
        if self.rect.x > rect.x || self.rect.x < rect.size_x || self.rect.y > rect.y || self.rect.y < rect.y.size{
            True
        }
        else {
            False
        }
    }
}