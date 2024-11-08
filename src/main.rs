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
    println!("Enter ticker symbol here: ");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input);

    let mut data: Vec<FMPHistoricalPrice> = init_data(&input).await;
    let mut currdata = Vec::new(); //present block
    for i in 0..90 {
        currdata.push(data[i].change_percent);
    }
    let x = data.len()%90;
    for i in 0..x {
        data.pop();
    }
    let mut min_diff:f64 = -1.0;
    let mut min_idx:usize = 0;
    let mut i = 90;
    while i + 90 <= data.len() {
        let mut temp_vec = Vec::new();
        for j in i..i+90 {
            temp_vec.push(data[j].change_percent);
        }
        let temp_val = calculate_diff(&temp_vec, &currdata);
        if min_diff < 0.0 || temp_val < min_diff {
            min_diff = temp_val;
            min_idx = i;
        }
        i+=1;
    }
    println!("{}", data[min_idx].date);
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
        sum += vec1[i] - vec2[i];
    } 
    sum * sum
}