//Okay! I have written this program to be able to accept different datatypes in the same program while also returning a befitting datatype.
use std::ops::Mul;

pub struct Rectangle<T, U> {
    pub width: T,
    pub height: U,
}

impl<T: Copy, U: Copy> Rectangle<T, U>
where
    T: Mul<U>,
    <T as Mul<U>>::Output: Copy,
{
    pub fn area(&self) -> <T as Mul<U>>::Output {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_of_rectangle() {
        let rectangle = Rectangle { width: 30, height: 50 };    
        assert_eq!(rectangle.area(), 1500);
    }

    #[test]
    fn area_of_rectangle_2() {
        let rectangle = Rectangle { width: -30, height: 50 };    
        assert_eq!(rectangle.area(), -1500);
    }

    #[test]
    fn area_of_rectangle_3() {
        let rectangle = Rectangle { width: 30.6, height: -50.8 };    
        assert_eq!(rectangle.area(), -1554.48);
    }

}