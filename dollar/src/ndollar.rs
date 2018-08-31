use std;

use std::f64::consts::PI;
use std::time::{Duration, Instant};

/*
二维多笔画识别器，专为基于手势的用户界面的快速原型设计而设计。
NDollarRecognizer建立在DollarRecognizer识别器上
use_protractor 可提高识别速度
*/

const NUM_UNISTROKES: usize = 16;
const NUM_POINTS: usize = 96;
const SQUARE_SIZE: f64 = 250.0;
const ONE_D_THRESHOLD: f64 = 0.25; // customize to desired gesture set (usually 0.20 - 0.35)
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
const START_ANGLE_INDEX: usize = NUM_POINTS / 8; // eighth of gesture length
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
        if use_bounded_rotation_invariance {
            points = rotate_by(&points, radians); //restore
        }
        points = translate_to(&points, &ORIGIN);
        let start_unit_vector = calc_start_unit_vector(&points, START_ANGLE_INDEX);
        let vector = vectorize(&points, use_bounded_rotation_invariance); // for Protractor

        Unistroke {
            name: name.to_string(),
            points,
            start_unit_vector,
            vector,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Multistroke {
    name: String,
    num_strokes: usize,
    unistrokes: Vec<Unistroke>,
}

impl Multistroke {
    fn new(
        name: &str,
        use_bounded_rotation_invariance: bool,
        strokes: Vec<Vec<Point>>,
    ) -> Multistroke {
        let num_strokes = strokes.len(); // number of individual strokes
        let mut order = vec![];
        for i in 0..strokes.len() {
            order.push(i); // initialize
        }
        let mut orders = vec![]; // array of integer arrays
        heap_permute(strokes.len(), &mut order, &mut orders);
        let unistrokes = make_unistrokes(&strokes, orders);

        Multistroke {
            name: name.to_string(),
            unistrokes: unistrokes
                .iter()
                .map(|points| Unistroke::new(name, use_bounded_rotation_invariance, points.clone()))
                .collect(),
            num_strokes,
        }
    }
}

impl Default for Multistroke {
    fn default() -> Multistroke {
        Multistroke {
            name: String::new(),
            num_strokes: 0,
            unistrokes: vec![],
        }
    }
}

pub struct NDollarRecognizer {
    multistrokes: Vec<Multistroke>,
}

impl NDollarRecognizer {
    pub fn new(use_bounded_rotation_invariance: bool) -> NDollarRecognizer {
        let mut multistrokes = vec![];

        multistrokes.push(Multistroke::new(
            "T",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(30, 7), Point::new(103, 7)],
                vec![Point::new(66, 7), Point::new(66, 87)],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "N",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(177, 92), Point::new(177, 2)],
                vec![Point::new(182, 1), Point::new(246, 95)],
                vec![Point::new(247, 87), Point::new(247, 1)],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "D",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(345, 9), Point::new(345, 87)],
                vec![
                    Point::new(351, 8),
                    Point::new(363, 8),
                    Point::new(372, 9),
                    Point::new(380, 11),
                    Point::new(386, 14),
                    Point::new(391, 17),
                    Point::new(394, 22),
                    Point::new(397, 28),
                    Point::new(399, 34),
                    Point::new(400, 42),
                    Point::new(400, 50),
                    Point::new(400, 56),
                    Point::new(399, 61),
                    Point::new(397, 66),
                    Point::new(394, 70),
                    Point::new(391, 74),
                    Point::new(386, 78),
                    Point::new(382, 81),
                    Point::new(377, 83),
                    Point::new(372, 85),
                    Point::new(367, 87),
                    Point::new(360, 87),
                    Point::new(355, 88),
                    Point::new(349, 87),
                ],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "P",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(507, 8), Point::new(507, 87)],
                vec![
                    Point::new(513, 7),
                    Point::new(528, 7),
                    Point::new(537, 8),
                    Point::new(544, 10),
                    Point::new(550, 12),
                    Point::new(555, 15),
                    Point::new(558, 18),
                    Point::new(560, 22),
                    Point::new(561, 27),
                    Point::new(562, 33),
                    Point::new(561, 37),
                    Point::new(559, 42),
                    Point::new(556, 45),
                    Point::new(550, 48),
                    Point::new(544, 51),
                    Point::new(538, 53),
                    Point::new(532, 54),
                    Point::new(525, 55),
                    Point::new(519, 55),
                    Point::new(513, 55),
                    Point::new(510, 55),
                ],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "X",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(30, 146), Point::new(106, 222)],
                vec![Point::new(30, 225), Point::new(106, 146)],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "H",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(188, 137), Point::new(188, 225)],
                vec![Point::new(188, 180), Point::new(241, 180)],
                vec![Point::new(241, 137), Point::new(241, 225)],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "I",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(371, 149), Point::new(371, 221)],
                vec![Point::new(341, 149), Point::new(401, 149)],
                vec![Point::new(341, 221), Point::new(401, 221)],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "exclamation",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(526, 142), Point::new(526, 204)],
                vec![Point::new(526, 221)],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "line",
            use_bounded_rotation_invariance,
            vec![vec![Point::new(12, 347), Point::new(119, 347)]],
        ));
        multistrokes.push(Multistroke::new(
            "five-point star",
            use_bounded_rotation_invariance,
            vec![vec![
                Point::new(177, 396),
                Point::new(223, 299),
                Point::new(262, 396),
                Point::new(168, 332),
                Point::new(278, 332),
                Point::new(184, 397),
            ]],
        ));
        multistrokes.push(Multistroke::new(
            "null",
            use_bounded_rotation_invariance,
            vec![
                vec![
                    Point::new(382, 310),
                    Point::new(377, 308),
                    Point::new(373, 307),
                    Point::new(366, 307),
                    Point::new(360, 310),
                    Point::new(356, 313),
                    Point::new(353, 316),
                    Point::new(349, 321),
                    Point::new(347, 326),
                    Point::new(344, 331),
                    Point::new(342, 337),
                    Point::new(341, 343),
                    Point::new(341, 350),
                    Point::new(341, 358),
                    Point::new(342, 362),
                    Point::new(344, 366),
                    Point::new(347, 370),
                    Point::new(351, 374),
                    Point::new(356, 379),
                    Point::new(361, 382),
                    Point::new(368, 385),
                    Point::new(374, 387),
                    Point::new(381, 387),
                    Point::new(390, 387),
                    Point::new(397, 385),
                    Point::new(404, 382),
                    Point::new(408, 378),
                    Point::new(412, 373),
                    Point::new(416, 367),
                    Point::new(418, 361),
                    Point::new(419, 353),
                    Point::new(418, 346),
                    Point::new(417, 341),
                    Point::new(416, 336),
                    Point::new(413, 331),
                    Point::new(410, 326),
                    Point::new(404, 320),
                    Point::new(400, 317),
                    Point::new(393, 313),
                    Point::new(392, 312),
                ],
                vec![Point::new(418, 309), Point::new(337, 390)],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "arrowhead",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(506, 349), Point::new(574, 349)],
                vec![
                    Point::new(525, 306),
                    Point::new(584, 349),
                    Point::new(525, 388),
                ],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "pitchfork",
            use_bounded_rotation_invariance,
            vec![
                vec![
                    Point::new(38, 470),
                    Point::new(36, 476),
                    Point::new(36, 482),
                    Point::new(37, 489),
                    Point::new(39, 496),
                    Point::new(42, 500),
                    Point::new(46, 503),
                    Point::new(50, 507),
                    Point::new(56, 509),
                    Point::new(63, 509),
                    Point::new(70, 508),
                    Point::new(75, 506),
                    Point::new(79, 503),
                    Point::new(82, 499),
                    Point::new(85, 493),
                    Point::new(87, 487),
                    Point::new(88, 480),
                    Point::new(88, 474),
                    Point::new(87, 468),
                ],
                vec![Point::new(62, 464), Point::new(62, 571)],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "six-point star",
            use_bounded_rotation_invariance,
            vec![
                vec![
                    Point::new(177, 554),
                    Point::new(223, 476),
                    Point::new(268, 554),
                    Point::new(183, 554),
                ],
                vec![
                    Point::new(177, 490),
                    Point::new(223, 568),
                    Point::new(268, 490),
                    Point::new(183, 490),
                ],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "asterisk",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(325, 499), Point::new(417, 557)],
                vec![Point::new(417, 499), Point::new(325, 557)],
                vec![Point::new(371, 486), Point::new(371, 571)],
            ],
        ));
        multistrokes.push(Multistroke::new(
            "half-note",
            use_bounded_rotation_invariance,
            vec![
                vec![Point::new(546, 465), Point::new(546, 531)],
                vec![
                    Point::new(540, 530),
                    Point::new(536, 529),
                    Point::new(533, 528),
                    Point::new(529, 529),
                    Point::new(524, 530),
                    Point::new(520, 532),
                    Point::new(515, 535),
                    Point::new(511, 539),
                    Point::new(508, 545),
                    Point::new(506, 548),
                    Point::new(506, 554),
                    Point::new(509, 558),
                    Point::new(512, 561),
                    Point::new(517, 564),
                    Point::new(521, 564),
                    Point::new(527, 563),
                    Point::new(531, 560),
                    Point::new(535, 557),
                    Point::new(538, 553),
                    Point::new(542, 548),
                    Point::new(544, 544),
                    Point::new(546, 540),
                    Point::new(546, 536),
                ],
            ],
        ));

        NDollarRecognizer { multistrokes }
    }

    pub fn multistrokes(&self) -> &Vec<Multistroke> {
        &self.multistrokes
    }
}

impl NDollarRecognizer {
    pub fn recognize(
        &self,
        strokes: Vec<Vec<Point>>,
        use_bounded_rotation_invariance: bool,
        require_same_no_of_strokes: bool,
        use_protractor: bool,
    ) -> Result {
        let t0 = Instant::now();
        let points = combine_strokes(&strokes);
        let points = resample(points, NUM_POINTS);
        let radians = indicative_angle(&points);
        let points = rotate_by(&points, -radians);
        let mut points = scale_dim_to(&points, SQUARE_SIZE, ONE_D_THRESHOLD);
        if use_bounded_rotation_invariance {
            points = rotate_by(&points, radians); //restore
        }
        let points = translate_to(&points, &ORIGIN);
        let startv = calc_start_unit_vector(&points, START_ANGLE_INDEX);
        let vector = vectorize(&points, use_bounded_rotation_invariance); // for Protractor

        let mut b = std::f64::MAX;
        let mut u = -1;

        for i in 0..self.multistrokes.len() {
            if !require_same_no_of_strokes || strokes.len() == self.multistrokes[i].num_strokes {
                // optional -- only attempt match when same # of component strokes
                for j in 0..self.multistrokes[i].unistrokes.len() {
                    if angle_between_unit_vectors(
                        &startv,
                        &self.multistrokes[i].unistrokes[j].start_unit_vector,
                    ) <= ANGLE_SIMILARITY_THRESHOLD
                    {
                        // strokes start in the same direction
                        let d = if use_protractor {
                            // for Protractor
                            optimal_cosine_distance(
                                &self.multistrokes[i].unistrokes[j].vector,
                                &vector,
                            )
                        } else {
                            // Golden Section Search (original $N)
                            distance_at_best_angle(
                                &points,
                                &self.multistrokes[i].unistrokes[j],
                                -ANGLE_RANGE,
                                ANGLE_RANGE,
                                ANGLE_PRECISION,
                            )
                        };

                        if d < b {
                            b = d;
                            u = i as i32;
                        }
                    }
                }
            }
        }

        let t1 = duration_to_milis(&t0.elapsed());

        if u == -1 {
            Result::new("No match.", 0.0, t1)
        } else {
            Result::new(
                &self.multistrokes[u as usize].name,
                if use_protractor {
                    1.0 / b
                } else {
                    1.0 - b / HALF_DIAGONAL
                },
                t1,
            )
        }
    }

    pub fn add_gesture(
        &mut self,
        name: &str,
        use_bounded_rotation_invariance: bool,
        strokes: Vec<Vec<Point>>,
    ) -> usize {
        self.multistrokes.push(Multistroke::new(
            name,
            use_bounded_rotation_invariance,
            strokes,
        ));
        let mut num = 0;
        for i in 0..self.multistrokes.len() {
            if self.multistrokes[i].name == name {
                num += 1;
            }
        }
        num
    }

    pub fn delete_user_gestures(&mut self) -> usize {
        self.multistrokes
            .resize(NUM_UNISTROKES, Multistroke::default());
        NUM_UNISTROKES
    }
}

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

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}

fn distance_at_best_angle(
    points: &Vec<Point>,
    t: &Unistroke,
    mut a: f64,
    mut b: f64,
    threshold: f64,
) -> f64 {
    let mut x1 = PHI * a + (1.0 - PHI) * b;
    let mut f1 = distance_at_angle(points, t, x1);
    let mut x2 = (1.0 - PHI) * a + PHI * b;
    let mut f2 = distance_at_angle(points, t, x2);
    while (b - a).abs() > threshold {
        if f1 < f2 {
            b = x2;
            x2 = x1;
            f2 = f1;
            x1 = PHI * a + (1.0 - PHI) * b;
            f1 = distance_at_angle(points, t, x1);
        } else {
            a = x1;
            x1 = x2;
            f1 = f2;
            x2 = (1.0 - PHI) * a + PHI * b;
            f2 = distance_at_angle(points, t, x2);
        }
    }
    f1.min(f2)
}

fn distance_at_angle(points: &Vec<Point>, t: &Unistroke, radians: f64) -> f64 {
    let newpoints = rotate_by(points, radians);
    path_distance(&newpoints, &t.points)
}

fn path_distance(pts1: &Vec<Point>, pts2: &Vec<Point>) -> f64 {
    let mut d = 0.0;
    for i in 0..pts1.len() {
        // assumes pts1.length == pts2.length
        d += distance(&pts1[i], &pts2[i]);
    }
    d / pts1.len() as f64
}

fn optimal_cosine_distance(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    // for Protractor
    let mut a = 0.0;
    let mut b = 0.0;
    for i in (0..v1.len()).step_by(2) {
        a += v1[i] * v2[i] + v1[i + 1] * v2[i + 1];
        b += v1[i] * v2[i + 1] - v1[i + 1] * v2[i];
    }
    let angle = (b / a).atan();
    (a * angle.cos() + b * angle.sin()).acos()
}

// gives acute angle between unit vectors from (0,0) to v1, and (0,0) to v2
fn angle_between_unit_vectors(v1: &Point, v2: &Point) -> f64 {
    let n = v1.x * v2.x + v1.y * v2.y;
    let c = (-1.0f64).max(1.0f64.min(n)); // ensure [-1,+1]
    c.acos() // arc cosine of the vector dot product
}

fn combine_strokes(strokes: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut points = vec![];

    for stroke in strokes {
        for point in stroke {
            points.push(point.clone());
        }
    }
    points
}

fn make_unistrokes(strokes: &Vec<Vec<Point>>, orders: Vec<Vec<usize>>) -> Vec<Vec<Point>> {
    let mut unistrokes = vec![]; // array of point arrays

    for r in 0..orders.len() {
        for b in 0..(2i32.pow(orders[r].len() as u32)) {
            // use b's bits for directions
            let mut unistroke = vec![];
            for i in 0..orders[r].len() {
                let pts = if b >> i & 1 == 1 {
                    // is b's bit at index i on?
                    strokes[orders[r][i]].iter().rev().cloned().collect()
                } else {
                    strokes[orders[r][i]].clone()
                };
                for p in 0..pts.len() {
                    unistroke.push(pts[p].clone());
                }
            }
            unistrokes.push(unistroke);
        }
    }
    unistrokes
}

fn heap_permute(n: usize, order: &mut Vec<usize>, /*out*/ orders: &mut Vec<Vec<usize>>) {
    if n == 1 {
        orders.push(order.clone()); // append copy
    } else {
        for i in 0..n {
            heap_permute(n - 1, order, orders);
            if n % 2 == 1 {
                // swap 0, n-1
                let tmp = order[0];
                order[0] = order[n - 1];
                order[n - 1] = tmp;
            } else {
                // swap i, n-1
                let tmp = order[i];
                order[i] = order[n - 1];
                order[n - 1] = tmp;
            }
        }
    }
}

fn vectorize(points: &Vec<Point>, use_bounded_rotation_invariance: bool) -> Vec<f64> {
    // for Protractor
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

fn calc_start_unit_vector(points: &Vec<Point>, index: usize) -> Point {
    // start angle from points[0] to points[index] normalized as a unit vector
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

fn scale_dim_to(points: &Vec<Point>, size: f64, ratio1D: f64) -> Vec<Point> {
    // scales bbox uniformly for 1D, non-uniformly for 2D
    let b = bounding_box(points);
    let uniformly = (b.width / b.height).min(b.height / b.width) <= ratio1D; // 1D or 2D gesture test
    let mut newpoints = vec![];
    for i in 0..points.len() {
        let qx = if uniformly {
            points[i].x * (size / b.width.max(b.height))
        } else {
            points[i].x * (size / b.width)
        };
        let qy = if uniformly {
            points[i].y * (size / b.width.max(b.height))
        } else {
            points[i].y * (size / b.height)
        };
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
