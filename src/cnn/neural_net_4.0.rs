//------------------------------------------------------------------------
//
//名称：neural_net.rs
//
//作者：Mat Buckland 2002
//
// Desc：用于创建前馈神经网络的类。来自神经网络和遗传算法的游戏AI编程。
//-------------------------------------------------------------------------

use cnn::utils;
use std::f32::consts::E;

//-----------------------------------------------
//  used in CNeuralNet
//-----------------------------------------------
const ACTIVATION_RESPONSE:f32 = 1.0;
const BIAS: f32 = -1.0;

//当总误差低于该值时，后备停止训练
const ERROR_THRESHOLD:f32 = 0.02;
const MOMENTUM:f32 = 0.9;

/// 定义输入或输出向量的类型（在训练方法中使用）
pub type IoVector = Vec<f32>;

pub trait Data{
    fn get_input_set(&self) -> &Vec<Vec<f32>>;
    fn get_output_set(&self) -> &Vec<Vec<f32>>;
}

//定义神经元结构
pub struct Neuron{
    //进入神经元的输入数量
    num_inputs: i32,
    //每个输入的权重
    weight: Vec<f32>,

    //之前的时间步长权重更新用于增加动量
    prev_update: Vec<f32>,

    //激活这个神经元
    activation: f32,

    //错误值
    error: f32,
}

impl Neuron{
    //构造函数
    pub fn new(num_inputs: i32) -> Neuron{
        let mut neuron = Neuron{
            num_inputs: num_inputs+1,
            activation: 0.0,
            error: 0.0,
            weight: vec![],
            prev_update: vec![]
        };

        //我们需要额外的偏差权重，因此+1
        for _ in 0..num_inputs + 1{
            //使用初始随机值设置权重
            neuron.weight.push(utils::random_clamped());
            neuron.prev_update.push(0.0);
        }

        neuron
    }
}

//---------------------------------------------------------------------
// struct用于保存一层神经元。
//---------------------------------------------------------------------

pub struct NeuronLayer{
    //该层中的神经元数量
    num_neurons: i32,

    //神经元层
    neurons: Vec<Neuron>,
}

impl NeuronLayer{
    //-----------------------------------------------------------------------
    // ctor通过调用SNeuron ctor rqd次数来创建一个所需大小的神经元层
    //-----------------------------------------------------------------------
    pub fn new(num_neurons:i32, num_inputs_per_neuron:i32) -> NeuronLayer{
        let mut layer = NeuronLayer{
            num_neurons,
            neurons: vec![],
        };

        for _ in 0..num_neurons{
            layer.neurons.push(Neuron::new(num_inputs_per_neuron));
        }

        layer
    }
}


pub struct NeuralNet{
    num_inputs: i32,

    num_outputs: i32,

    num_hidden_layers: i32,

    neurons_per_hidden_lyr: i32,

    //我们必须为backprop指定学习率
    learning_rate: f32,

    //网络的累积误差（总和（输出 - 预期））
    error_sum: f32,

    //如果网络已经过培训，则为true
    trained: bool,

    //如果需要softmax输出，则设置为TRUE
    soft_max: bool,

    //纪元柜台
    num_epochs: i32,

    //存储每层神经元，包括输出层
    layers: Vec<NeuronLayer>,
}

impl NeuralNet{
    

    pub fn new(num_inputs: i32,
                num_outputs: i32,
                hidden_neurons: i32,
                learning_rate: f32,
                soft_max: bool)-> NeuralNet{
        let mut net = NeuralNet{
            num_inputs,
            num_outputs,
            num_hidden_layers: 1,
            soft_max,
            neurons_per_hidden_lyr: hidden_neurons,
            learning_rate,
            error_sum: 9999.0,
            trained: false,
            num_epochs: 0,
            layers: vec![]
        };

        net.create_net();

        net
    }


	//计算一组输入的输出
    pub fn update(&mut self, inputs: &Vec<f32>) -> Vec<f32>{
        //存储每层的结果输出
        let mut outputs = vec![];
        let mut inputs = inputs.clone();

        //首先检查我们是否有正确的输入量
        if inputs.len() != self.num_inputs as usize{
            //如果不正确，只返回一个空向量。
            return outputs;
        }
        
        //For each layer...
        for i in 0..self.num_hidden_layers as usize + 1{
            
            if i > 0{
                inputs.clear();
                //将outputs放入inputs, 并清空outputs
                inputs.append(&mut outputs);
            }
            
            let mut weight = 0;

            //为每个神经元求和（输入*对应的权重）。在我们的sigmoid函数中输出总数以获得输出。
            for n in 0..self.layers[i].num_neurons as usize{
                let mut netinput = 0.0;

                let num_inputs = self.layers[i].neurons[n].num_inputs as usize;
                
                //每个权重
                for k in 0..num_inputs - 1{
                    //sum the weights x inputs
                    netinput += self.layers[i].neurons[n].weight[k] * 
                        inputs[weight];
                    weight += 1;
                }

                //add in the bias
                netinput += self.layers[i].neurons[n].weight[num_inputs-1] * BIAS;

                
                //softmax on output layers
                if self.soft_max && (i == self.num_hidden_layers as usize){
                    self.layers[i].neurons[n].activation = netinput.exp();
                }else{
                    //首先通过sigmoid函数过滤组合激活，并为每个神经元保留记录
                    self.layers[i].neurons[n].activation = NeuralNet::sigmoid(netinput, ACTIVATION_RESPONSE);
                }

                //在生成它们时存储每个层的输出。
                outputs.push(self.layers[i].neurons[n].activation);
                weight = 0;
            }
        }

        if self.soft_max{
            let mut exp_tot = 0.0;

            //首先计算输出总和的exp
            for o in 0..outputs.len(){
                exp_tot += outputs[o];
            }

            //现在相应地调整每个输出
            for o in 0..outputs.len(){
                outputs[o] = outputs[o]/exp_tot;
                self.layers[self.num_hidden_layers as usize].neurons[o].activation = outputs[o];    
            }
        }

        outputs
    }

    ///给定训练集，此方法执行反向传播算法的一次迭代。
    ///训练集包括一系列矢量输入和一系列预期矢量输出。
    ///返回错误总和
    fn network_training_epoch(&mut self, set_in: &Vec<IoVector>, set_out: &Vec<IoVector>) -> Result<(), &str>{
        //这将保留训练集的累积误差值
        self.error_sum = 0.0;

        //通过网络运行每个输入模式，计算错误并相应地更新权重
        for vec in 0..set_in.len(){
            //首先通过网络运行此输入向量并检索输出
            let outputs = self.update(&set_in[vec]);

            //如果发生错误则返回
            if outputs.len() == 0{
                return Err("network_training_epoch 网络更新出错!");
            }

            //对于每个输出神经元计算误差并相应地调整权重
            for op in 0..self.num_outputs as usize{
                //首先计算误差值
                let err = (set_out[vec][op] - outputs[op]) * outputs[op]
                            * (1.0 - outputs[op]);

                //记录错误值
                self.layers[1].neurons[op].error = err;

                let mut cur_index = 0;

                //循环每个权重，但不包括偏移
                while cur_index < self.layers[1].neurons[op].weight.len()-1{
                    //计算权重更新
                    let weight_update = err * self.learning_rate * self.layers[0].neurons[cur_index].activation;
                    
                    //根据backprop规则计算新权重并添加动量
                    self.layers[1].neurons[op].weight[cur_index] += weight_update + self.layers[1].neurons[op].prev_update[cur_index] * MOMENTUM;

                    //保留此权重更新的记录
                    self.layers[1].neurons[op].prev_update[cur_index] = weight_update;

                    cur_index += 1;
                }

                //当前神经元的偏移
                let weight_update = err * self.learning_rate * BIAS;

                self.layers[1].neurons[op].weight[cur_index] += weight_update + self.layers[1].neurons[op].prev_update[cur_index] * MOMENTUM;

                //保留此重量更新的记录
                self.layers[1].neurons[op].prev_update[cur_index] = weight_update;
            }

            //更新错误总数。 （当此值低于预设阈值时，我们知道培训成功）
            let mut error = 0.0;

            if !self.soft_max{ //Use SSE
                for o in 0..self.num_outputs as usize{
                    error += (set_out[vec][o] - outputs[o]) *
                            (set_out[vec][o] - outputs[o]);
                }
            }else{//使用交叉熵错误
                for o in 0..self.num_outputs as usize{
                    error += set_out[vec][o] * outputs[o].log(E);
                }

                error = -error;
            }
            
            self.error_sum += error;


            //**向后移动到隐藏层**
            
            //对于隐藏层中的每个神经元计算误差信号，然后相应地调整权重
            for cur_nrn_hid in 0..self.layers[0].neurons.len(){
                let mut err = 0.0;

                //为了计算这个神经元的误差，我们需要迭代它所连接的输出层中的所有神经元并求和误差*权重
                for cur_nrn_out in 0..self.layers[1].neurons.len(){
                    err += self.layers[1].neurons[cur_nrn_out].error * self.layers[1].neurons[cur_nrn_out].weight[cur_nrn_hid];
                }

                //现在我们可以计算错误
                err *= self.layers[0].neurons[cur_nrn_hid].activation * (1.0 - self.layers[0].neurons[cur_nrn_hid].activation);     
                
                //对于该神经元中的每个权重，基于误差信号和学习速率计算新的权重
                let mut w = 0;
                while w<self.num_inputs as usize{
                    let weight_update = err * self.learning_rate * set_in[vec][w];

                    //根据backprop规则计算新权重并添加动量
                    self.layers[0].neurons[cur_nrn_hid].weight[w] += weight_update + self.layers[0].neurons[cur_nrn_hid].prev_update[w] * MOMENTUM;
                    
                    //保留此重量更新的记录
                    self.layers[0].neurons[cur_nrn_hid].prev_update[w] = weight_update;

                    w += 1;
                }

                //偏移
                let weight_update = err * self.learning_rate * BIAS;

                self.layers[0].neurons[cur_nrn_hid].weight[self.num_inputs as usize] += weight_update + self.layers[0].neurons[cur_nrn_hid].prev_update[w] * MOMENTUM;

                //保留此重量更新的记录
                self.layers[0].neurons[cur_nrn_hid].prev_update[w] = weight_update;
            }

        }//next input vector
        
        Ok(())
    }

    //以CData对象的形式给出一些训练数据这个函数
    //训练网络，直到误差在可接受的范围内。
    pub fn train(net: &mut NeuralNet, data: &Data) -> Result<(), String>{
        let set_in  = data.get_input_set();
        let set_out = data.get_output_set();

        //首先确保训练集有效
        if set_in.len()  != set_out.len()  || 
            set_in[0].len()  != net.num_inputs as usize   ||
            set_out[0].len() != net.num_outputs as usize {
            return Err("输入不等于输出!".to_string());
        }
        
        //将所有权重初始化为小的随机值
        net.initialize_network();

        //使用backprop训练直到SSE低于用户定义的阈值
        let mut i = 0;
        while net.error_sum > ERROR_THRESHOLD{
            //if i%5000==0{
            println!("epochs={} error_sum={}", net.num_epochs, net.error_sum);
           // }
            //如果有任何问题，则返回false
            match net.network_training_epoch(set_in, set_out){
                Err(err) =>{
                    return Err(err.to_string());
                }
                Ok(()) => ()
            }

            net.num_epochs += 1;
            
            //调用render例程来显示错误总和
            //InvalidateRect(hwnd, NULL, TRUE);
                //UpdateWindow(hwnd);

            i += 1;
        }

        println!("epochs={} error_sum={}", net.num_epochs, net.error_sum);

        net.trained = true;
        
        Ok(())
    }

  
    pub fn rained(&self) -> bool{ self.trained }
    pub fn error(&self) -> f32{ self.error_sum}
    pub fn epoch(&self) -> i32 { self.num_epochs}

    //------------------------------ createNet（）------------------------------
    //
    //这个方法构建了ANN。 最初将权重设置为随机值-1 <w <1
    //------------------------------------------------------------------------
    fn create_net(&mut self){
        //创建网络层
        if self.num_hidden_layers>0{
            //创建第一个隐藏图层
            self.layers.push(NeuronLayer::new(self.neurons_per_hidden_lyr, self.num_inputs));
                
            for _ in 0..self.num_hidden_layers-1{
                self.layers.push(NeuronLayer::new(self.neurons_per_hidden_lyr,
                                                    self.neurons_per_hidden_lyr));
            }

            //创建输出图层
            self.layers.push(NeuronLayer::new(self.num_outputs, self.neurons_per_hidden_lyr));
        }else{
            //创建输出图层
            self.layers.push(NeuronLayer::new(self.num_outputs, self.num_inputs));
        }
    }

    //将所有权重设置为小的随机值
    fn initialize_network(&mut self){
        //为每一层
        for i in 0..self.num_hidden_layers as usize+1{
            //为每个神经元
            for n in 0..self.layers[i].num_neurons as usize{
                //每个权重
                for k in 0..self.layers[i].neurons[n].num_inputs as usize{
                    self.layers[i].neurons[n].weight[k] = utils::random_clamped();
                }
            }
        }

        self.error_sum = 9999.0;
        self.num_epochs = 0;
    }
  
    // Sigmoid响应曲线
    fn sigmoid(netinput: f32, response:f32) -> f32{
        1.0 / ( 1.0 + (-netinput / response).exp())
    }
}