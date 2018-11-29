extern crate md5;
extern crate gif;
extern crate gif_dispose;
extern crate image;
extern crate rand;
extern crate bincode;
extern crate imageproc;
extern crate resize;
extern crate polylabel;
use polylabel::polylabel;

extern crate geo;
use geo::{Point, LineString, Polygon};
use imageproc::drawing::Point as ImgPoint;
mod retina;
mod fetch;
mod pdollarplus;
use rand::{thread_rng, Rng};

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
------------------------------
完整的轮廓用于显示汉字(不支持动画着色)
笔画分块数据用来统计笔画轨迹(支持动画轨迹)

*/

/**
 ---------------------
 第二种：
 完整的笔画轮廓，导致体积过大，可以考虑存储汉字黑白图片压缩数据 / 或者使用字体文件渲染文本，笔画按照相对距离计算。
 怎么实现中心线细化？怎么计算中心线？
 https://stackoverflow.com/questions/1203135/what-is-the-fastest-way-to-find-the-visual-center-of-an-irregularly-shaped-pol

 计算多边形中心点!!
    https://github.com/urschrei/polylabel-rs
 */

use resize::Pixel::RGB24;
use resize::Type::Lanczos3;
use bincode::serialize;
use std::fs::File;
use std::io::prelude::*;
use gif::{Frame, ColorOutput, Encoder, Decoder, Repeat, SetParameter};
use gif_dispose::Screen;
use image::{ImageBuffer, RgbImage, Rgb};

const SIZE:usize = 300;

fn main() {
    let file = File::open("fan.gif").unwrap();
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
        let edges = retina::edge_detect(SIZE as u32, SIZE as u32, &buffer_clone, vec![100]);
        let contours = retina::edge_track(edges);
        if contours.len()==0{
            stroke_id += 1;
        }
        //id，边缘，原始buffer
        contours_list.push((stroke_id, contours, buffer_clone));

        last_buffer = buffer.clone();
    }

    //过滤无效的笔画
    let mut cur_id = 0;
    let mut strokes = vec![];
    for (id, contours, buffer) in &contours_list{//每个笔画块
        if cur_id!=*id && contours.len()>0{
            cur_id = *id;
            strokes.push(vec![]);//区分笔画id
        }
        for points in contours{//每个块内的所有边缘, 所有这些边缘都是从buffer中检测出来
            //其实最终每个块只有一个有效的边缘
            if points.len()>10{
                let len = strokes.len();
                strokes[len-1].push((points, buffer));
            }
        }
    }

    //过滤无效的笔画
    strokes.retain(|stroke| !(stroke.len()==1 && stroke[0].0.len()<30));

    //Vec<Vec<(Vec<Point>, Vec<u8>)>>
    
    let mut whole_strokes:Vec<Vec<Vec<retina::Point>>> = vec![];

    //至此所有的笔画块都有了(像素点)

    //生成每个笔画完整的轮廓
    let mut i = 1;
    let mut center_points = vec![];
    for stroke in &strokes{
        let mut centers = vec![];
        let mut stroke_buffer = vec![0; SIZE*SIZE*3];
        for (bi, blocks) in stroke.iter().enumerate(){
            //(Vec<Point>, Vec<u8>)
            // println!("{:?}", blocks);
            let mut rng = thread_rng();
            let color1 = rng.gen_range(0, 255);
            let color2 = rng.gen_range(0, 255);
            let color3 = rng.gen_range(0, 255);

            //计算中心点用
            let mut test_buffer = vec![0; SIZE*SIZE*3];

            for i in (0..blocks.1.len()).step_by(3){
                if blocks.1[i] == 0{
                    stroke_buffer[i] = color1;
                    stroke_buffer[i+1] = color2;
                    stroke_buffer[i+2] = color3;

                    test_buffer[i] = 255;
                    test_buffer[i+1] = 255;
                    test_buffer[i+2] = 255;
                }
            }

            //识别每个block的边缘，并计算中心点
            let edges = retina::edge_detect(SIZE as u32, SIZE as u32, &test_buffer, vec![100]);
            let contours = retina::edge_track(edges);
            println!("第{}笔 block{}的边缘数量:{}", i, bi, contours.len());
            //if i == 1 && bi==0{
                //绘图
                let mut image = RgbImage::new(SIZE as u32, SIZE as u32);
                let mut max_idx = 0;
                let mut max_len = 0;
                for (id, points) in contours.iter().enumerate(){
                    if points.len()>max_len{
                        max_idx = id;
                        max_len = points.len();
                    }
                }
                //计算block中心点
                let poly:Vec<ImgPoint<i32>> = contours[max_idx].iter().map(|point| ImgPoint::new(point.x as i32, point.y as i32)).collect();
                imageproc::drawing::draw_convex_polygon_mut(&mut image, &poly, Rgb([255, 255, 255]));
                image.save(format!("A_stroke{}_block{}.bmp", i, bi)).unwrap();

                let poly:Vec<(f32, f32)> = contours[max_idx].iter().map(|point| (point.x as f32, point.y as f32)).collect();
                let poly = Polygon::new(poly.into(), vec![]);
                let label_pos = polylabel(&poly, &0.10);
                centers.push(label_pos);
           // }
        }
        center_points.push(centers);
        //println!("{:?}", center_points);
        //一幅完整的笔画
        image::save_buffer(&format!("stroke{}.bmp", i), &stroke_buffer, SIZE as u32, SIZE as u32, image::RGB(8)).unwrap();

        //检测边缘之前，首先将图片每行从左往右第一个点，每列从上往下第一个点删除
        //调整边缘的偏移每列从上往下的第一个点的y值都+1, 每行从左往右的第一个点的x值都+1，笔画最终大小为stroke+fill
        for y in 0..SIZE{//每一行
            //找到每行的第一个白色点
            for x in 1..SIZE{
                let i = y*SIZE*3+x*3;
                let ib = y*SIZE*3+(x-1)*3;
                //找到一个白色点，将其删除
                if stroke_buffer[ib]==0 && stroke_buffer[i] >= 254{
                    if stroke_buffer[i] == 255{
                        //删除当前点
                        stroke_buffer[i] = 254;
                        stroke_buffer[i+1] = 254;
                        stroke_buffer[i+2] = 254;
                    }
                }
            }
        }
        for x in 0..SIZE{//每一列
            //找到每行的第一个白色点
            for y in 1..SIZE{
                let i = y*SIZE*3+x*3;
                let ib = (y-1)*SIZE*3+x*3;
                //找到一个白色点，将其删除
                if stroke_buffer[ib]==0 && stroke_buffer[i] >= 254{
                    if stroke_buffer[i] == 255{
                        //删除当前点
                        stroke_buffer[i] = 254;
                        stroke_buffer[i+1] = 254;
                        stroke_buffer[i+2] = 254;
                    }
                }
            }
        }
        //将所有254的点设置黑色
        for i in (0..stroke_buffer.len()).step_by(3){
            if stroke_buffer[i] == 254{
                stroke_buffer[i] = 0;
                stroke_buffer[i+1] = 0;
                stroke_buffer[i+2] = 0;
            }
        }

        //边缘检测的笔画(一个完整的笔画)
        let edges = retina::edge_detect(SIZE as u32, SIZE as u32, &stroke_buffer, vec![100]);
        let contours = retina::edge_track(edges);
        whole_strokes.push(contours.clone());
        let mut edge_buffer = vec![0; SIZE*SIZE*3];
        for points in contours{
            for point in points{
                //println!("point.y={}, point.x={}", point.y, point.x);
                edge_buffer[point.y as usize*SIZE*3+point.x as usize*3] = 255;
                edge_buffer[point.y as usize*SIZE*3+point.x as usize*3 +1] = 255;
                edge_buffer[point.y as usize*SIZE*3+point.x as usize*3 +2] = 255;
            }
        }
        
        image::save_buffer(&format!("stroke_edges{}.bmp", i), &edge_buffer, SIZE as u32, SIZE as u32, image::RGB(8)).unwrap();

        i+=1;
    }

    //压缩数据
    //Vec<Vec<(Vec<Point>, Vec<u8>)>>
    let strokes = whole_strokes;
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
    let mut image = RgbImage::new(SIZE as u32, SIZE as u32);
    //画1笔
    // imageproc::drawing::draw_filled_rect_mut(&mut image, 
    //     imageproc::rect::Rect::at(0,0).of_size(SIZE,SIZE), Rgb([255, 255, 255]));
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
        //let mut color = Rgb([rand::random(), rand::random(), rand::random()]);
        let mut color = Rgb([255, 255, 255]);
        for points in contours{
            for i in 1..points.len(){
                imageproc::drawing::draw_line_segment_mut(&mut image,
                (points[i-1].x as f32, points[i-1].y as f32),
                (points[i].x as f32, points[i].y as f32),
                color);
            }
        }
    }

    //绘制中心点
    for points in center_points{
        let mut color = Rgb([rand::random(), rand::random(), rand::random()]);
        for point in points{
            imageproc::drawing::draw_filled_rect_mut(
                &mut image,
                imageproc::rect::Rect::at(point.x() as i32, point.y() as i32).of_size(4, 4),
                color
            );
        }
    }

    image.save("out.bmp").unwrap();

    // image::save_buffer("out.bmp", &buffer, SIZE, SIZE, image::RGB(8)).unwrap();

    //println!("繁:{}", '繁'.escape_unicode().to_string());
    //let digest = md5::compute(b"#7e41");
    //println!("{:x}", digest);
    //assert_eq!(format!("{:x}", digest), "993ad6ae0193e2cccaf5eca38b6f2ffe");
}