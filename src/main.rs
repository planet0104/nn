extern crate rand;

mod matrix2d;
mod nn;
use nn::NeuralNetwork;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use	std::time::{Instant};
use matrix2d::Matrix2D;

fn main() {
    println!("开始训练网络...");
    //输入、隐藏、输出层的节点数
    let input_nodes:usize = 784;
    let hidden_nodes:usize = 200;
    let output_nodes:usize = 10;
    //学习率
    let learning_rate:f32 = 0.1;
    
    //创建神经网络实例
    let mut network = NeuralNetwork::new(input_nodes, hidden_nodes, output_nodes, learning_rate);
    
    //读取训练数据
    let mut data_file = File::open("mnist_train_1000.csv").unwrap();
    let mut data_str = String::new();
    data_file.read_to_string(&mut data_str).unwrap();
    let training_data_list: Vec<&str> = data_str.split('\n').collect();

    //训练神经网络
    //let mut start_time = Instant::now();
    let epochs = 5;
    for e in 0..epochs{
        println!("Epochs:{}", e+1);
        //处理每一条记录
        for record in &training_data_list{
            //逗号分割记录
            println!("{:?}", record.split(","));
            let all_values:Vec<u8> = record.split(",").map(|s| u8::from_str(s).unwrap()).collect();
            //转换输入值
            let inputs:Vec<f32> = all_values.get(1..all_values.len()).unwrap().iter().map(|u| (*u as f32/255f32*0.99)+0.01).collect();
            //创建目标输出值(除了期望值为0.99外，其他均为0.01)
            let mut targets:Vec<f32> = vec![0.01; output_nodes];
            //all_values[0]存储的是目标位置
            targets[all_values[0] as usize] = 0.99;
            network.train(&inputs, &targets);
        }
        //print_elapsed(1, &mut start_time);
    }
    println!("训练完毕.");
    //query_test(&network, "number_train_10.csv");
    query_test(&network, "mnist_test.csv");
}

fn query_test(network:&NeuralNetwork, test_file_name:&str){
    //读取训练数据
    let mut data_file = File::open(test_file_name).unwrap();
    let mut data_str = String::new();
    data_file.read_to_string(&mut data_str).unwrap();

    //scorecard 记录网络得分
    let test_data_list: Vec<&str> = data_str.split('\n').collect();
    let mut scorecard:Vec<i32> = vec![];

    for record in test_data_list{
        //逗号分割记录
        let all_values:Vec<u8> = record.split(",").map(|s| u8::from_str(s).unwrap()).collect();
        //正确答案为第一个值
        let correct_label = all_values[0] as usize;
        //println!("正确答案:{}", correct_label);
        //转换输入值
        let inputs:Vec<f32> = all_values.get(1..all_values.len()).unwrap().iter().map(|u| (*u as f32/255f32*0.99)+0.01).collect();
        //查询网络
        let outputs = network.query(&inputs);
        let label = max_index(&outputs);
        //println!("网络输出:{}", label);
        scorecard.push(if label == correct_label{
            1
        }else{
            0
        });
    }
    println!("绩效得分:{}", scorecard.iter().fold(0, |sum, x| sum + x) as f32/ scorecard.len() as f32);
}

fn max_index(matrx:&Matrix2D)->usize{
    let values = matrx.matrix();
    let mut max_val = &values[0];
    let mut max_index = 0usize;
    for i in 1..values.len() {
        if values[i] > *max_val {
            max_val = &values[i];
            max_index = i;
        }
    }
    max_index
}