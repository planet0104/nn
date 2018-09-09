extern crate reqwest;
extern crate bincode;
extern crate piston_window;

mod dollar;
mod ndollar;
mod pdollar;
mod pdollarplus;
mod fetch_stroke;

use piston_window::*;
use std::fs::File;
use std::io::prelude::*;
use bincode::{serialize, deserialize};
use dollar::DollarRecognizer;
use std::collections::HashMap;
use std::fs;
const STROKE_ORDER_DATA:&[u8] = include_bytes!("../stroke_order_data");
const ORIGINAL_STROKE_DATA:&[u8] = include_bytes!("../original_stroke_data");


pub fn fetch_all_ch(){
    let contents = {
        let mut contents = String::new();
        let mut file = File::open("yq.txt").unwrap();
        file.read_to_string(&mut contents).unwrap();
        contents
    };

    //存放所有的笔画map
    let mut all_map:HashMap<char, Vec<Vec<[i32;2]>>>= HashMap::new();

    for line in contents.lines(){
        for ch in line.chars(){
            let file = File::open(format!("strokes_new/{}.stroke", ch));
            let file_err = File::open(format!("strokes_new/{}.stroke_err", ch));
            if file.is_err() && file_err.is_err(){
                //println!("没有{}的文件", ch);
                let result = fetch_original_stroke(&ch);
                if let Some(strokes) = result{
                    if strokes.1.len() == 0 && strokes.0.len() ==0{
                        //超时的文件
                        let mut file = File::create(format!("strokes_new/{}.stroke_time_out", ch)).unwrap();
                        file.write_all(b"").unwrap();
                    }else{
                        //println!("{:?}", strokes);
                        let strokes = strokes.1;
                        let encoded: Vec<u8> = serialize(&strokes).unwrap();
                        let mut file = File::create(format!("strokes_new/{}.stroke", ch)).unwrap();
                        file.write_all(&encoded).unwrap();
                    }
                }else{
                    //println!("找不到{}的笔画，删除", ch);
                    let mut file = File::create(format!("strokes_new/{}.stroke_err", ch)).unwrap();
                    file.write_all(b"").unwrap();
                }
            }else{
                //读取笔画放入map
                if !file.is_err(){
                    let mut contents = vec![];
                    file.unwrap().read_to_end(&mut contents).unwrap();
                    let mut data:Vec<Vec<[i32;2]>> = deserialize(&contents[..]).unwrap();
                    all_map.insert(ch, data);   
                }
            }
        }    
    }

    //保存原始笔画文件
    println!("原始笔画 文字数量:{}", all_map.len());
    let encoded: Vec<u8> = serialize(&all_map).unwrap();
    let mut file = File::create("original_stroke_data").unwrap();
    file.write_all(&encoded).unwrap();
}

pub fn fetch_all_order(){

    let mut stroke_orders_map:HashMap<char, Vec<usize>>= HashMap::new();
    
    for entry in fs::read_dir("strokes_new").unwrap() {
        let dir = entry.unwrap();
        let file_name = dir.file_name().into_string().unwrap();
        let mut file_name = file_name.split(".");
        let ch = file_name.next().unwrap().chars().next().unwrap();
        let ext = file_name.next().unwrap();
        let file = File::open(format!("strokes_new/{}.stroke_order", ch));
        if ext=="stroke" && file.is_err(){
            //获取unicode码
            let unicode = ch.escape_unicode().to_string().replace("\\", "").replace("u{", "").replace("}", "");
            let dict_url = format!("http://dict.r12345.com/0x{}.html", unicode);
            let dict_html = fetch(&dict_url);
            if dict_html.len()>0{
                let mut stroke_orders:Vec<usize> = vec![];
                if let Some(j2) = dict_html.split("笔顺编号:</span>").skip(1).next(){
                    if let Some(s) = j2.split("<br>").next(){
                        for c in s.chars(){
                            if let Ok(u) = format!("{}", c).parse::<usize>(){
                                stroke_orders.push(u);
                            }
                        }
                    }
                }
                println!("stroke_orders={:?}", stroke_orders);
                if stroke_orders.len()>0{
                    let encoded: Vec<u8> = serialize(&stroke_orders).unwrap();
                    let mut file = File::create(format!("strokes_new/{}.stroke_order", ch)).unwrap();
                    file.write_all(&encoded).unwrap();
                }
            }
        }else{
            if !file.is_err(){
                let mut contents = vec![];
                file.unwrap().read_to_end(&mut contents).unwrap();
                let mut data:Vec<usize> = deserialize(&contents[..]).unwrap();
                stroke_orders_map.insert(ch, data);   
            }
        }
    }

    println!("文字数量:{}", stroke_orders_map.len());
    //写入数据
    let encoded: Vec<u8> = serialize(&stroke_orders_map).unwrap();
    let mut file = File::create("stroke_order_data").unwrap();
    file.write_all(&encoded).unwrap();
}

//http://depts.washington.edu/madlab/proj/dollar/
fn main() {
    //unistroke_demo();
    //multistroke_demo();
    //point_cloud_demo();
    //point_cloud_plus_demo();

    /*
    APP名称：拾字
    1、主页由候选区、写字区组成。
    2、候选区，可以切换文章。（文章可以写入、选字、内置分类文章、服务器同步文章）
    3、写字区，可以回退、发音、展示笔画动画。
    4、可查看练字记录，总共写对了多少字。

    收集一年级，到六年级的语文书上的汉字，古诗，文章
    练字app游戏模式，往下掉字！！！填充成语练字，填充成语游戏模式

    程序界面用HTML+JS实现
    rust通过执行js来操作。

    笔画点数组，需要进行填充，笔画点数少于60，要在两点之间插入新的点。

    每个笔画单独进行识别。
    每写一笔之前，创建一个空的单笔识别器，并加入该笔画，然后判定用户写完的笔画和此笔画匹配度，匹配度为得分。
    所有笔画写完以后，创建一个空的多笔画识别器，将文字笔画加入，然后对比用户的笔画，如果不匹配，说明写的不对。

    写字板中文章预览，写对一次标黑色，写错一字整个字标红色，写错一划，单独笔画标红。
    */

    //创建笔画数据库
    //fetch_all_ch();
    //return;

    //创建笔画顺序数据库
    // fetch_all_order();
    // return;

    //根据原始笔画文件，创建新的笔画数据
    let original_map:HashMap<char, Vec<Vec<[i32;2]>>> = deserialize(&ORIGINAL_STROKE_DATA[..]).unwrap();
    /*let mut strokes = original_map.get(&'了').unwrap().clone();
    strokes.remove(1);
    draw_stroke(&strokes);
    */

    let stroke_orders_map:HashMap<char, Vec<usize>> = deserialize(&STROKE_ORDER_DATA[..]).unwrap();
    let mut map:HashMap<char, Vec<Vec<[i32;2]>>> = HashMap::new();
    for entry in fs::read_dir("strokes_new").unwrap() {
        let dir = entry.unwrap();
        let file_name = dir.file_name().into_string().unwrap();
        let mut file_name = file_name.split(".");
        let ch = file_name.next().unwrap().chars().next().unwrap();
        let ext = file_name.next().unwrap();
        if ext != "stroke"{
            continue;
        }
        let strokes = original_map.get(&ch).unwrap();
        let stroke_orders = stroke_orders_map.get(&ch).unwrap();
        //println!("识别{}", ch);

        //个别字笔画进行调整
        
        //綮的第三画没用，最后一画替换掉第三画
        let mut clone_strokes = vec![];
        clone_strokes.append(&mut strokes.clone());
        if ch == '綮'{
            let last = clone_strokes.pop().unwrap();
            clone_strokes[2] = last;
        }

        let strokes = &clone_strokes;

        // if ch == '霸'{
        //     println!("旧笔画={:?}\n", strokes);
        // }

        let mut new_data = vec![];

        let mut si = 0;
        for points in strokes{
            // if ch == '霸'{
            //     println!("第{}笔 {:?} 长度{}", si, points, points.len());
            // }
            //如果是横(提)，只要起点和终点
            let points = 
            if stroke_orders.len()>si && stroke_orders[si] == 1{
                // if ch == '霸'{
                //     println!("横线1:{:?}", points);
                // }
                //有些竖笔画写成了横，导致只有两个点，这样取就错了，大于2个点的才需要减少点
                //y最大的点为起点
                let mut lowest = 0;
                for pi in 0..points.len(){
                    if points[pi][1]>points[lowest][1]{
                        lowest = pi;
                    }
                }
                let mut newpoints = vec![points[lowest], points[points.len()-1]];
                // if ch == '霸'{
                //     println!("横线2:{:?}", newpoints);
                // }
                //如果第一个点和第二个点相等，回退
                if newpoints[0][0] == newpoints[1][0] && newpoints[0][1] == newpoints[1][1]{
                    newpoints = points.clone();
                }
                // if ch == '霸'{
                //     println!("横线3:{:?}", newpoints);
                // }
                //如果起点x大于终点x，反过来
                if newpoints[0][0]>newpoints[1][0]{
                    vec![newpoints[1], newpoints[0]]
                }else{
                    newpoints
                }
            }else{
                points.clone()
            };

            //折线中间的突起去掉 捺不删凸起
            let mut new_points = if stroke_orders[si] != 4{
                let mut new_points:Vec<[i32;2]> = vec![];
                for [x, y] in points{
                    let len = new_points.len();
                    if len>=2 && new_points[len-2][0] == x &&
                                        new_points[len-2][1] == y{
                            new_points.pop();
                    }else{
                        new_points.push([x, y]);
                    }
                }
                new_points
            }else{
                points.clone()
            };
            
            //折末尾的勾去掉(如：每)
            if stroke_orders.len()>si &&  stroke_orders[si] == 5{
                let len = new_points.len();
                if len>=5{
                    //最后一个点的y和倒数第3个点的y相等
                    if new_points[len-1][1] == new_points[len-3][1]{
                        new_points.pop();
                        new_points.pop();
                    }
                }
            }
            //如果当前笔画是撇，【撇开头的勾去掉】
            if stroke_orders.len()>si &&  stroke_orders[si] == 3{
                //如果第一个点的x小于第二个点的x，删掉第一个点
                if new_points.len()>=3 &&
                    new_points[0][0] < new_points[1][0]{
                    new_points.remove(0);
                }
            }
            // if ch == '霸'{
            //     println!("第{}笔 新{:?} 笔画:{} 长度{}", si, new_points, stroke_orders[si], new_points.len());
            // }
            
            //如果是竖，并且起点y大于终点y，需要反向
            if stroke_orders[si] == 2{
                if new_points[0][1]>new_points[new_points.len()-1][1]{
                    new_points = new_points.iter().rev().map(|p|{
                        [p[0], p[1]]
                    }).collect();
                }

                //如果竖>=3个点，并且第1个点的x<第2个点的x，删除第一个点
                //如果竖>=3个点，并且第1个点的x>第2个点的x，删除第一个点
                if new_points.len()>2{
                    if new_points[0][0]!=new_points[1][0]{
                        new_points.remove(0);
                    }
                }
            }

            //如果是撇，并且起点y大于终点y，需要反向
            if stroke_orders[si] == 3{
                if new_points[0][1]>new_points[new_points.len()-1][1]{
                    new_points = new_points.iter().rev().map(|p|{
                        [p[0], p[1]]
                    }).collect();
                }
            }


            //心 虽然是捺，但是方向是对的
            //火 虽然是捺，但是方向是错的
            //怵 虽然是捺，但是方向是对的
            //

            //结论 只有 火 是错的！
            //如果是捺，并且起点x小于终点x，需要反转（如 '火'第1笔，'灭'第2笔）
            // if stroke_orders[si] == 4{
            //     let start = new_points[0];
            //     let end = new_points[new_points.len()-1];
            //     // let dx = (start[0]-end[0]).abs();
            //     // let dy = (start[1]-end[1]).abs();
            //     if start[0] > end[0]{
            //         //middle x
            //         let mx = end[0]+(start[0]-end[0])/2;
            //         for point in &mut new_points{
            //             let d = point[0]-mx;
            //             point[0] += -d*2;
            //         }
            //     }
            // }

            //如果是折，并且起点y大于终点y，需要反向
            if stroke_orders[si] == 5{
                if new_points[0][1]>new_points[new_points.len()-1][1]{
                    new_points = new_points.iter().rev().map(|p|{
                        [p[0], p[1]]
                    }).collect();
                }
            }

            new_data.push(new_points);
            si += 1;
        }

        //火字旁第一笔需要反转（有问题！！！！！！！！！！！！！！！！！！！！！ 有些字火字旁不是第一个！！）
        if "火灭灯灰灮灳灱灲灿灸灵灺炀灾灶灼災灻灴灹灷炉炝炆炘炎炙炬炜炕炔炅炖炊炒炞炐炂烎炈炇炋炍炄炑炗炚炛炌炏炓烁炱炭烃炫炸炻炼烂炯烀炟炽炳炮炷炧炤炢炿炡炴炨炠炵炲炶炦炪炥炾炣炩烨烛烉烖烔烠烢烥烊烟烜烦烘烩烬烤烙烧烫烡烆烚烍烌烅烕烑烐烵烓烶烒烣烄烗烮焒烞烇烻焐烯烴烱焅烲烷焖烺焌焗焕焊焓烽烾焍焋焔烼焇焈焁焂焫烿烰烸焃焀焆烳焻焧焨焤焵焿焸煱焥煑焙焯焠煚焜焮焰焱焢焝焳焽焹煀焟焬焲煐焴焺焼煡焞焛焾焷焩焪焭煜煴煒煉煙煠煩煗煬煊煖煨煲煏煸煅煳煌煤煣煺煢煇煄熍煪煰煶煫煓煟煆煋煔煵煘煁煈煂煥煍煯煃煷煝熢熚煿煼煾熕熒熗燁熄熥煽熔熘熇煹熆熉熅熎熖熁熂熓熃煻熀煛熑熐熋熌熩熣熜熝熨熠熵熰熳熯熛熿熞熧熫熼熪熤熭熡熮熴熦熲燄營熺燒燀燙熾燏燠燖燧燊燃燋燎熸燔燉燚燜熷燅熻燍燆燂燘燐燤燗燈燪熶燵燑燝燇燣燛燷燴燭燦燮燥燬燫燯燶燳燱燡燢燲熽燨燰燩燼燿爗燻燹燽燺爀燸爃爄爁爌爊爆爕爍爂爑爉爎爈爅爓爔爐爘爏爒爋爝爚爛爟爖爙爡爞爜爠爣爤爦爥爧爨爩".contains(ch){
            
            //如果当前比是捺，并且当前开始是4334，说明是火
            for pi in 0..new_data.len(){
                if pi<=new_data.len()-3{
                    if stroke_orders[pi] == 4
                    && stroke_orders[pi+1] == 3
                    && stroke_orders[pi+2] == 3
                    && stroke_orders[pi+3] == 4{
                        let start = new_data[pi][0];
                        let end = new_data[pi][new_data[pi].len()-1];
                        let dx = (start[0]-end[0]).abs();
                        let dy = (start[1]-end[1]).abs();
                        if start[0] > end[0] && dx<dy{
                            //middle x
                            let mx = end[0]+(start[0]-end[0])/2;
                            for point in &mut new_data[pi]{
                                let d = point[0]-mx;
                                point[0] += -d*2;
                            }
                        }
                    }
                }
            }
        }

        // if ch == '霸'{
        //     println!("新笔画:{:?}", new_data);
        // }

        use pdollarplus::{PDollarPlusRecognizer, Point};
        let mut pdpr = PDollarPlusRecognizer::new();
        //添加了第一画的模板
        let le_strokes = original_map.get(&'了').unwrap();
        let le_one = le_strokes[0].iter().map(|p|{ Point::new(p[0], p[1], 1) }).collect();
        pdpr.add_gesture("leone", le_one);
        //'了'的第一画是反的，循环所有笔画，如果匹配了的第一画，如果起点x大于终点x，要反过来
        //println!("识别{}:", ch);
        let mut bi = 0;
        let mut new_data:Vec<Vec<[i32;2]>> = new_data.iter().map(|ps|{
            let points = ps.iter().map(|psraw|{
                Point::new(psraw[0], psraw[1], 1)
            }).collect();
            bi += 1;
            // if ch == '霸'{
            //     println!("{:?}", points);
            // }
            let result = pdpr.recognize(points);
            if result.name == "leone"{
                let mut rev = false;
                if result.score<2.0{
                    //检查笔画是否需要反转, 如('了'第1笔，'子'第一笔，'郭'第6笔)
                    
                    //如果起点的y低于终点y，那么需要反转
                    if ps[0][1]>ps[ps.len()-1][1]{
                        rev = true;
                        //println!("'了'第1笔类似的反转:[{}]", ch);
                    }
                }
                if rev{
                    ps.iter().rev().map(|p|{
                        [p[0], p[1]]
                    }).collect()
                }else{
                    ps.clone()
                }
            }else{
                ps.clone()
            }
        }).collect();

        //反转 了 的第二笔
        // let mut pdpr = PDollarPlusRecognizer::new();
        // let le_strokes = original_map.get(&'了').unwrap();
        // let le_two = le_strokes[1].iter().map(|p|{ Point::new(p[0], p[1], 1) }).collect();
        // pdpr.add_gesture("letwo", le_two);
        // let mut bi = 0;
        // let new_data:Vec<Vec<[i32;2]>> = new_data.iter().map(|ps|{
        //     let points = ps.iter().map(|psraw|{
        //         Point::new(psraw[0], psraw[1], 1)
        //     }).collect();
        //     bi += 1;
        //     let result = pdpr.recognize(points);
        //     //大于2点的才是竖弯钩
        //     if result.name == "letwo" && ps.len()>2 && stroke_orders[bi-1] == 2{
        //         let mut rev = false;
        //         if result.score<2.0{
        //             //检查笔画是否需要反转, 如('了'第二笔)
                    
        //             //如果起点的y低于终点y，那么需要反转
        //             if ps[0][1]>ps[ps.len()-1][1]{
        //                 rev = true;
        //                 println!("'了'第2笔类似的反转:[{}]", ch);
        //             }
        //         }
        //         if rev{
        //             ps.iter().rev().map(|p|{
        //                 [p[0], p[1]]
        //             }).collect()
        //         }else{
        //             ps.clone()
        //         }
        //     }else{
        //         ps.clone()
        //     }
        // }).collect();

        map.insert(ch, new_data);
    }

    //

    println!("文字数量:{}", map.len());
    //写入数据
    let encoded: Vec<u8> = serialize(&map).unwrap();
    let mut file = File::create("stroke_data").unwrap();
    file.write_all(&encoded).unwrap();
}

// let strokes_data = "144,128-144,128-161,127-164,148-182,160-178,126-194,125-198,157-215,154-211,124-227,122-231,151-247,148-244,121-260,120-264,145-280,142-277,119-293,117-296,140-313,137-310,116-327,115-329,134-345,131-343,114-360,113-362,128-378,126-376,110-392,108-394,124-411,121-409,106-425,103-427,119-444,117-442,101-458,99-460,114-477,112-474,96-491,94-493,110-509,107-507,92-524,89-526,105-542,103-539,75-553,57-559,100-575,98-572,76-591,95-591,95#588,92-592,96-578,106-567,95-545,98-565,116-553,128-528,105-519,119-542,140-530,152-509,133-500,147-519,164-507,176-491,161-481,175-496,188-484,200-472,189-462,203-473,212-461,224-451,214-439,226-450,236-438,248-427,238-415,250-427,260-415,272-403,261-392,273-404,284-392,296-384,280#336,224-336,224-349,240-342,240-348,256-361,255-371,271-353,272-359,288-377,287-383,302-365,303-370,319-389,318-393,334-376,335-378,351-397,350-400,366-380,367-383,383-403,382-406,397-385,399-387,414-409,413-411,429-389,430-391,446-412,445-414,461-392,462-392,478-416,477-416,493-392,494-392,510-416,509-416,525-392,526-392,543-416,541-416,557-390,559-388,575-416,573-414,590-386,591-385,607-411,606-407,622-379,623-374,640-404,638-401,654-369,656-363,672-393,671-385,687-357,689-350,705-377,704-369,720-349,721-360,736-360,736#364,681-347,718-337,704-361,681-344,674-327,690-317,677-331,663-318,652-305,665-293,653-305,641-291,630-281,641-269,628-278,619-265,608-259,614-248,600-256,600".trim();
// let mut strokes:Vec<Vec<[i32; 2]>> = vec![];
// for stroke in strokes_data.split("#"){
//     let mut points = vec![];
//     for point in stroke.split("-"){
//         let mut iter = point.split(",");
//         let val1 = iter.next().unwrap();
//         let val2 = iter.next().unwrap();
//         points.push([val1.parse::<i32>().unwrap(), val2.parse::<i32>().unwrap()]);
//     }
//     strokes.push(points);
// }
// draw_stroke(&strokes);

fn draw_stroke(strokes:&Vec<Vec<[i32;2]>>){
    let mut window: PistonWindow = WindowSettings::new("dollar", [900, 900])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            for points in strokes{
                for i in 1..points.len(){
                    line(
                        [0.0, 0.0, 0.0, 255.0],
                        1.0,
                        [points[i - 1][0].into(), points[i - 1][1].into(), points[i][0].into(), points[i][1].into()],
                        context.transform,
                        graphics,
                    );
                    ellipse(
                        [0.0, 255.0, 0.0, 255.0],
                        [points[i][0].into(), points[i][1].into(), 5.0, 5.0],
                        context.transform,
                        graphics,
                    );
                }
                ellipse(
                    [0.0, 0.0, 255.0, 255.0],
                    [points[0][0].into(), points[0][1].into(), 25.0, 25.0],
                    context.transform,
                    graphics,
                );
            }
        });
    }
}

//笔画点数少于60，要在两点之间插入新的点
// fn resample(strokes: Vec<Vec<[i32;2]>>) ->Vec<Vec<[i32;2]>>{
//     use dollar::{resample, Point};
//     let mut new_strokes = vec![];
//     for points in strokes{
//         let mut new_points = vec![];
//         for point in points{
//             new_points.push(Point::new(point[0], point[1]));
//         }
//         let re = resample(new_points, 64);
//         new_strokes.push(re.iter().map(|rp|{
//             [rp.x  as i32, rp.y as i32]
//         }).collect());
//     }

//     new_strokes
// }

fn get_strokes_from_file(ch:char) -> Vec<Vec<[i32;2]>>{
    let mut file = File::open(format!("strokes/{}.stroke", ch)).unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();
    let mut decoded:Vec<Vec<[i32;2]>> = deserialize(&contents[..]).unwrap();
    decoded
}

fn fetch(url:&str) -> String{
    reqwest::get(url).and_then(|mut res|{
        let s:String = res.text().unwrap_or("".to_string());
        Ok(s)
    }).unwrap_or("".to_string())
}

//从 http://bishun.shufaji.com 解析一个汉字的笔画
fn fetch_original_stroke<'a>(ch: &char)->Option<(String, Vec<Vec<[i32;2]>>)>{
    //获取unicode码
    let unicode = ch.escape_unicode().to_string().replace("\\", "").replace("u{", "").replace("}", "");
    let strock_url = format!("http://bihua.shufami.com/0x{}.html", unicode);
    let html = fetch(&strock_url);

    if html.len() == 0{
        return Some(("".to_string(), vec![]));
    }

    /*

	hzbh.main('繁', 繁:[17,'0:(162,18) (186,36) (138,96) (96,144) (30,204)#1:(138,96) (420,96) (378,78) (336,96)#2:(144,138) (108,336) (84,354) (108,336) (444,336) (402,324) (366,336)#3:(138,162) (360,162) (390,138) (360,162) (330,360)#4:(192,168) (246,204) (264,228) (270,246)#5:(24,252) (462,252) (420,234) (384,252)#6:(192,252) (246,276) (264,300) (270,324)#7:(528,18) (552,30) (510,96) (474,144) (444,186)#8:(498,114) (726,114) (684,96) (648,114)#9:(654,114) (636,162) (612,216) (582,264) (546,306) (492,354) (438,390)#10:(486,132) (522,210) (552,258) (588,300) (630,336) (660,360) (714,390)#11:(312,360) (348,366) (198,456) (162,468) (198,456) (402,444)#12:(468,384) (498,396) (348,474) (150,564) (114,576) (150,564) (576,540)#13:(480,474) (552,516) (594,552) (618,588)#14:(390,552) (390,708) (378,732) (348,762) (270,702)#15:(234,594) (276,612) (192,672) (120,714) (54,744)#16:(480,606) (540,636) (618,684) (690,738)']});hzbh.flash('繁','fj/fan7');
    */
   let s = html.split("hzbh.main(");
   if let Some(s) = s.skip(1).next(){
       let mut s = s.split(");");
       if let Some(s) = s.next(){
            //println!("{}", s);
            let s = s.split("{");
            if let Some(s) = s.skip(1).next(){
                //繁:[17, '0:(x,y)..#2:(x,y)..#3..']}
                
                let mut map = s.split(":[");
                let key = map.next().unwrap();
                let mut value = map.next().unwrap().trim_right_matches("']}").split(",'");
                let count = value.next().unwrap();
                let mut string = String::from(value.next().unwrap());
                println!("汉字={}", key);
                println!("笔画数={}", count);
                let mut result = vec![];
                string.replace_range(0..2, "");
                for i in 1..count.parse().unwrap(){
                    string = string.replace(&format!("#{}:", i), "#");
                }
                let arr = string.split("#");
                
                for b in arr{
                    if b.trim().len() == 0{
                        continue;
                    }
                    let mut points:Vec<[i32;2]> = b.split(" ").map(|p|{
                        let xy:Vec<&str> = p.trim_right_matches(")")
                        .trim_left_matches("(").split(",").collect();
                        [xy[0].parse().unwrap(), xy[1].parse().unwrap()]
                    }).collect();
                    result.push(points);
                }

                return Some((key.to_string(), result));
                
            }else{
                println!("没有找到花括号");
            }
       }else{
           println!("没有找到);");
       }
   }else{
       println!("没有找到hzbh.main(");
   }
   
   None
}