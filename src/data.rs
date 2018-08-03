
use cnn::Data as TData;
use ::controller::{NUM_VECTORS, NUM_PATTERNS};

const INPUT_VECTORS:[[f32; NUM_VECTORS as usize*2]; NUM_PATTERNS as usize] = [
	//右
	[1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0],
	//左
	[-1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0],
	//下
	[0.0,1.0, 0.0,1.0, 0.0,1.0, 0.0,1.0, 0.0,1.0, 0.0,1.0, 0.0,1.0, 0.0,1.0, 0.0,1.0, 0.0,1.0, 0.0,1.0, 0.0,1.0],
	//上
	[0.0,-1.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0],
	//顺时针正方形
	[1.0,0.0, 1.0,0.0, 1.0,0.0, 0.0,1.0, 0.0,1.0, 0.0,1.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0],
	//逆时针正方形
	[-1.0,0.0, -1.0,0.0, -1.0,0.0, 0.0,1.0, 0.0,1.0, 0.0,1.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 0.0,-1.0, 0.0,-1.0, 0.0,-1.0],
	//右箭头
	[1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, -0.45,0.9, -0.9, 0.45, -0.9,0.45],
	//左箭头
	[-1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, -1.0,0.0, 0.45,0.9, 0.9, 0.45, 0.9,0.45],
	//西南
	[-0.7,0.7, -0.7,0.7, -0.7,0.7, -0.7,0.7, -0.7,0.7, -0.7,0.7, -0.7,0.7, -0.7,0.7,-0.7,0.7, -0.7,0.7, -0.7,0.7, -0.7,0.7],
	//东南
	[0.7,0.7, 0.7,0.7, 0.7,0.7, 0.7,0.7, 0.7,0.7, 0.7,0.7, 0.7,0.7, 0.7,0.7, 0.7,0.7,0.7,0.7, 0.7,0.7, 0.7,0.7],
	//zorro
	[1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, -0.72,0.69,-0.7,0.72,0.59,0.81, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0, 1.0,0.0]
]; 

const NAMES: [&str; NUM_PATTERNS as usize]=[
  "右",
  "左",
  "下",
  "上",
  "顺时针正方形",
  "逆时针正方形",
  "右箭头",
  "左箭头",
  "西南",
  "东南",
  "zorro"
];

pub struct Data{
    set_in: Vec<Vec<f32>>,
    set_out: Vec<Vec<f32>>,
    names: Vec<String>,
    patterns: Vec<Vec<f32>>,
    num_patterns: usize,
    vector_size: usize,
}

impl TData for Data{
    fn get_input_set(&self) -> &Vec<Vec<f32>>{
        &self.set_in
    }

    fn get_output_set(&self) -> &Vec<Vec<f32>>{
        &self.set_out
    }
}

impl Data{
    pub fn new(num_patterns: i32, vector_size:i32) -> Data{
        let mut data = Data{
            set_in: vec![],
            set_out: vec![],
            names: vec![],
            patterns: vec![],
            num_patterns: num_patterns as usize,
            vector_size: vector_size as usize
        };

        data.init();
        data.create_training_set_from_data();

        data
    }

    pub fn init(&mut self){
        //循环每个模式
        for ptn in 0..self.num_patterns{
            //添加到模式数组
            let mut tmp = vec![];
            for v in 0..self.vector_size*2{
                tmp.push(INPUT_VECTORS[ptn][v]);
            }

            self.patterns.push(tmp);
            self.names.push(NAMES[ptn].to_string());
        }
    }

    pub fn create_training_set_from_data(&mut self){
        self.set_in.clear();
        self.set_out.clear();

        //添加每个模式
        for ptn in 0..self.num_patterns{
            //将数据添加到训练集合
            self.set_in.push(self.patterns[ptn].clone());
            //创建输出数组
            let mut outputs = vec![0.0; self.num_patterns];
            outputs[ptn] = 0.99;
            self.set_out.push(outputs);
        }
    }

    pub fn pattern_name(&self, val: usize) -> &str{
        if self.names.len() > val{
            &self.names[val]
        }else{
            ""
        }
    }
}