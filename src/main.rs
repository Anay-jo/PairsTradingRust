use fmp::Client;
use fmp::period::FMPPeriod;
use fmp::historical_price::*;
use reqwest::StatusCode;
use std::env;
use std::io;
use std::thread;
use std::io::Write;

#[tokio::main]
async fn main() {
    println!("Enter ticker symbol 1 here: ");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input);
    input = input.trim().to_string();

    println!("Enter ticker symbol 2 here: ");
    let mut input2: String = String::new();
    io::stdin().read_line(&mut input2);
    input2 = input2.trim().to_string();

    println!("Enter how much money you want to invest: ");
    let mut money: String = String::new();
    io::stdin().read_line(&mut money);

    let money2: f64 = match money.trim().parse::<f64>() {
        Ok(num) => num,
        Err(e) => 0.0,
    };

    let mut data: Vec<FMPHistoricalPrice> = init_data(&input).await;
    let mut data2: Vec<FMPHistoricalPrice> = init_data(&input2).await;
    let mut currdata = Vec::new();
    for i in 0..90 {
        currdata.push(data[i].change_percent);
    }
    let mut currdata2 = Vec::new();
    for i in 0..90 {
        currdata2.push(data2[i].change_percent);
    }
    let mut x = data.len()%90;
    for i in 0..x {
        data.pop();
    }
    x = data2.len()%90;
    for i in 0..x {
        data2.pop();
    }
    let mut min_diff: f64 = -1.0;
    let mut min_idx: usize = 0;
    let mut i = 90;
    while i + 90 <= data.len() && i + 90 <= data2.len() {
        let mut temp_vec = Vec::new();
        let mut temp_vec2 = Vec::new();
        for j in i..i+90 {
            temp_vec.push(data[j].change_percent);
            temp_vec2.push(data2[j].change_percent);
        }
        let temp_val = calculate_diff(&temp_vec, &currdata) + calculate_diff(&temp_vec2, &currdata2);
        if min_diff < 0.0 || temp_val < min_diff {
            min_diff = temp_val;
            min_idx = i;
        }
        i += 1;
    }

    // let mut data2: Vec<FMPHistoricalPrice> = init_data(&input2).await;
    // let mut currdata2 = Vec::new(); //present block
    // for i in 0..90 {
    //     currdata2.push(data2[i].change_percent);
    // }
    // let x = data2.len()%90;
    // for i in 0..x {
    //     data2.pop();
    // }
    // let mut min_diff2:f64 = -1.0;
    // let mut min_idx2:usize = 0;
    // let mut i2 = 90;
    // while i2 + 90 <= data2.len() {
    //     let mut temp_vec2 = Vec::new();
    //     for j in i2..i2+90 {
    //         temp_vec2.push(data2[j].change_percent);
    //     }
    //     let temp_val2 = calculate_diff(&temp_vec2, &currdata2);
    //     if min_diff2 < 0.0 || temp_val2 < min_diff2 {
    //         min_diff2 = temp_val2;
    //         min_idx2 = i2;
    //     }
    //     i2+=1;
    // }

    println!("{}", data[min_idx].date);
    // println!("{}", data2[min_idx2].date);
    let change1: f64 = calculate_change(data, min_idx).await;
    let change2: f64 = calculate_change(data2, min_idx).await;
    let change3: f64 = change1.abs();
    let change4: f64 = change2.abs();
    let percent1: f64 = money2 * change3 / (change3 + change4);
    let percent2: f64 = money2 * change4 / (change3 + change4);
    println!("{}", change1);
    println!("{}", change2);
    if (change1 > 0.0 && change2 > 0.0) || (change1 < 0.0 && change2 < 0.0) {
        if change1 > change2 {
            println!("Long {} with ${} and short {} with ${}.", input, round_to_places(percent1, 2).await, input2, round_to_places(percent2, 2).await);
        } else {
            println!("Long {} with ${} and short {} with ${}.", input2, round_to_places(percent2, 2).await, input, round_to_places(percent1, 2).await);
        }
    } else {
        if change1 > 0.0 {
            println!("Long {} with ${} and short {} with ${}.", input, round_to_places(percent1, 2).await, input2, round_to_places(percent2, 2).await);
        } else {
            println!("Long {} with ${} and short {} with ${}.", input2, round_to_places(percent2, 2).await, input, round_to_places(percent1, 2).await);
        }
    }
}

async fn calculate_change(data: Vec<FMPHistoricalPrice>, min_idx: usize) -> f64 {
    (data[min_idx - 90].close - data[min_idx - 1].close) / data[min_idx - 1].close
}

async fn round_to_places(num: f64, places: u32) -> f64 {
    let scale = 10f64.powi(places as i32);
    (num * scale).round() / scale
}

async fn init_data(ticker: &str) -> Vec<FMPHistoricalPrice> {
    let data = Client::new(
        "https://financialmodelingprep.com/api",
        "0mIYdxrV89xwrkb99a8zdcElTiQu44sn"
    );
    let res: Result<Vec<FMPHistoricalPrice>, StatusCode> = data.historical_prices(ticker).await;
    let vec: Vec<FMPHistoricalPrice> = res.unwrap();
    println!("Change in percent during {} was {}%", vec[0].date, vec[0].change_percent);
    vec
}

//calculates trend difference
fn calculate_diff(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 0..90 {
        sum += (vec1[i] - vec2[i]) * (vec1[i] - vec2[i]);
    } 
    sum
}
