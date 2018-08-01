use matrix2d::Matrix2D;
//use std::fs::File;
//use std::io::{ Write,Read };

// 神经网络
pub struct NeuralNetwork{
    inodes:usize, //输入节点数
    hnodes:usize, //隐藏节点数
    onodes:usize, //输出节点数
    lr:f32,  //学习率

    // 连接权重矩阵, wih (W input_hidden) 和 who (W hidden_output)
    // 数组中的权重是 w_i_j, 链接是从节点i到下一层的节点j
    // w11  w21
    // w12  w22 等
    wih:Matrix2D,
    who:Matrix2D,
}

impl NeuralNetwork{
    //初始化神经网络
    pub fn new(input_nodes:usize, hidden_nodes:usize, output_nodes:usize, learning_rate:f32)->NeuralNetwork{
        NeuralNetwork{
            inodes: input_nodes,
            hnodes: hidden_nodes,
            onodes: output_nodes,
            lr: learning_rate,
            wih: Matrix2D::random(hidden_nodes, input_nodes),
            who: Matrix2D::random(output_nodes, hidden_nodes),
        }
    }

    //训练神经网络
    pub fn train(&mut self, inputs_list:&Vec<f32>, targets_list:&Vec<f32>){
        //1维数组 转换成2维数组
        let inputs = Matrix2D::from(inputs_list);
        let targets = Matrix2D::from(targets_list);

        //计算进入隐藏层的信号
        let hidden_inputs =  Matrix2D::dot(&self.wih, &inputs);
        //计算从隐藏层产生的信号
        let hidden_outputs = Matrix2D::sigmoid(&hidden_inputs);

        //计算进入最终输出层的信号
        let final_inputs = Matrix2D::dot(&self.who, &hidden_outputs);
        //计算输出层产生的信号
        let final_outputs = Matrix2D::sigmoid(&final_inputs);

        //错误 = 目标-实际输出
        let output_errors =  targets - final_outputs.clone();

        //隐藏层的错误
        //error_hidden = W(T)_hidden_output ● error_output
        let hidden_errors = Matrix2D::dot(&Matrix2D::transpose(&self.who), &output_errors);

        //我们需要调整每层的权重。
        //"隐藏层->输出层"的权重使用output_errors(来调整)
        //"输入层->隐藏层"的权重使用hidden_errors(来调整)

        //更新隐藏层到输出层之间连接的权重
        self.who += self.lr * Matrix2D::dot(&(output_errors * final_outputs.clone() * (1.0 - final_outputs)), &Matrix2D::transpose(&hidden_outputs));
        
        //更新输入层和隐藏层之间连接的权重
        self.wih += self.lr * Matrix2D::dot(&(hidden_errors * hidden_outputs.clone() * (1.0-hidden_outputs)), &Matrix2D::transpose(&inputs));
    }

    //查询神经网络
    pub fn query(&self, inputs_list:&Vec<f32>)->Matrix2D{
        //1维数组 转换成2维数组
        let inputs = Matrix2D::from(inputs_list);
        //计算进入隐藏层的信号
        let hidden_inputs =  Matrix2D::dot(&self.wih, &inputs);
        //计算从隐藏层产生的信号
        let hidden_outputs = Matrix2D::sigmoid(&hidden_inputs);

        //计算进入最终输出层的信号
        let final_inputs = Matrix2D::dot(&self.who, &hidden_outputs);
        //计算输出层产生的信号
        let final_outputs = Matrix2D::sigmoid(&final_inputs);

        final_outputs
    }

    //反向查询网络
    //我们将对每个项目使用相同的术语，
    //例如，target是网络右侧的值，尽管用作输入
    //例如，hidden_output是中间节点右侧的信号
    pub fn backquery(&self, targets_list:&Vec<f32>)->Matrix2D{
        //将目标列表转置为垂直数组
        let final_outputs = Matrix2D::from(targets_list);
        
        //将信号计算到最终输出层
        let final_inputs = Matrix2D::inverse_sigmoid(&final_outputs);

        //计算隐藏层中的信号
        let mut hidden_outputs = Matrix2D::dot(&Matrix2D::transpose(&self.who), &final_inputs);
        //将它们缩放到0.01到.99

        hidden_outputs -= hidden_outputs.min();
        hidden_outputs /= hidden_outputs.max();
        hidden_outputs *= 0.98;
        hidden_outputs += 0.01;

        
        //将信号计算到隐藏层
        let hidden_inputs = Matrix2D::inverse_sigmoid(&hidden_outputs);
        
        //计算出输入层的信号
        let mut inputs = Matrix2D::dot(&Matrix2D::transpose(&self.wih), &hidden_inputs);
        //将它们缩放到0.01到.99
        inputs -= inputs.min();
        inputs /= inputs.max();
        inputs *= 0.98;
        inputs += 0.01;
        
        inputs
    }

    //存储到文件
    // pub fn store_to_file(&self, file_name:&String){
    //     let inodes = format!("{}\n", self.inodes);
    //     let hnodes = format!("{}\n", self.hnodes);
    //     let onodes = format!("{}\n", self.onodes);
    //     let lr = format!("{}\n", self.lr);
    //     let mut file = File::create(file_name).unwrap();
    //     let mut data_str = String::new();
    //     data_str.push_str(&inodes);
    //     data_str.push_str(&hnodes);
    //     data_str.push_str(&onodes);
    //     data_str.push_str(&lr);

    //     for row in self.wih.matrix(){
    //         for col in row{
    //             //逗号分割每一个权重
    //             data_str.push_str(&format!("{},", col));
    //         }
    //         //去掉最后一个逗号
    //         data_str.pop();
    //         //空格分割每一行
    //         data_str.push_str(" ");
    //     }
    //     //去掉最后一个空格
    //     data_str.pop();
    //     data_str.push_str("\n");

    //     for row in self.who.matrix(){
    //         for col in row{
    //             //逗号分割每一个权重
    //             data_str.push_str(&format!("{},", col));
    //         }
    //         //去掉最后一个逗号
    //         data_str.pop();
    //         //空格分割每一行
    //         data_str.push_str(" ");
    //     }
    //     //去掉最后一个空格
    //     data_str.pop();

    //     file.write_all(data_str.as_bytes());
    // }

    //读入文件
    // pub fn from_file(file_name:&String)->Option<NeuralNetwork>{
    //     let mut data_file = match File::open(file_name) {
    //         Ok(file) => file,
    //         Err(err) => return None,
    //     };

    //     let mut data_str = String::new();
    //     data_file.read_to_string(&mut data_str).unwrap();
    //     let data_list: Vec<&str> = data_str.split('\n').collect();
    //     let inodes:usize = data_list[0].parse().unwrap();
    //     let hnodes:usize = data_list[1].parse().unwrap();
    //     let onodes:usize = data_list[2].parse().unwrap();
    //     let lr:f32 = data_list[3].parse().unwrap();

    //     let wih:Vec<Vec<f32>> = data_list[4].split(' ').map(|line| line.split(',').map(|val| val.parse().unwrap()).collect()).collect();
    //     let who:Vec<Vec<f32>> = data_list[5].split(' ').map(|line| line.split(',').map(|val| val.parse().unwrap()).collect()).collect();

    //     Some(NeuralNetwork{
    //         inodes: inodes,
    //         hnodes: hnodes,
    //         onodes: onodes,
    //         lr: lr,
    //         wih: Matrix2D::new(wih),
    //         who: Matrix2D::new(who)
    //     })
    // }
}
