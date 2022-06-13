extern crate reqwest;
extern crate serde;
use serde::{Deserialize, Serialize};

pub async fn ohlc(ticker: &'static str) -> OHLCData { // Result<FnInfo, &'static str> {
    let base_url = String::from("https://query2.finance.yahoo.com/v8/finance/chart/");
    let response = reqwest::get(base_url + ticker)
    	.await
    	.unwrap();
    
    let json_resp: StockAPIResponse = response.json().await.unwrap();
    let ohlc_data = &json_resp.chart.result.unwrap()[0].indicators.quote[0];

    OHLCData {
    	ticker: ticker,
    	open: *ohlc_data.open.last().unwrap(),
    	high: *ohlc_data.high.last().unwrap(),
    	low: *ohlc_data.low.last().unwrap(),
    	close: *ohlc_data.close.last().unwrap(),
    	volume: *ohlc_data.volume.last().unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct StockAPIResponse {
	chart: StockAPIChartResponse
}

#[derive(Serialize, Deserialize, Debug)]
struct StockAPIChartResponse {
	result: Option<Vec<StockData>>
}

#[derive(Serialize, Deserialize, Debug)]
struct StockData {
	indicators: StockIndicators
}

#[derive(Serialize, Deserialize, Debug)]
struct StockIndicators {
	quote: Vec<OHLCHistoricalData>
}

#[derive(Serialize, Deserialize, Debug)]
struct OHLCHistoricalData {
	open: Vec<f64>,
	high: Vec<f64>,
	low: Vec<f64>,
	close: Vec<f64>,
	volume: Vec<f64>
}

#[derive(Debug, Clone)]
pub struct OHLCData {
	ticker: &'static str,
	open: f64,
	high: f64,
	low: f64,
	close: f64,
	volume: f64
}