pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn swap(&mut self){
        std::mem::swap(&mut self.x, &mut self.y);
    }
}
