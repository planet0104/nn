extern crate md5;
extern crate gif;
extern crate gif_dispose;
extern crate image;
extern crate rand;
extern crate bincode;
extern crate imageproc;
mod retina;
mod fetch;
mod pdollarplus;

//网站1 https://www.hanzi5.com/bishun/7ed9.html
//网站2 http://www.bihuashunxu.com/

//矢量化 http://potrace.sourceforge.net/
//矢量化 http://autotrace.sourceforge.net/

/*
设置通过难度级别
线条压缩:
1、多个连续点的x相等，或者多个连续点的y相等，删除中间的点
支持多字横屏书写
每个笔画多边形，中心线细化
支持多个字一起写
*/

use bincode::serialize;
use std::fs::File;
use std::io::prelude::*;
use gif::{Frame, ColorOutput, Encoder, Decoder, Repeat, SetParameter};
use gif_dispose::Screen;
use image::{ImageBuffer, RgbImage, Rgb};

fn main() {
    let file = File::open("tao.gif").unwrap();
    let mut decoder = Decoder::new(file);

    // Important:
    decoder.set(ColorOutput::Indexed);

    let mut reader = decoder.read_info().unwrap();

    let mut screen = Screen::new_reader(&reader);
    let mut stroke_id = 0;
    let mut last_buffer = vec![];
    let mut contours_list = vec![];
    while let Some(frame) = reader.read_next_frame().unwrap(){
        screen.blit_frame(&frame).unwrap();
        //screen.pixels // that's the frame now in RGBA format
        let mut buffer = vec![];
        for row in screen.pixels.rows(){
            for pixel in row{
                buffer.push(pixel.r);
                buffer.push(pixel.g);
                buffer.push(pixel.b);
                buffer.push(pixel.a);
            }
        }
        //对比不同
        let mut buffer_clone = vec![];
        if last_buffer.len()>0{
           for i in (0..buffer.len()).step_by(4){
               if buffer[i] != last_buffer[i]
                    || buffer[i+1] != last_buffer[i+1]
                    || buffer[i+2] != last_buffer[i+2]
                    || buffer[i+3] != last_buffer[i+3]{
                    
                    //获取每隔颜色的差值
                    let dr = buffer[i] as f64 - last_buffer[i] as f64;
                    let dg = buffer[i+1] as f64 - last_buffer[i+1] as f64;
                    let db = buffer[i+2] as f64 - last_buffer[i+2] as f64;
                    let da = buffer[i+3] as f64 - last_buffer[i+3] as f64;
                    //计算颜色之间的3D空间距离
                    let e = dr * dr + dg * dg + db * db + da * da;
                    //260100
                    if e>260100.0*0.05{
                        buffer_clone.push(0);
                        buffer_clone.push(0);
                        buffer_clone.push(0);
                    }else{
                        buffer_clone.push(255);
                        buffer_clone.push(255);
                        buffer_clone.push(255);
                    }
               }else{
                    buffer_clone.push(255);
                    buffer_clone.push(255);
                    buffer_clone.push(255);
                }
           }
        }
        //buffer_clone为差异，检测其边缘
        let edges = retina::edge_detect(300, 300, &buffer_clone, vec![100]);
        let contours = retina::edge_track(edges);
        if contours.len()==0{
            stroke_id += 1;
        }
        contours_list.push((stroke_id, contours));

        last_buffer = buffer.clone();
    }

    //过滤无效的笔画
    let mut cur_id = 0;
    let mut strokes = vec![];
    for (id, contours) in &contours_list{
        if cur_id!=*id && contours.len()>0{
            cur_id = *id;
            strokes.push(vec![]);
        }
        for points in contours{
            if points.len()>10{
                let len = strokes.len();
                strokes[len-1].push(points);
            }
        }
    }

    //过滤无效的笔画
    strokes.retain(|stroke| !(stroke.len()==1 && stroke[0].len()<30));

    //压缩数据
    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor<block_len && block[cursor].x == block[cursor-1].x{//检查竖线
                        cursor += 1;
                        count += 1;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        if count == 1{
                            new_points.push(block[cursor-1].clone());    
                        }else{
                            new_points.push(block[cursor].clone());
                            cursor += 1;
                        }
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };

    //横线过滤
    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor<block_len && block[cursor].y == block[cursor-1].y{//检查横线
                        cursor += 1;
                        count += 1;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        if count == 1{
                            new_points.push(block[cursor-1].clone());    
                        }else{
                            new_points.push(block[cursor].clone());
                            cursor += 1;
                        }
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };

    //斜线过滤
    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor<block_len && block[cursor].y != block[cursor-1].y && block[cursor].x != block[cursor-1].x{//检查横线
                        cursor += 1;
                        count += 1;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        if count == 1{
                            new_points.push(block[cursor-1].clone());    
                        }else{
                            new_points.push(block[cursor].clone());
                            cursor += 1;
                        }
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };

    //--------- 14.9K ------

    //两个点的斜线过滤
    /*
          --
            --
              --
    */
    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor+2<block_len
                        && block[cursor+1].x == block[cursor].x+1
                        && block[cursor+1].y == block[cursor].y+1
                        && block[cursor+2].y == block[cursor].y+1
                    {//检查横线
                        cursor += 2;
                        count += 2;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        new_points.push(block[cursor].clone());
                        cursor += 1;
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };

    //--------- 14.5K --------------

    /*
            --
          --
        --
    */
    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor+2<block_len
                        && block[cursor+1].x == block[cursor].x-1
                        && block[cursor+1].y == block[cursor].y+1
                        && block[cursor+2].y == block[cursor].y+1
                    {//检查横线
                        cursor += 2;
                        count += 2;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        new_points.push(block[cursor].clone());
                        cursor += 1;
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };


    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor+2<block_len
                        && block[cursor+1].x == block[cursor].x-1
                        && block[cursor+1].y == block[cursor].y-1
                        && block[cursor+2].y == block[cursor].y-1
                    {//检查横线
                        cursor += 2;
                        count += 2;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        new_points.push(block[cursor].clone());
                        cursor += 1;
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };

    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor+2<block_len
                        && block[cursor+1].x == block[cursor].x+1
                        && block[cursor+1].y == block[cursor].y-1
                        && block[cursor+2].y == block[cursor].y-1
                    {//检查横线
                        cursor += 2;
                        count += 2;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        new_points.push(block[cursor].clone());
                        cursor += 1;
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };

    //--------- 13.8K --------------

    //三个点的斜线过滤
    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor+3<block_len
                        && block[cursor+1].x == block[cursor].x+1
                        && block[cursor+1].y == block[cursor].y+1
                        && block[cursor+2].y == block[cursor].y+1
                        && block[cursor+3].y == block[cursor].y+1
                    {//检查横线
                        cursor += 3;
                        count += 3;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        new_points.push(block[cursor].clone());
                        cursor += 1;
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };
    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor+3<block_len
                        && block[cursor+1].x == block[cursor].x-1
                        && block[cursor+1].y == block[cursor].y+1
                        && block[cursor+2].y == block[cursor].y+1
                        && block[cursor+3].y == block[cursor].y+1
                    {//检查横线
                        cursor += 3;
                        count += 3;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        new_points.push(block[cursor].clone());
                        cursor += 1;
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };


    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor+3<block_len
                        && block[cursor+1].x == block[cursor].x-1
                        && block[cursor+1].y == block[cursor].y-1
                        && block[cursor+2].y == block[cursor].y-1
                        && block[cursor+3].y == block[cursor].y-1
                    {//检查横线
                        cursor += 3;
                        count += 3;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        new_points.push(block[cursor].clone());
                        cursor += 1;
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };

    let strokes = {
        let mut new_strokes = vec![];
        for stroke in strokes{//每一笔
            let mut new_stroke = vec![];
            for block in stroke{//每一块
                let mut new_points = vec![];
                //每个点
                new_points.push(block[0].clone());
                let mut cursor = 1;
                let block_len = block.len();
                while cursor<block_len{
                    let mut count = 0;
                    while cursor+3<block_len
                        && block[cursor+1].x == block[cursor].x+1
                        && block[cursor+1].y == block[cursor].y-1
                        && block[cursor+2].y == block[cursor].y-1
                        && block[cursor+3].y == block[cursor].y-1
                    {//检查横线
                        cursor += 3;
                        count += 3;
                    }
                    if count>1{
                        new_points.push(block[cursor-count].clone());
                        new_points.push(block[cursor-1].clone());
                    }else{
                        new_points.push(block[cursor].clone());
                        cursor += 1;
                    }
                }
                new_stroke.push(new_points);
            }
            new_strokes.push(new_stroke);
        }

        new_strokes
    };

    //--------- 11.6K --------------

    println!("笔画数量:{}", strokes.len());

    //序列化 Vec<Vec<Vec<Point>>> to Vec<Vec<Vec<(i16,i16)>>>
    let data:Vec<Vec<Vec<(i16,i16)>>> = strokes.iter().map(|stroke|{
        stroke.iter().map(|rect|{
            rect.iter().map(|point|{
                (point.x as i16, point.y as i16)
            }).collect()
        }).collect()
    }).collect();
    let encoded: Vec<u8> = serialize(&data).unwrap();
    let mut file = File::create("stroke.data").unwrap();
    file.write_all(&encoded).unwrap();

    //绘图
    let mut image = RgbImage::new(300, 300);
    //画1笔
    // imageproc::drawing::draw_filled_rect_mut(&mut image, 
    //     imageproc::rect::Rect::at(0,0).of_size(300,300), Rgb([255, 255, 255]));
    // let points = strokes[0][0];
    // println!("{:?}", points);

    // let data:Vec<(i16,i16)> = points.iter().map(|point|{
    //     (point.x as i16, point.y as i16)
    // }).collect();
    // let encoded: Vec<u8> = serialize(&data).unwrap();
    // let mut file = File::create("points.data").unwrap();
    // file.write_all(&encoded).unwrap();

    // for i in 1..points.len(){
    //     imageproc::drawing::draw_line_segment_mut(&mut image,
    //     (points[i-1].x as f32, points[i-1].y as f32),
    //     (points[i].x as f32, points[i].y as f32),
    //     Rgb([0, 0, 0]));
    // }

    for contours in strokes{
        let mut color = Rgb([rand::random(), rand::random(), rand::random()]);
        for points in contours{
            for i in 1..points.len(){
                imageproc::drawing::draw_line_segment_mut(&mut image,
                (points[i-1].x as f32, points[i-1].y as f32),
                (points[i].x as f32, points[i].y as f32),
                color);
            }
        }
    }
    image.save("out.bmp").unwrap();

    // image::save_buffer("out.bmp", &buffer, 300, 300, image::RGB(8)).unwrap();

    //println!("繁:{}", '繁'.escape_unicode().to_string());
    //let digest = md5::compute(b"#7e41");
    //println!("{:x}", digest);
    //assert_eq!(format!("{:x}", digest), "993ad6ae0193e2cccaf5eca38b6f2ffe");
}