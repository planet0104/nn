use std;
use std::time::{Duration, Instant};

/*
 二维单笔画识别器，专为基于手势的用户界面的快速原型设计而设计
 use_protractor 可提高识别速度
*/

// DollarRecognizer constants
//

const NUM_UNISTROKES: usize = 16;
const NUM_POINTS: usize = 64;
const SQUARE_SIZE: f64 = 250.0;
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
    vector: Vec<f64>,
}

impl Unistroke {
    fn new(name: &str, points: Vec<Point>) -> Unistroke {
        let points = resample(points, NUM_POINTS);
        let radians = indicative_angle(&points);
        let points = rotate_by(&points, -radians);
        let points = scale_to(&points, SQUARE_SIZE);
        let points = translate_to(&points, &ORIGIN);
        let vector = vectorize(&points);

        Unistroke {
            name: name.to_string(),
            points,
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
        }
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

//
// DollarRecognizer class
//
pub struct DollarRecognizer {
    unistrokes: Vec<Unistroke>,
}

impl DollarRecognizer {
    pub fn new() -> DollarRecognizer {
        // constructor
        DollarRecognizer { unistrokes: vec![] }
    }

    pub fn unistrokes(&self) -> &Vec<Unistroke> {
        &self.unistrokes
    }

    pub fn delete_user_gestures(&mut self) -> usize {
        self.unistrokes.resize(NUM_UNISTROKES, Unistroke::default());
        NUM_UNISTROKES
    }

    pub fn add_gesture(&mut self, name: &str, points: Vec<Point>) -> usize {
        self.unistrokes.push(Unistroke::new(name, points)); // append new unistroke
        let mut num = 0;
        for i in 0..self.unistrokes.len() {
            if self.unistrokes[i].name == name {
                num += 1;
            }
        }
        num
    }

    pub fn init_demo_gesture(&mut self){
        self.unistrokes.clear();
        self.unistrokes.push(Unistroke::new(
            "triangle",
            vec![
                Point::new(137, 139),
                Point::new(135, 141),
                Point::new(133, 144),
                Point::new(132, 146),
                Point::new(130, 149),
                Point::new(128, 151),
                Point::new(126, 155),
                Point::new(123, 160),
                Point::new(120, 166),
                Point::new(116, 171),
                Point::new(112, 177),
                Point::new(107, 183),
                Point::new(102, 188),
                Point::new(100, 191),
                Point::new(95, 195),
                Point::new(90, 199),
                Point::new(86, 203),
                Point::new(82, 206),
                Point::new(80, 209),
                Point::new(75, 213),
                Point::new(73, 213),
                Point::new(70, 216),
                Point::new(67, 219),
                Point::new(64, 221),
                Point::new(61, 223),
                Point::new(60, 225),
                Point::new(62, 226),
                Point::new(65, 225),
                Point::new(67, 226),
                Point::new(74, 226),
                Point::new(77, 227),
                Point::new(85, 229),
                Point::new(91, 230),
                Point::new(99, 231),
                Point::new(108, 232),
                Point::new(116, 233),
                Point::new(125, 233),
                Point::new(134, 234),
                Point::new(145, 233),
                Point::new(153, 232),
                Point::new(160, 233),
                Point::new(170, 234),
                Point::new(177, 235),
                Point::new(179, 236),
                Point::new(186, 237),
                Point::new(193, 238),
                Point::new(198, 239),
                Point::new(200, 237),
                Point::new(202, 239),
                Point::new(204, 238),
                Point::new(206, 234),
                Point::new(205, 230),
                Point::new(202, 222),
                Point::new(197, 216),
                Point::new(192, 207),
                Point::new(186, 198),
                Point::new(179, 189),
                Point::new(174, 183),
                Point::new(170, 178),
                Point::new(164, 171),
                Point::new(161, 168),
                Point::new(154, 160),
                Point::new(148, 155),
                Point::new(143, 150),
                Point::new(138, 148),
                Point::new(136, 148),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "x",
            vec![
                Point::new(87, 142),
                Point::new(89, 145),
                Point::new(91, 148),
                Point::new(93, 151),
                Point::new(96, 155),
                Point::new(98, 157),
                Point::new(100, 160),
                Point::new(102, 162),
                Point::new(106, 167),
                Point::new(108, 169),
                Point::new(110, 171),
                Point::new(115, 177),
                Point::new(119, 183),
                Point::new(123, 189),
                Point::new(127, 193),
                Point::new(129, 196),
                Point::new(133, 200),
                Point::new(137, 206),
                Point::new(140, 209),
                Point::new(143, 212),
                Point::new(146, 215),
                Point::new(151, 220),
                Point::new(153, 222),
                Point::new(155, 223),
                Point::new(157, 225),
                Point::new(158, 223),
                Point::new(157, 218),
                Point::new(155, 211),
                Point::new(154, 208),
                Point::new(152, 200),
                Point::new(150, 189),
                Point::new(148, 179),
                Point::new(147, 170),
                Point::new(147, 158),
                Point::new(147, 148),
                Point::new(147, 141),
                Point::new(147, 136),
                Point::new(144, 135),
                Point::new(142, 137),
                Point::new(140, 139),
                Point::new(135, 145),
                Point::new(131, 152),
                Point::new(124, 163),
                Point::new(116, 177),
                Point::new(108, 191),
                Point::new(100, 206),
                Point::new(94, 217),
                Point::new(91, 222),
                Point::new(89, 225),
                Point::new(87, 226),
                Point::new(87, 224),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "rectangle",
            vec![
                Point::new(78, 149),
                Point::new(78, 153),
                Point::new(78, 157),
                Point::new(78, 160),
                Point::new(79, 162),
                Point::new(79, 164),
                Point::new(79, 167),
                Point::new(79, 169),
                Point::new(79, 173),
                Point::new(79, 178),
                Point::new(79, 183),
                Point::new(80, 189),
                Point::new(80, 193),
                Point::new(80, 198),
                Point::new(80, 202),
                Point::new(81, 208),
                Point::new(81, 210),
                Point::new(81, 216),
                Point::new(82, 222),
                Point::new(82, 224),
                Point::new(82, 227),
                Point::new(83, 229),
                Point::new(83, 231),
                Point::new(85, 230),
                Point::new(88, 232),
                Point::new(90, 233),
                Point::new(92, 232),
                Point::new(94, 233),
                Point::new(99, 232),
                Point::new(102, 233),
                Point::new(106, 233),
                Point::new(109, 234),
                Point::new(117, 235),
                Point::new(123, 236),
                Point::new(126, 236),
                Point::new(135, 237),
                Point::new(142, 238),
                Point::new(145, 238),
                Point::new(152, 238),
                Point::new(154, 239),
                Point::new(165, 238),
                Point::new(174, 237),
                Point::new(179, 236),
                Point::new(186, 235),
                Point::new(191, 235),
                Point::new(195, 233),
                Point::new(197, 233),
                Point::new(200, 233),
                Point::new(201, 235),
                Point::new(201, 233),
                Point::new(199, 231),
                Point::new(198, 226),
                Point::new(198, 220),
                Point::new(196, 207),
                Point::new(195, 195),
                Point::new(195, 181),
                Point::new(195, 173),
                Point::new(195, 163),
                Point::new(194, 155),
                Point::new(192, 145),
                Point::new(192, 143),
                Point::new(192, 138),
                Point::new(191, 135),
                Point::new(191, 133),
                Point::new(191, 130),
                Point::new(190, 128),
                Point::new(188, 129),
                Point::new(186, 129),
                Point::new(181, 132),
                Point::new(173, 131),
                Point::new(162, 131),
                Point::new(151, 132),
                Point::new(149, 132),
                Point::new(138, 132),
                Point::new(136, 132),
                Point::new(122, 131),
                Point::new(120, 131),
                Point::new(109, 130),
                Point::new(107, 130),
                Point::new(90, 132),
                Point::new(81, 133),
                Point::new(76, 133),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "circle",
            vec![
                Point::new(127, 141),
                Point::new(124, 140),
                Point::new(120, 139),
                Point::new(118, 139),
                Point::new(116, 139),
                Point::new(111, 140),
                Point::new(109, 141),
                Point::new(104, 144),
                Point::new(100, 147),
                Point::new(96, 152),
                Point::new(93, 157),
                Point::new(90, 163),
                Point::new(87, 169),
                Point::new(85, 175),
                Point::new(83, 181),
                Point::new(82, 190),
                Point::new(82, 195),
                Point::new(83, 200),
                Point::new(84, 205),
                Point::new(88, 213),
                Point::new(91, 216),
                Point::new(96, 219),
                Point::new(103, 222),
                Point::new(108, 224),
                Point::new(111, 224),
                Point::new(120, 224),
                Point::new(133, 223),
                Point::new(142, 222),
                Point::new(152, 218),
                Point::new(160, 214),
                Point::new(167, 210),
                Point::new(173, 204),
                Point::new(178, 198),
                Point::new(179, 196),
                Point::new(182, 188),
                Point::new(182, 177),
                Point::new(178, 167),
                Point::new(170, 150),
                Point::new(163, 138),
                Point::new(152, 130),
                Point::new(143, 129),
                Point::new(140, 131),
                Point::new(129, 136),
                Point::new(126, 139),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "check",
            vec![
                Point::new(91, 185),
                Point::new(93, 185),
                Point::new(95, 185),
                Point::new(97, 185),
                Point::new(100, 188),
                Point::new(102, 189),
                Point::new(104, 190),
                Point::new(106, 193),
                Point::new(108, 195),
                Point::new(110, 198),
                Point::new(112, 201),
                Point::new(114, 204),
                Point::new(115, 207),
                Point::new(117, 210),
                Point::new(118, 212),
                Point::new(120, 214),
                Point::new(121, 217),
                Point::new(122, 219),
                Point::new(123, 222),
                Point::new(124, 224),
                Point::new(126, 226),
                Point::new(127, 229),
                Point::new(129, 231),
                Point::new(130, 233),
                Point::new(129, 231),
                Point::new(129, 228),
                Point::new(129, 226),
                Point::new(129, 224),
                Point::new(129, 221),
                Point::new(129, 218),
                Point::new(129, 212),
                Point::new(129, 208),
                Point::new(130, 198),
                Point::new(132, 189),
                Point::new(134, 182),
                Point::new(137, 173),
                Point::new(143, 164),
                Point::new(147, 157),
                Point::new(151, 151),
                Point::new(155, 144),
                Point::new(161, 137),
                Point::new(165, 131),
                Point::new(171, 122),
                Point::new(174, 118),
                Point::new(176, 114),
                Point::new(177, 112),
                Point::new(177, 114),
                Point::new(175, 116),
                Point::new(173, 118),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "caret",
            vec![
                Point::new(79, 245),
                Point::new(79, 242),
                Point::new(79, 239),
                Point::new(80, 237),
                Point::new(80, 234),
                Point::new(81, 232),
                Point::new(82, 230),
                Point::new(84, 224),
                Point::new(86, 220),
                Point::new(86, 218),
                Point::new(87, 216),
                Point::new(88, 213),
                Point::new(90, 207),
                Point::new(91, 202),
                Point::new(92, 200),
                Point::new(93, 194),
                Point::new(94, 192),
                Point::new(96, 189),
                Point::new(97, 186),
                Point::new(100, 179),
                Point::new(102, 173),
                Point::new(105, 165),
                Point::new(107, 160),
                Point::new(109, 158),
                Point::new(112, 151),
                Point::new(115, 144),
                Point::new(117, 139),
                Point::new(119, 136),
                Point::new(119, 134),
                Point::new(120, 132),
                Point::new(121, 129),
                Point::new(122, 127),
                Point::new(124, 125),
                Point::new(126, 124),
                Point::new(129, 125),
                Point::new(131, 127),
                Point::new(132, 130),
                Point::new(136, 139),
                Point::new(141, 154),
                Point::new(145, 166),
                Point::new(151, 182),
                Point::new(156, 193),
                Point::new(157, 196),
                Point::new(161, 209),
                Point::new(162, 211),
                Point::new(167, 223),
                Point::new(169, 229),
                Point::new(170, 231),
                Point::new(173, 237),
                Point::new(176, 242),
                Point::new(177, 244),
                Point::new(179, 250),
                Point::new(181, 255),
                Point::new(182, 257),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "zig-zag",
            vec![
                Point::new(307, 216),
                Point::new(333, 186),
                Point::new(356, 215),
                Point::new(375, 186),
                Point::new(399, 216),
                Point::new(418, 186),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "arrow",
            vec![
                Point::new(68, 222),
                Point::new(70, 220),
                Point::new(73, 218),
                Point::new(75, 217),
                Point::new(77, 215),
                Point::new(80, 213),
                Point::new(82, 212),
                Point::new(84, 210),
                Point::new(87, 209),
                Point::new(89, 208),
                Point::new(92, 206),
                Point::new(95, 204),
                Point::new(101, 201),
                Point::new(106, 198),
                Point::new(112, 194),
                Point::new(118, 191),
                Point::new(124, 187),
                Point::new(127, 186),
                Point::new(132, 183),
                Point::new(138, 181),
                Point::new(141, 180),
                Point::new(146, 178),
                Point::new(154, 173),
                Point::new(159, 171),
                Point::new(161, 170),
                Point::new(166, 167),
                Point::new(168, 167),
                Point::new(171, 166),
                Point::new(174, 164),
                Point::new(177, 162),
                Point::new(180, 160),
                Point::new(182, 158),
                Point::new(183, 156),
                Point::new(181, 154),
                Point::new(178, 153),
                Point::new(171, 153),
                Point::new(164, 153),
                Point::new(160, 153),
                Point::new(150, 154),
                Point::new(147, 155),
                Point::new(141, 157),
                Point::new(137, 158),
                Point::new(135, 158),
                Point::new(137, 158),
                Point::new(140, 157),
                Point::new(143, 156),
                Point::new(151, 154),
                Point::new(160, 152),
                Point::new(170, 149),
                Point::new(179, 147),
                Point::new(185, 145),
                Point::new(192, 144),
                Point::new(196, 144),
                Point::new(198, 144),
                Point::new(200, 144),
                Point::new(201, 147),
                Point::new(199, 149),
                Point::new(194, 157),
                Point::new(191, 160),
                Point::new(186, 167),
                Point::new(180, 176),
                Point::new(177, 179),
                Point::new(171, 187),
                Point::new(169, 189),
                Point::new(165, 194),
                Point::new(164, 196),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "left square bracket",
            vec![
                Point::new(140, 124),
                Point::new(138, 123),
                Point::new(135, 122),
                Point::new(133, 123),
                Point::new(130, 123),
                Point::new(128, 124),
                Point::new(125, 125),
                Point::new(122, 124),
                Point::new(120, 124),
                Point::new(118, 124),
                Point::new(116, 125),
                Point::new(113, 125),
                Point::new(111, 125),
                Point::new(108, 124),
                Point::new(106, 125),
                Point::new(104, 125),
                Point::new(102, 124),
                Point::new(100, 123),
                Point::new(98, 123),
                Point::new(95, 124),
                Point::new(93, 123),
                Point::new(90, 124),
                Point::new(88, 124),
                Point::new(85, 125),
                Point::new(83, 126),
                Point::new(81, 127),
                Point::new(81, 129),
                Point::new(82, 131),
                Point::new(82, 134),
                Point::new(83, 138),
                Point::new(84, 141),
                Point::new(84, 144),
                Point::new(85, 148),
                Point::new(85, 151),
                Point::new(86, 156),
                Point::new(86, 160),
                Point::new(86, 164),
                Point::new(86, 168),
                Point::new(87, 171),
                Point::new(87, 175),
                Point::new(87, 179),
                Point::new(87, 182),
                Point::new(87, 186),
                Point::new(88, 188),
                Point::new(88, 195),
                Point::new(88, 198),
                Point::new(88, 201),
                Point::new(88, 207),
                Point::new(89, 211),
                Point::new(89, 213),
                Point::new(89, 217),
                Point::new(89, 222),
                Point::new(88, 225),
                Point::new(88, 229),
                Point::new(88, 231),
                Point::new(88, 233),
                Point::new(88, 235),
                Point::new(89, 237),
                Point::new(89, 240),
                Point::new(89, 242),
                Point::new(91, 241),
                Point::new(94, 241),
                Point::new(96, 240),
                Point::new(98, 239),
                Point::new(105, 240),
                Point::new(109, 240),
                Point::new(113, 239),
                Point::new(116, 240),
                Point::new(121, 239),
                Point::new(130, 240),
                Point::new(136, 237),
                Point::new(139, 237),
                Point::new(144, 238),
                Point::new(151, 237),
                Point::new(157, 236),
                Point::new(159, 237),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "right square bracket",
            vec![
                Point::new(112, 138),
                Point::new(112, 136),
                Point::new(115, 136),
                Point::new(118, 137),
                Point::new(120, 136),
                Point::new(123, 136),
                Point::new(125, 136),
                Point::new(128, 136),
                Point::new(131, 136),
                Point::new(134, 135),
                Point::new(137, 135),
                Point::new(140, 134),
                Point::new(143, 133),
                Point::new(145, 132),
                Point::new(147, 132),
                Point::new(149, 132),
                Point::new(152, 132),
                Point::new(153, 134),
                Point::new(154, 137),
                Point::new(155, 141),
                Point::new(156, 144),
                Point::new(157, 152),
                Point::new(158, 161),
                Point::new(160, 170),
                Point::new(162, 182),
                Point::new(164, 192),
                Point::new(166, 200),
                Point::new(167, 209),
                Point::new(168, 214),
                Point::new(168, 216),
                Point::new(169, 221),
                Point::new(169, 223),
                Point::new(169, 228),
                Point::new(169, 231),
                Point::new(166, 233),
                Point::new(164, 234),
                Point::new(161, 235),
                Point::new(155, 236),
                Point::new(147, 235),
                Point::new(140, 233),
                Point::new(131, 233),
                Point::new(124, 233),
                Point::new(117, 235),
                Point::new(114, 238),
                Point::new(112, 238),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "v",
            vec![
                Point::new(89, 164),
                Point::new(90, 162),
                Point::new(92, 162),
                Point::new(94, 164),
                Point::new(95, 166),
                Point::new(96, 169),
                Point::new(97, 171),
                Point::new(99, 175),
                Point::new(101, 178),
                Point::new(103, 182),
                Point::new(106, 189),
                Point::new(108, 194),
                Point::new(111, 199),
                Point::new(114, 204),
                Point::new(117, 209),
                Point::new(119, 214),
                Point::new(122, 218),
                Point::new(124, 222),
                Point::new(126, 225),
                Point::new(128, 228),
                Point::new(130, 229),
                Point::new(133, 233),
                Point::new(134, 236),
                Point::new(136, 239),
                Point::new(138, 240),
                Point::new(139, 242),
                Point::new(140, 244),
                Point::new(142, 242),
                Point::new(142, 240),
                Point::new(142, 237),
                Point::new(143, 235),
                Point::new(143, 233),
                Point::new(145, 229),
                Point::new(146, 226),
                Point::new(148, 217),
                Point::new(149, 208),
                Point::new(149, 205),
                Point::new(151, 196),
                Point::new(151, 193),
                Point::new(153, 182),
                Point::new(155, 172),
                Point::new(157, 165),
                Point::new(159, 160),
                Point::new(162, 155),
                Point::new(164, 150),
                Point::new(165, 148),
                Point::new(166, 146),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "delete",
            vec![
                Point::new(123, 129),
                Point::new(123, 131),
                Point::new(124, 133),
                Point::new(125, 136),
                Point::new(127, 140),
                Point::new(129, 142),
                Point::new(133, 148),
                Point::new(137, 154),
                Point::new(143, 158),
                Point::new(145, 161),
                Point::new(148, 164),
                Point::new(153, 170),
                Point::new(158, 176),
                Point::new(160, 178),
                Point::new(164, 183),
                Point::new(168, 188),
                Point::new(171, 191),
                Point::new(175, 196),
                Point::new(178, 200),
                Point::new(180, 202),
                Point::new(181, 205),
                Point::new(184, 208),
                Point::new(186, 210),
                Point::new(187, 213),
                Point::new(188, 215),
                Point::new(186, 212),
                Point::new(183, 211),
                Point::new(177, 208),
                Point::new(169, 206),
                Point::new(162, 205),
                Point::new(154, 207),
                Point::new(145, 209),
                Point::new(137, 210),
                Point::new(129, 214),
                Point::new(122, 217),
                Point::new(118, 218),
                Point::new(111, 221),
                Point::new(109, 222),
                Point::new(110, 219),
                Point::new(112, 217),
                Point::new(118, 209),
                Point::new(120, 207),
                Point::new(128, 196),
                Point::new(135, 187),
                Point::new(138, 183),
                Point::new(148, 167),
                Point::new(157, 153),
                Point::new(163, 145),
                Point::new(165, 142),
                Point::new(172, 133),
                Point::new(177, 127),
                Point::new(179, 127),
                Point::new(180, 125),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "left curly brace",
            vec![
                Point::new(150, 116),
                Point::new(147, 117),
                Point::new(145, 116),
                Point::new(142, 116),
                Point::new(139, 117),
                Point::new(136, 117),
                Point::new(133, 118),
                Point::new(129, 121),
                Point::new(126, 122),
                Point::new(123, 123),
                Point::new(120, 125),
                Point::new(118, 127),
                Point::new(115, 128),
                Point::new(113, 129),
                Point::new(112, 131),
                Point::new(113, 134),
                Point::new(115, 134),
                Point::new(117, 135),
                Point::new(120, 135),
                Point::new(123, 137),
                Point::new(126, 138),
                Point::new(129, 140),
                Point::new(135, 143),
                Point::new(137, 144),
                Point::new(139, 147),
                Point::new(141, 149),
                Point::new(140, 152),
                Point::new(139, 155),
                Point::new(134, 159),
                Point::new(131, 161),
                Point::new(124, 166),
                Point::new(121, 166),
                Point::new(117, 166),
                Point::new(114, 167),
                Point::new(112, 166),
                Point::new(114, 164),
                Point::new(116, 163),
                Point::new(118, 163),
                Point::new(120, 162),
                Point::new(122, 163),
                Point::new(125, 164),
                Point::new(127, 165),
                Point::new(129, 166),
                Point::new(130, 168),
                Point::new(129, 171),
                Point::new(127, 175),
                Point::new(125, 179),
                Point::new(123, 184),
                Point::new(121, 190),
                Point::new(120, 194),
                Point::new(119, 199),
                Point::new(120, 202),
                Point::new(123, 207),
                Point::new(127, 211),
                Point::new(133, 215),
                Point::new(142, 219),
                Point::new(148, 220),
                Point::new(151, 221),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "right curly brace",
            vec![
                Point::new(117, 132),
                Point::new(115, 132),
                Point::new(115, 129),
                Point::new(117, 129),
                Point::new(119, 128),
                Point::new(122, 127),
                Point::new(125, 127),
                Point::new(127, 127),
                Point::new(130, 127),
                Point::new(133, 129),
                Point::new(136, 129),
                Point::new(138, 130),
                Point::new(140, 131),
                Point::new(143, 134),
                Point::new(144, 136),
                Point::new(145, 139),
                Point::new(145, 142),
                Point::new(145, 145),
                Point::new(145, 147),
                Point::new(145, 149),
                Point::new(144, 152),
                Point::new(142, 157),
                Point::new(141, 160),
                Point::new(139, 163),
                Point::new(137, 166),
                Point::new(135, 167),
                Point::new(133, 169),
                Point::new(131, 172),
                Point::new(128, 173),
                Point::new(126, 176),
                Point::new(125, 178),
                Point::new(125, 180),
                Point::new(125, 182),
                Point::new(126, 184),
                Point::new(128, 187),
                Point::new(130, 187),
                Point::new(132, 188),
                Point::new(135, 189),
                Point::new(140, 189),
                Point::new(145, 189),
                Point::new(150, 187),
                Point::new(155, 186),
                Point::new(157, 185),
                Point::new(159, 184),
                Point::new(156, 185),
                Point::new(154, 185),
                Point::new(149, 185),
                Point::new(145, 187),
                Point::new(141, 188),
                Point::new(136, 191),
                Point::new(134, 191),
                Point::new(131, 192),
                Point::new(129, 193),
                Point::new(129, 195),
                Point::new(129, 197),
                Point::new(131, 200),
                Point::new(133, 202),
                Point::new(136, 206),
                Point::new(139, 211),
                Point::new(142, 215),
                Point::new(145, 220),
                Point::new(147, 225),
                Point::new(148, 231),
                Point::new(147, 239),
                Point::new(144, 244),
                Point::new(139, 248),
                Point::new(134, 250),
                Point::new(126, 253),
                Point::new(119, 253),
                Point::new(115, 253),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "star",
            vec![
                Point::new(75, 250),
                Point::new(75, 247),
                Point::new(77, 244),
                Point::new(78, 242),
                Point::new(79, 239),
                Point::new(80, 237),
                Point::new(82, 234),
                Point::new(82, 232),
                Point::new(84, 229),
                Point::new(85, 225),
                Point::new(87, 222),
                Point::new(88, 219),
                Point::new(89, 216),
                Point::new(91, 212),
                Point::new(92, 208),
                Point::new(94, 204),
                Point::new(95, 201),
                Point::new(96, 196),
                Point::new(97, 194),
                Point::new(98, 191),
                Point::new(100, 185),
                Point::new(102, 178),
                Point::new(104, 173),
                Point::new(104, 171),
                Point::new(105, 164),
                Point::new(106, 158),
                Point::new(107, 156),
                Point::new(107, 152),
                Point::new(108, 145),
                Point::new(109, 141),
                Point::new(110, 139),
                Point::new(112, 133),
                Point::new(113, 131),
                Point::new(116, 127),
                Point::new(117, 125),
                Point::new(119, 122),
                Point::new(121, 121),
                Point::new(123, 120),
                Point::new(125, 122),
                Point::new(125, 125),
                Point::new(127, 130),
                Point::new(128, 133),
                Point::new(131, 143),
                Point::new(136, 153),
                Point::new(140, 163),
                Point::new(144, 172),
                Point::new(145, 175),
                Point::new(151, 189),
                Point::new(156, 201),
                Point::new(161, 213),
                Point::new(166, 225),
                Point::new(169, 233),
                Point::new(171, 236),
                Point::new(174, 243),
                Point::new(177, 247),
                Point::new(178, 249),
                Point::new(179, 251),
                Point::new(180, 253),
                Point::new(180, 255),
                Point::new(179, 257),
                Point::new(177, 257),
                Point::new(174, 255),
                Point::new(169, 250),
                Point::new(164, 247),
                Point::new(160, 245),
                Point::new(149, 238),
                Point::new(138, 230),
                Point::new(127, 221),
                Point::new(124, 220),
                Point::new(112, 212),
                Point::new(110, 210),
                Point::new(96, 201),
                Point::new(84, 195),
                Point::new(74, 190),
                Point::new(64, 182),
                Point::new(55, 175),
                Point::new(51, 172),
                Point::new(49, 170),
                Point::new(51, 169),
                Point::new(56, 169),
                Point::new(66, 169),
                Point::new(78, 168),
                Point::new(92, 166),
                Point::new(107, 164),
                Point::new(123, 161),
                Point::new(140, 162),
                Point::new(156, 162),
                Point::new(171, 160),
                Point::new(173, 160),
                Point::new(186, 160),
                Point::new(195, 160),
                Point::new(198, 161),
                Point::new(203, 163),
                Point::new(208, 163),
                Point::new(206, 164),
                Point::new(200, 167),
                Point::new(187, 172),
                Point::new(174, 179),
                Point::new(172, 181),
                Point::new(153, 192),
                Point::new(137, 201),
                Point::new(123, 211),
                Point::new(112, 220),
                Point::new(99, 229),
                Point::new(90, 237),
                Point::new(80, 244),
                Point::new(73, 250),
                Point::new(69, 254),
                Point::new(69, 252),
            ],
        ));
        self.unistrokes.push(Unistroke::new(
            "pigtail",
            vec![
                Point::new(81, 219),
                Point::new(84, 218),
                Point::new(86, 220),
                Point::new(88, 220),
                Point::new(90, 220),
                Point::new(92, 219),
                Point::new(95, 220),
                Point::new(97, 219),
                Point::new(99, 220),
                Point::new(102, 218),
                Point::new(105, 217),
                Point::new(107, 216),
                Point::new(110, 216),
                Point::new(113, 214),
                Point::new(116, 212),
                Point::new(118, 210),
                Point::new(121, 208),
                Point::new(124, 205),
                Point::new(126, 202),
                Point::new(129, 199),
                Point::new(132, 196),
                Point::new(136, 191),
                Point::new(139, 187),
                Point::new(142, 182),
                Point::new(144, 179),
                Point::new(146, 174),
                Point::new(148, 170),
                Point::new(149, 168),
                Point::new(151, 162),
                Point::new(152, 160),
                Point::new(152, 157),
                Point::new(152, 155),
                Point::new(152, 151),
                Point::new(152, 149),
                Point::new(152, 146),
                Point::new(149, 142),
                Point::new(148, 139),
                Point::new(145, 137),
                Point::new(141, 135),
                Point::new(139, 135),
                Point::new(134, 136),
                Point::new(130, 140),
                Point::new(128, 142),
                Point::new(126, 145),
                Point::new(122, 150),
                Point::new(119, 158),
                Point::new(117, 163),
                Point::new(115, 170),
                Point::new(114, 175),
                Point::new(117, 184),
                Point::new(120, 190),
                Point::new(125, 199),
                Point::new(129, 203),
                Point::new(133, 208),
                Point::new(138, 213),
                Point::new(145, 215),
                Point::new(155, 218),
                Point::new(164, 219),
                Point::new(166, 219),
                Point::new(177, 219),
                Point::new(182, 218),
                Point::new(192, 216),
                Point::new(196, 213),
                Point::new(199, 212),
                Point::new(201, 211),
            ],
        ));
    }

    pub fn clear_gestures(&mut self){
        self.unistrokes.clear();
    }

    //
    // The $1 Gesture Recognizer API begins here -- 3 methods: Recognize(), AddGesture(), and DeleteUserGestures()
    //
    pub fn recognize(&self, points: Vec<Point>, use_protractor: bool) -> Result {
        let t0 = Instant::now();
        let points = resample(points, NUM_POINTS);
        let radians = indicative_angle(&points);
        let points = rotate_by(&points, -radians);
        let points = scale_to(&points, SQUARE_SIZE);
        let points = translate_to(&points, &ORIGIN);
        let vector = vectorize(&points); // for Protractor

        let mut b = std::f64::MAX;
        let mut u = -1;
        for i in 0..self.unistrokes.len() {
            // for each unistroke
            let d = if use_protractor {
                // for Protractor
                optimal_cosine_distance(&self.unistrokes[i].vector, &vector)
            } else {
                // Golden Section Search (original $1)
                distance_at_best_angle(
                    &points,
                    &self.unistrokes[i],
                    -ANGLE_RANGE,
                    ANGLE_RANGE,
                    ANGLE_PRECISION,
                )
            };
            if d < b {
                b = d; // best (least) distance
                u = i as i32; // unistroke index
            }
        }
        let t1 = duration_to_milis(&t0.elapsed());
        if u == -1 {
            Result::new("No match.", 0.0, t1)
        } else {
            Result::new(
                &self.unistrokes[u as usize].name,
                if use_protractor {
                    1.0 / b
                } else {
                    1.0 - b / HALF_DIAGONAL
                },
                t1,
            )
        }
    }
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
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

fn vectorize(points: &Vec<Point>) -> Vec<f64> {
    // for Protractor
    let mut sum = 0.0;
    let mut vector = vec![];
    for point in points {
        vector.push(point.x);
        vector.push(point.y);
        sum += point.x * point.x + point.y * point.y;
    }
    let magnitude = sum.sqrt();
    for i in 0..vector.len() {
        vector[i] /= magnitude;
    }
    vector
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

fn scale_to(points: &Vec<Point>, size: f64) -> Vec<Point> {
    // non-uniform scale; assumes 2D gestures (i.e., no lines)
    let b = bounding_box(points);
    let mut newpoints = vec![];
    for i in 0..points.len() {
        let qx = points[i].x * (size / b.width);
        let qy = points[i].y * (size / b.height);
        newpoints.push(Point::new(qx, qy));
    }
    return newpoints;
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

fn deg2rad(d: f64) -> f64 {
    d * std::f64::consts::PI / 180.0
}
