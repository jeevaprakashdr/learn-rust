#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct GenericPointT<T> {
    x: T,
    y: T,
}

impl<T> GenericPointT<T> {
    
    fn x(&self) -> &T {
        &self.x
    }
}

impl GenericPointT<f32> {
    
    fn y(&self) -> f32 {
        self.y
    }
}

#[derive(Debug)]
struct GenericPointTU<T, U> {
    x: T,
    y: U,
}

impl<T, U> GenericPointTU<T, U> {
    fn mixup<V, W>(self, other: GenericPointTU<V, W>) -> GenericPointTU<T, W> {
        GenericPointTU{
            x: self.x,
            y: other.y
        }
    }
}

pub fn basics() {
    let p1 = Point{x : 5, y:10};
    let  p2 = GenericPointT{x : 5.5, y:10.8};
    let  p3 = GenericPointTU{x : 5, y:10.8};
    
    println!("Point 1 {:#?}", p1);
    println!("Point 2 {:#?}", p2);
    println!("Point 3 {:#?}", p3);

    let  other = GenericPointTU{x : 5.4, y:"abc"};
    println!("Point other {:#?}", other);

    let mixed_up_point = p3.mixup(other);
    println!("mixed up point {:#?}", mixed_up_point);  
}