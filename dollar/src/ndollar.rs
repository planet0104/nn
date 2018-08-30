use std;

use std::time::{Duration, Instant};
use std::f64::consts::PI;

const NUM_UNISTROKES: usize = 16;
const NUM_POINTS: usize = 96;
const SQUARE_SIZE: f64 = 250.0;
const ONE_D_THRESHOLD:f64 = 0.25;// customize to desired gesture set (usually 0.20 - 0.35)
const ORIGIN: Point = Point { x: 0.0, y: 0.0 };
//const DIAGONAL:f64 = (SQUARE_SIZE * SQUARE_SIZE + SQUARE_SIZE * SQUARE_SIZE).sqrt();
const DIAGONAL: f64 = 353.5533905932738;
const HALF_DIAGONAL: f64 = 0.5 * DIAGONAL;
//const ANGLE_RANGE:f64 = deg2rad(45.0);
const ANGLE_RANGE: f64 = 0.7853981633974483;
//const ANGLE_PRECISION:f64 = deg2rad(2.0);
const ANGLE_PRECISION: f64 = 0.03490658503988659;
//const PHI:f64 = 0.5 * (-1.0 + 5.0f64.sqrt()); // Golden Ratio
const PHI: f64 = 0.5 * (-1.0 + 2.23606797749979); // Golden Ratio
const START_ANGLE_INDEX:usize = NUM_POINTS/8; // eighth of gesture length
//const AngleSimilarityThreshold = Deg2Rad(30.0);
const ANGLE_SIMILARITY_THRESHOLD: f64 = 0.5235987755982988;

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

struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new<T: Into<f64>>(x: T, y: T, width: T, height: T) -> Rectangle {
        Rectangle {
            x: x.into(),
            y: y.into(),
            width: width.into(),
            height: height.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unistroke {
    name: String,
    points: Vec<Point>,
    start_unit_vector: Point,
    vector: Vec<f64>,
}

impl Unistroke {
    fn new(name: &str, use_bounded_rotation_invariance: bool, mut points: Vec<Point>) -> Unistroke {
        points = resample(points, NUM_POINTS);
        let radians = indicative_angle(&points);
        points = rotate_by(&points, -radians);
        points = scale_dim_to(&points, SQUARE_SIZE, ONE_D_THRESHOLD);
        if use_bounded_rotation_invariance{
            points = rotate_by(&points, radians);//restore
        }
        points = translate_to(&points, &ORIGIN);
        let start_unit_vector = calc_start_unit_vector(&points, START_ANGLE_INDEX);
        let vector = vectorize(&points, use_bounded_rotation_invariance);// for Protractor

        Unistroke {
            name: name.to_string(),
            points,
            start_unit_vector,
            vector,
        }
    }
}

impl Default for Unistroke {
    fn default() -> Unistroke {
        Unistroke {
            name: String::new(),
            points: vec![],
            vector: vec![],
            start_unit_vector: Point::new(0, 0)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Multistroke {
    name: String,
    num_strokes: usize,
    unistrokes: Vec<Unistroke>
}

impl Multistroke {
    fn new(name: &str, use_bounded_rotation_invariance: bool, mut strokes: Vec<Point>) -> Multistroke {
        
        let num_strokes = strokes.len();// number of individual strokes
        let mut order = vec![];
        for i in 0..strokes.len(){
            order.push(i);// initialize
        }
        let mut orders = vec![vec![]];// array of integer arrays
        heap_permute(strokes.len(), &mut order, &mut orders);

        let unistrokes = make_unistrokes(strokes, orders);
        

        Multistroke {
            name: name.to_string(),
            unistrokes: vec![],
            num_strokes
        }
    }
}

fn make_unistrokes(strokes:&Vec<Point>, orders:Vec<Vec<usize>>)-> Vec<Point>{
    let mut unistrokes = vec![]; // array of point arrays

    for r in 0..orders.len(){
        for b in 0..(2i32.pow(orders[r].len() as u32)){// use b's bits for directions
            let mut unistroke = vec![];
            for i in 0..orders[r].len(){
                let pts =
                if b>>i&1 == 1{// is b's bit at index i on?
                    strokes[orders[r][i]].reverse()
                }else{

                };
            }
        }
    }

    unistrokes
}

fn heap_permute(n:usize, order:&mut Vec<usize>, /*out*/ orders:&mut Vec<Vec<usize>>){
    if n==1{
        orders.push(order.clone()); // append copy
    }else{
        for i in 0..n{
            heap_permute(n-1, order, orders);
            if n%2==1{// swap 0, n-1
                let tmp = order[0];
                order[0] = order[n-1];
                order[n-1] = tmp;
            }else{// swap i, n-1
                let tmp = order[i];
                order[i] = order[n-1];
                order[n-1] = tmp;
            }
        }
    }
}

fn vectorize(points:&Vec<Point>, use_bounded_rotation_invariance:bool) -> Vec<f64>{ // for Protractor
	let mut cos = 1.0;
	let mut sin = 0.0;
	if use_bounded_rotation_invariance {
		let i_angle = points[0].y.atan2(points[0].x);
		let base_orientation = (PI / 4.0) * ((i_angle + PI / 8.0) / (PI / 4.0)).floor();
		cos = (base_orientation - i_angle).cos();
		sin = (base_orientation - i_angle).sin();
	}
	let mut sum = 0.0;
	let mut vector = vec![];
	for i in 0..points.len() {
		let new_x = points[i].x * cos - points[i].y * sin;
		let new_y = points[i].y * cos + points[i].x * sin;
		vector.push(new_x);
		vector.push(new_y);
		sum += new_x * new_x + new_y * new_y;
	}
	let magnitude = sum.sqrt();
	for i in 0..vector.len() {
        vector[i] /= magnitude;
    }
	vector
}

fn calc_start_unit_vector(points:&Vec<Point>, index: usize) -> Point{ // start angle from points[0] to points[index] normalized as a unit vector
	let v = Point::new(points[index].x - points[0].x, points[index].y - points[0].y);
	let len = (v.x * v.x + v.y * v.y).sqrt();
	Point::new(v.x / len, v.y / len)
}

fn translate_to(points: &Vec<Point>, pt: &Point) -> Vec<Point> {
    // translates points' centroid
    let c = centroid(points);
    let mut newpoints = vec![];
    for point in points {
        let qx = point.x + pt.x - c.x;
        let qy = point.y + pt.y - c.y;
        newpoints.push(Point::new(qx, qy));
    }
    newpoints
}

fn scale_dim_to(points:&Vec<Point>, size: f64, ratio1D: f64) -> Vec<Point>{ // scales bbox uniformly for 1D, non-uniformly for 2D
	let b = bounding_box(points);
	let uniformly = (b.width / b.height).min(b.height / b.width) <= ratio1D; // 1D or 2D gesture test
	let mut newpoints = vec![];
	for i in 0..points.len(){
		let qx = if uniformly { points[i].x * (size / b.width.max(b.height)) }else{ points[i].x * (size / b.width)};
		let qy = if uniformly { points[i].y * (size / b.width.max(b.height)) }else{ points[i].y * (size / b.height)};
		newpoints.push(Point::new(qx, qy));
	}
	newpoints
}

fn bounding_box(points: &Vec<Point>) -> Rectangle {
    let mut min_x = std::f64::MAX;
    let mut max_x = std::f64::MIN;
    let mut min_y = std::f64::MAX;
    let mut max_y = std::f64::MIN;
    for i in 0..points.len() {
        min_x = min_x.min(points[i].x);
        min_y = min_y.min(points[i].y);
        max_x = max_x.max(points[i].x);
        max_y = max_y.max(points[i].y);
    }
    Rectangle::new(min_x, min_y, max_x - min_x, max_y - min_y)
}

fn rotate_by(points: &Vec<Point>, radians: f64) -> Vec<Point> {
    // rotates points around centroid
    let c = centroid(points);
    let cos = radians.cos();
    let sin = radians.sin();
    let mut newpoints = vec![];
    for i in 0..points.len() {
        let qx = (points[i].x - c.x) * cos - (points[i].y - c.y) * sin + c.x;
        let qy = (points[i].x - c.x) * sin + (points[i].y - c.y) * cos + c.y;
        newpoints.push(Point::new(qx, qy));
    }
    return newpoints;
}

fn indicative_angle(points: &Vec<Point>) -> f64 {
    let c = centroid(points);
    (c.y - points[0].y).atan2(c.x - points[0].x)
}

fn centroid(points: &Vec<Point>) -> Point {
    let mut x = 0.0;
    let mut y = 0.0;
    for point in points {
        x += point.x;
        y += point.y;
    }
    x /= points.len() as f64;
    y /= points.len() as f64;
    Point::new(x, y)
}

fn resample(mut points: Vec<Point>, n: usize) -> Vec<Point> {
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