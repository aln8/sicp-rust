struct Points2D {
    x: f64,
    y: f64,
}

impl Points2D {
    fn new(x: f64, y: f64) -> Points2D {
        Points2D { x, y }
    }
}

struct LineSegment {
    start: Points2D,
    end: Points2D,
}

impl LineSegment {
    fn new(start: Points2D, end: Points2D) -> LineSegment {
        LineSegment { start, end }
    }

    fn mid_segment(&self) -> Points2D {
        Points2D {
            x: (self.start.x + self.end.x) / 2.0,
            y: (self.start.y + self.end.y) / 2.0,
        }
    }
}

#[test]
fn test_mid_segment() {
    let start = Points2D::new(1.0, 2.0);
    let end = Points2D::new(2.0, 3.0);
    let line = LineSegment::new(start, end);
    let mid_point = line.mid_segment();
    assert_eq!(mid_point.x, 1.5);
    assert_eq!(mid_point.y, 2.5);
}

trait Polygon {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Rectangle {
    left_top: Points2D,
    length: f64,
    width: f64,
}

impl Rectangle {
    fn new(left_top: Points2D, length: f64, width: f64) -> Rectangle {
        Rectangle {
            left_top,
            length,
            width,
        }
    }
}

impl Polygon for Rectangle {
    fn perimeter(&self) -> f64 {
        (self.width + self.length) * 2.0
    }

    fn area(&self) -> f64 {
        self.width * self.length
    }
}

#[test]
fn test_rectangle() {
    let rec = Rectangle::new(Points2D::new(1.0, 2.0), 3.0, 1.0);
    assert_eq!(8.0, rec.perimeter());
    assert_eq!(3.0, rec.area());
}

struct Rectangle2 {
    left_top: Points2D,
    right_bot: Points2D,
}

impl Rectangle2 {
    fn new(left_top: Points2D, right_bot: Points2D) -> Rectangle2 {
        Rectangle2 {
            left_top,
            right_bot,
        }
    }
}

impl Polygon for Rectangle2 {
    fn area(&self) -> f64 {
        (self.left_top.y - self.right_bot.y) * (self.right_bot.x - self.left_top.x)
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.left_top.y - self.right_bot.y) + 2.0 * (self.right_bot.x - self.left_top.x)
    }
}

#[test]
fn test_rectangle2() {
    let rec = Rectangle2::new(Points2D::new(1.0, 2.0), Points2D { x: 4.0, y: 1.0 });
    assert_eq!(8.0, rec.perimeter());
    assert_eq!(3.0, rec.area());
}
