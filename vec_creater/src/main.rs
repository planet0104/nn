extern crate lodepng;

fn main(){
    //println!("{},{}", STROKE_POINTS_PIE.len(), STROKE_POINTS_NA.len() );
    // let mut s = String::new();
    // for i in 167..175{
    //     s.push_str(&format!("({},112),", i));
    // }

    let mut s = String::new();
    s.push_str(&lrbt(&lodepng::decode24_file("hp1_0.png").unwrap()));
    s.push_str(&rltb(&lodepng::decode24_file("hp1_1.png").unwrap()));
    println!("横撇1{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&lrbt(&lodepng::decode24_file("hp2_0.png").unwrap()));
    s.push_str(&rltb(&lodepng::decode24_file("hp2_1.png").unwrap()));
    println!("横撇2{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&lrbt(&lodepng::decode24_file("hp3_0.png").unwrap()));
    s.push_str(&rltb(&lodepng::decode24_file("hp3_1.png").unwrap()));
    println!("横撇3{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&tbrl(&lodepng::decode24_file("xp.png").unwrap()));
    println!("斜撇{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&tbrl(&lodepng::decode24_file("sp.png").unwrap()));
    println!("竖撇{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&tblr(&lodepng::decode24_file("xiena.png").unwrap()));//上下左右
    println!("斜捺{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&lrtb(&lodepng::decode24_file("pingna.png").unwrap()));//左右上下
    println!("平捺{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&tbrl(&lodepng::decode24_file("szp0.png").unwrap()));
    s.push_str(&lrtb(&lodepng::decode24_file("szp1.png").unwrap()));
    s.push_str(&rltb(&lodepng::decode24_file("szp2.png").unwrap()));
    println!("竖折撇{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&lrbt(&lodepng::decode24_file("hzzp0.png").unwrap()));
    s.push_str(&rltb(&lodepng::decode24_file("hzzp1.png").unwrap()));
    s.push_str(&lrtb(&lodepng::decode24_file("hzzp2.png").unwrap()));
    s.push_str(&rltb(&lodepng::decode24_file("hzzp3.png").unwrap()));
    println!("横折折撇{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&tblr(&lodepng::decode24_file("shuti1_0.png").unwrap()));
    s.push_str(&lrbt(&lodepng::decode24_file("shuti1_1.png").unwrap()));
    println!("竖提1{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&tbrl(&lodepng::decode24_file("shuti2_0.png").unwrap()));
    s.push_str(&lrbt(&lodepng::decode24_file("shuti2_1.png").unwrap()));
    println!("竖提2{:#?}", s);
    println!("-------------------------------------------------------------");
    let mut s = String::new();
    s.push_str(&tbrl(&lodepng::decode24_file("shuti3_0.png").unwrap()));
    s.push_str(&lrbt(&lodepng::decode24_file("shuti3_1.png").unwrap()));
    println!("竖提3{:#?}", s);
}

//左-右-下-上
fn lrbt(image:&lodepng::Bitmap<lodepng::RGB<u8>>) -> String{
    let mut s = String::new();
    for x in 0..image.width{
        for y in (0..image.height).rev(){
            let id = y*image.width+x;
            if image.buffer[id].r != 255{
                s.push_str(&format!("({},{}),", x, y));
            }
        }
    }
    s
}
//左右上下
fn lrtb(image:&lodepng::Bitmap<lodepng::RGB<u8>>) -> String{
    let mut s = String::new();
    for x in 0..image.width{
        for y in 0..image.height{
            let id = y*image.width+x;
            if image.buffer[id].r != 255{
                s.push_str(&format!("({},{}),", x, y));
            }
        }
    }
    s
}

//右左上下
fn rltb(image:&lodepng::Bitmap<lodepng::RGB<u8>>) -> String{
    let mut s = String::new();
    for x in (0..image.width).rev(){
        for y in 0..image.height{
            let id = y*image.width+x;
            if image.buffer[id].r != 255{
                s.push_str(&format!("({},{}),", x, y));
            }
        }
    }
    s
}

//上-下-右-左
fn tbrl(image:&lodepng::Bitmap<lodepng::RGB<u8>>) -> String{
    let mut s = String::new();
    for y in 0..image.height{
        for x in (0..image.width).rev(){
            let id = y*image.width+x;
            if image.buffer[id].r != 255{
                s.push_str(&format!("({},{}),", x, y));
            }
        }
    }
    s
}

//上-下-左-右
fn tblr(image:&lodepng::Bitmap<lodepng::RGB<u8>>) -> String{
    let mut s = String::new();
    for y in 0..image.height{
        for x in 0..image.width{
            let id = y*image.width+x;
            if image.buffer[id].r == 0{
                s.push_str(&format!("({},{}),", x, y));
            }
        }
    }
    s
}