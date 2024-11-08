use fmp::Client;
use fmp::period::FMPPeriod;
use fmp::historical_price::*;
use reqwest::StatusCode;


#[tokio::main]
async fn main() {
    res().await;
}

async fn init_data() -> Client {
    let data = Client::new(
        "https://financialmodelingprep.com/api",
        "0mIYdxrV89xwrkb99a8zdcElTiQu44sn"
    );
    return data;
}

async fn res(){
    let data = init_data();
    let res: Result<Vec<FMPHistoricalPrice>, StatusCode> = data.await.historical_prices("AAPL").await;
    let vec: Vec<FMPHistoricalPrice> = res.unwrap();
    for i in 0..vec.len() {
        println!("{}", vec[i].change_percent);
    }

}


