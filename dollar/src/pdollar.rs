use std;

use std::time::{Duration, Instant};
use std::f64::consts::PI;

const NUM_POINT_CLOUDS:usize = 16;
const NUM_POINTS:usize = 32;
const ORIGIN: Point = Point { x: 0.0, y: 0.0, id:0 };

//
// Result class
//
pub struct Result {
    pub name: String,
    pub score: f64,
    pub ms: f64,
}

impl Result {
    fn new(name: &str, score: f64, ms: f64) -> Result {
        // constructor
        Result {
            name: name.to_string(),
            score,
            ms,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub id: usize
}

impl Point {
    pub fn new<T: Into<f64>>(x: T, y: T, id: usize) -> Point {
        Point {
            x: x.into(),
            y: y.into(),
            id
        }
    }
}

#[derive(Debug, Clone)]
struct PointCloud{
    name: String,
    points: Vec<Point>
}

impl PointCloud{
    fn new(name:&str, points:Vec<Point>) ->PointCloud{
        let points = resample(points, NUM_POINTS);
        let points = scale(&points);
        PointCloud{
            name: name.to_string(),
            points: translate_to(&points, &ORIGIN)
        }
    }
}

impl Default for PointCloud {
    fn default() -> PointCloud {
        PointCloud {
            name: String::new(),
            points: vec![]
        }
    }
}

pub struct PDollarRecognizer{
    point_clouds: Vec<PointCloud>
}

impl PDollarRecognizer{
    pub fn new() -> PDollarRecognizer{
        let mut point_clouds = vec![];

        point_clouds.push(PointCloud::new("T", vec![
            Point::new(30,7,1),Point::new(103,7,1),
            Point::new(66,7,2),Point::new(66,87,2)
        ]));
        point_clouds.push(PointCloud::new("N", vec![
            Point::new(177,92,1),Point::new(177,2,1),
            Point::new(182,1,2),Point::new(246,95,2),
            Point::new(247,87,3),Point::new(247,1,3)
        ]));
        point_clouds.push(PointCloud::new("D", vec![
            Point::new(345,9,1),Point::new(345,87,1),
            Point::new(351,8,2),Point::new(363,8,2),Point::new(372,9,2),Point::new(380,11,2),Point::new(386,14,2),Point::new(391,17,2),Point::new(394,22,2),Point::new(397,28,2),Point::new(399,34,2),Point::new(400,42,2),Point::new(400,50,2),Point::new(400,56,2),Point::new(399,61,2),Point::new(397,66,2),Point::new(394,70,2),Point::new(391,74,2),Point::new(386,78,2),Point::new(382,81,2),Point::new(377,83,2),Point::new(372,85,2),Point::new(367,87,2),Point::new(360,87,2),Point::new(355,88,2),Point::new(349,87,2)
        ]));
        point_clouds.push(PointCloud::new("P", vec![
            Point::new(507,8,1),Point::new(507,87,1),
            Point::new(513,7,2),Point::new(528,7,2),Point::new(537,8,2),Point::new(544,10,2),Point::new(550,12,2),Point::new(555,15,2),Point::new(558,18,2),Point::new(560,22,2),Point::new(561,27,2),Point::new(562,33,2),Point::new(561,37,2),Point::new(559,42,2),Point::new(556,45,2),Point::new(550,48,2),Point::new(544,51,2),Point::new(538,53,2),Point::new(532,54,2),Point::new(525,55,2),Point::new(519,55,2),Point::new(513,55,2),Point::new(510,55,2)
        ]));
        point_clouds.push(PointCloud::new("X", vec![
            Point::new(30,146,1),Point::new(106,222,1),
            Point::new(30,225,2),Point::new(106,146,2)
        ]));
        point_clouds.push(PointCloud::new("H", vec![
            Point::new(188,137,1),Point::new(188,225,1),
            Point::new(188,180,2),Point::new(241,180,2),
            Point::new(241,137,3),Point::new(241,225,3)
        ]));
        point_clouds.push(PointCloud::new("I", vec![
            Point::new(371,149,1),Point::new(371,221,1),
            Point::new(341,149,2),Point::new(401,149,2),
            Point::new(341,221,3),Point::new(401,221,3)
        ]));
        point_clouds.push(PointCloud::new("exclamation", vec![
            Point::new(526,142,1),Point::new(526,204,1),
            Point::new(526,221,2)
        ]));
        point_clouds.push(PointCloud::new("line", vec![
            Point::new(12,347,1),Point::new(119,347,1)
        ]));
        point_clouds.push(PointCloud::new("five-point star", vec![
            Point::new(177,396,1),Point::new(223,299,1),Point::new(262,396,1),Point::new(168,332,1),Point::new(278,332,1),Point::new(184,397,1)
        ]));
        point_clouds.push(PointCloud::new("null", vec![
            Point::new(382,310,1),Point::new(377,308,1),Point::new(373,307,1),Point::new(366,307,1),Point::new(360,310,1),Point::new(356,313,1),Point::new(353,316,1),Point::new(349,321,1),Point::new(347,326,1),Point::new(344,331,1),Point::new(342,337,1),Point::new(341,343,1),Point::new(341,350,1),Point::new(341,358,1),Point::new(342,362,1),Point::new(344,366,1),Point::new(347,370,1),Point::new(351,374,1),Point::new(356,379,1),Point::new(361,382,1),Point::new(368,385,1),Point::new(374,387,1),Point::new(381,387,1),Point::new(390,387,1),Point::new(397,385,1),Point::new(404,382,1),Point::new(408,378,1),Point::new(412,373,1),Point::new(416,367,1),Point::new(418,361,1),Point::new(419,353,1),Point::new(418,346,1),Point::new(417,341,1),Point::new(416,336,1),Point::new(413,331,1),Point::new(410,326,1),Point::new(404,320,1),Point::new(400,317,1),Point::new(393,313,1),Point::new(392,312,1),
            Point::new(418,309,2),Point::new(337,390,2)
        ]));
        point_clouds.push(PointCloud::new("arrowhead", vec![
            Point::new(506,349,1),Point::new(574,349,1),
            Point::new(525,306,2),Point::new(584,349,2),Point::new(525,388,2)
        ]));
        point_clouds.push(PointCloud::new("pitchfork", vec![
            Point::new(38,470,1),Point::new(36,476,1),Point::new(36,482,1),Point::new(37,489,1),Point::new(39,496,1),Point::new(42,500,1),Point::new(46,503,1),Point::new(50,507,1),Point::new(56,509,1),Point::new(63,509,1),Point::new(70,508,1),Point::new(75,506,1),Point::new(79,503,1),Point::new(82,499,1),Point::new(85,493,1),Point::new(87,487,1),Point::new(88,480,1),Point::new(88,474,1),Point::new(87,468,1),
            Point::new(62,464,2),Point::new(62,571,2)
        ]));
        point_clouds.push(PointCloud::new("six-point star", vec![
            Point::new(177,554,1),Point::new(223,476,1),Point::new(268,554,1),Point::new(183,554,1),
            Point::new(177,490,2),Point::new(223,568,2),Point::new(268,490,2),Point::new(183,490,2)
        ]));
        point_clouds.push(PointCloud::new("asterisk", vec![
            Point::new(325,499,1),Point::new(417,557,1),
            Point::new(417,499,2),Point::new(325,557,2),
            Point::new(371,486,3),Point::new(371,571,3)
        ]));
        point_clouds.push(PointCloud::new("half-note", vec![
            Point::new(546,465,1),Point::new(546,531,1),
            Point::new(540,530,2),Point::new(536,529,2),Point::new(533,528,2),Point::new(529,529,2),Point::new(524,530,2),Point::new(520,532,2),Point::new(515,535,2),Point::new(511,539,2),Point::new(508,545,2),Point::new(506,548,2),Point::new(506,554,2),Point::new(509,558,2),Point::new(512,561,2),Point::new(517,564,2),Point::new(521,564,2),Point::new(527,563,2),Point::new(531,560,2),Point::new(535,557,2),Point::new(538,553,2),Point::new(542,548,2),Point::new(544,544,2),Point::new(546,540,2),Point::new(546,536,2)
        ]));

        PDollarRecognizer{ point_clouds }
    }

    pub fn recognize(&self, points: Vec<Point>) -> Result{
        let t0 = Instant::now();
        let points = translate_to(&scale(&resample(points, NUM_POINTS)), &ORIGIN);

        let mut b = std::f64::MAX;
        let mut u = -1;
        for i in 0..self.point_clouds.len(){ // for each point-cloud template
            let d = greedy_cloud_match(&points, &self.point_clouds[i]);
            if d<b{
                b = d; // best (least) distance
                u = i as i32; // point-cloud index
            }
        }

        let t1 = duration_to_milis(&t0.elapsed());

        if u == -1 {
            Result::new("No match.", 0.0, t1)
        } else {
            Result::new(
                &self.point_clouds[u as usize].name,
                ((b-2.0)/-2.0).max(0.0),
                t1,
            )
        }
    }

    pub fn add_gesture(&mut self, name:&str, points:Vec<Point>) -> usize{
        self.point_clouds.push(PointCloud::new(name, points));
        let mut num = 0;
        for i in 0..self.point_clouds.len() {
            if self.point_clouds[i].name == name {
                num += 1;
            }
        }
        num
    }

    pub fn delete_user_gestures(&mut self) -> usize {
        self.point_clouds.resize(NUM_POINT_CLOUDS, PointCloud::default());
        NUM_POINT_CLOUDS
    }

    pub fn point_clouds(&self) -> &Vec<PointCloud>{
        &self.point_clouds
    }
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}

fn greedy_cloud_match(points:&Vec<Point>, p:&PointCloud) -> f64{
    let e = 0.50;
    let step = (points.len() as f64).powf(1.0-e).floor() as usize;
    let mut min = std::f64::MAX;
    for i in (0..points.len()).step_by(step){
        let d1 = cloud_distance(&points, &p.points, i);
        let d2 = cloud_distance(&p.points, &points, i);
        min = min.min(d1.min(d2));
    }
    min
}

fn cloud_distance(pts1:&Vec<Point>, pts2:&Vec<Point>, start: usize) -> f64{
    let mut matched = vec![false; pts1.len()];
    let mut sum = 0.0;
    let mut i = start;

    while{
        let mut index = -1;
        let mut min = std::f64::MAX;
        for j in 0..matched.len(){
            if !matched[j]{
                let d = distance(&pts1[i], &pts2[j]);
                if d<min{
                    min = d;
                    index = j as i32;
                }
            }
        }
        matched[index as usize] = true;
        let weight = 1 - ((i - start + pts1.len()) % pts1.len()) / pts1.len();
        sum += weight as f64 * min;
        i = (i + 1) % pts1.len();

        i != start
    }{}

    sum
}

fn translate_to(points: &Vec<Point>, pt: &Point) -> Vec<Point> {
    // translates points' centroid
    let c = centroid(points);
    let mut newpoints = vec![];
    for point in points {
        let qx = point.x + pt.x - c.x;
        let qy = point.y + pt.y - c.y;
        newpoints.push(Point::new(qx, qy, point.id));
    }
    newpoints
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
    Point::new(x, y, 0)
}

fn scale(points: &Vec<Point>) -> Vec<Point>{
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
    let size = (max_x-min_x).max(max_y-min_y);
    let mut new_points = vec![];
    for i in 0..points.len(){
        let qx = (points[i].x - min_x)/size;
        let qy = (points[i].y - min_y)/size;
        new_points.push(Point::new(qx, qy, points[i].id));
    }
    new_points
}

fn resample(mut points: Vec<Point>, n: usize) -> Vec<Point>{
    let len = path_length(&points)/(n as f64-1.0);// interval length
    let mut dist = 0.0;
    let mut new_points = vec![points[0].clone()];

    let mut i = 1;
    while i < points.len() {
        if points[i].id == points[i-1].id{
            let d = distance(&points[i - 1], &points[i]);
            if (dist + d) >= len {
                let qx = points[i - 1].x + ((len - dist) / d) * (points[i].x - points[i - 1].x);
                let qy = points[i - 1].y + ((len - dist) / d) * (points[i].y - points[i - 1].y);
                let q = Point::new(qx, qy, points[i].id);
                new_points.push(q.clone()); // append Point::new 'q'
                points.insert(i, q); // insert 'q' at position i in points s.t. 'q' will be the next i
                dist = 0.0;
            } else {
                dist += d;
            }
        }
        i += 1;
    }
    if new_points.len() == n as usize - 1 {
        // somtimes we fall a rounding-error short of adding the last point, so add it if so
        new_points.push(Point::new(
            points[points.len() - 1].x,
            points[points.len() - 1].y,
            points[points.len() - 1].id
        ));
    }

    new_points
}

// length traversed by a point path
fn path_length(points:&Vec<Point>) -> f64{
    let mut d = 0.0;
    for i in 1..points.len(){
        if points[i].id == points[i-1].id{
            d += distance(&points[i-1], &points[i]);
        }
    }
    d
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    (dx * dx + dy * dy).sqrt()
}