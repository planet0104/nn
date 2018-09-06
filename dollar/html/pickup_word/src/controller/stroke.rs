#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new<T: Into<f64>>(x: T, y: T) -> Point {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }
}

pub fn parse(points: &Vec<[i32; 2]>) -> Vec<Point>{
    let mut result = vec![];
    for point in points.iter().rev(){
        result.push(Point::new(point[0] as f64, point[1] as f64));
    }
    result
}

pub fn resample(mut points: Vec<Point>, n: usize) -> Vec<Point> {
    let len = path_length(&points) / (n as f64 - 1.0); // interval length
    let mut dist = 0.0;
    let mut newpoints = vec![points[0].clone()];
    let mut i = 1;
    while i < points.len() {
        let d = distance(&points[i - 1], &points[i]);
        if (dist + d) >= len {
            let qx = points[i - 1].x + ((len - dist) / d) * (points[i].x - points[i - 1].x);
            let qy = points[i - 1].y + ((len - dist) / d) * (points[i].y - points[i - 1].y);
            let q = Point::new(qx, qy);
            newpoints.push(q.clone()); // append Point::new 'q'
            points.insert(i, q); // insert 'q' at position i in points s.t. 'q' will be the next i
            dist = 0.0;
        } else {
            dist += d;
        }
        i += 1;
    }
    if newpoints.len() == n as usize - 1 {
        // somtimes we fall a rounding-error short of adding the last point, so add it if so
        newpoints.push(Point::new(
            points[points.len() - 1].x,
            points[points.len() - 1].y,
        ));
    }
    newpoints
}

fn path_length(points: &Vec<Point>) -> f64 {
    let mut d = 0.0;
    for i in 1..points.len() {
        d += distance(&points[i - 1], &points[i])
    }
    d
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    (dx * dx + dy * dy).sqrt()
}