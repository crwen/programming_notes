fn main() {
    let mut p1 = Point::point_at(3, 5);
    let _p2 = Point{x: 5.0, y: 10.2};
    let _p3 = Point{x: 5, y: 10.2};

    p1.move_to(3, 4);
    p1.move_x(1);
    p1.move_y(-1);
}


struct Point<T, U> {
    x: T,
    y: U,
}

impl<T,U> Point<T, U> {
    fn point_at(x: T, y: U) -> Point<T, U> {
        Point {
            x, y
        }
    }
    fn move_to<T2,U2>(&self, x: T2, y: U2) -> Point<T2, U2> {
        Point {
            x, y
        }
    }
    fn move_y(&mut self, y: U) {
        self.y = y;
    }

}

impl<U> Point<i32, U> {
    fn move_x(&mut self, x: i32) {
        self.x += x;
    }
}

