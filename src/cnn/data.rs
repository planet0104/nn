pub struct Data{
    set_in: Vec<Vec<f32>>,
    set_out: Vec<Vec<f32>>,
    names: Vec<String>,
    patterns: Vec<Vec<f32>>,
    num_patterns: usize,
    vector_size: usize,
}

impl Data{
    pub fn new(num_patterns: i32, vector_size:i32, vectors: &Vec<Vec<f32>>, names: &Vec<String>) -> Data{
        let mut data = Data{
            set_in: vec![],
            set_out: vec![],
            names: vec![],
            patterns: vec![],
            num_patterns: num_patterns as usize,
            vector_size: vector_size as usize
        };

        data.init(&vectors, &names);
        data.create_training_set_from_data();

        data
    }

    pub fn init(&mut self, vectors:&Vec<Vec<f32>>, names: &Vec<String>){
        //循环每个模式
        for ptn in 0..self.num_patterns{
            //添加到模式数组
            let mut tmp = vec![];
            for v in 0..self.vector_size*2{
                tmp.push(vectors[ptn][v]);
            }
            self.patterns.push(tmp);
            self.names.push(names[ptn].to_string());
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

    pub fn get_input_set(&self) -> &Vec<Vec<f32>>{
        &self.set_in
    }

    pub fn get_output_set(&self) -> &Vec<Vec<f32>>{
        &self.set_out
    }
}