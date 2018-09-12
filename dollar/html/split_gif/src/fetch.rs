
// pub fn fetch_all(){
//     let contents = {
//         let mut contents = String::new();
//         let mut file = File::open("yq.txt").unwrap();
//         file.read_to_string(&mut contents).unwrap();
//         contents
//     };

//     for line in contents.lines(){
//         for ch in line.chars(){
//             let file = File::open(format!("data/{}.stroke", ch));
//             let file_err = File::open(format!("strokes_new/{}.stroke_err", ch));
//             if file.is_err() && file_err.is_err(){
//                 //println!("没有{}的文件", ch);
//                 let result = fetch_original_stroke(&ch);
//                 if let Some(strokes) = result{
//                     if strokes.1.len() == 0 && strokes.0.len() ==0{
//                         //超时的文件
//                         let mut file = File::create(format!("strokes_new/{}.stroke_time_out", ch)).unwrap();
//                         file.write_all(b"").unwrap();
//                     }else{
//                         //println!("{:?}", strokes);
//                         let strokes = strokes.1;
//                         let encoded: Vec<u8> = serialize(&strokes).unwrap();
//                         let mut file = File::create(format!("strokes_new/{}.stroke", ch)).unwrap();
//                         file.write_all(&encoded).unwrap();
//                     }
//                 }else{
//                     //println!("找不到{}的笔画，删除", ch);
//                     let mut file = File::create(format!("strokes_new/{}.stroke_err", ch)).unwrap();
//                     file.write_all(b"").unwrap();
//                 }
//             }else{
//                 //读取笔画放入map
//                 if !file.is_err(){
//                     let mut contents = vec![];
//                     file.unwrap().read_to_end(&mut contents).unwrap();
//                     let mut data:Vec<Vec<[i32;2]>> = deserialize(&contents[..]).unwrap();
//                     all_map.insert(ch, data);   
//                 }
//             }
//         }    
//     }

//     //保存原始笔画文件
//     println!("原始笔画 文字数量:{}", all_map.len());
//     let encoded: Vec<u8> = serialize(&all_map).unwrap();
//     let mut file = File::create("original_stroke_data").unwrap();
//     file.write_all(&encoded).unwrap();
// }