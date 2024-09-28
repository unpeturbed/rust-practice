pub struct Point<T> {
    pub first_param: T,
    pub second_param: T,
}

impl<T> Point<T> {
    pub fn swap(&mut self){
        std::mem::swap(&mut self.first_param, &mut self.second_param);
    }
}
