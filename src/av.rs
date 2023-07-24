use polars_core::prelude::*;
use anyhow::{Result, anyhow};
use chrono::NaiveDate;

pub enum TimeSeriesStep {
    Daily,
    Weekly,
    Monthly
}
pub enum AvFunctionCall {
    TimeSeries{
        step: TimeSeriesStep,
        symbol: String,
        outputsize: Option<String>, // compact || full
        datatype: Option<String>, // json || csv
        api_key: String,
    }, 
}
impl AvFunctionCall {
    pub fn build_url(&self) -> Result<String> {
        let mut query_url = String::from(
            "https://www.alphavantage.co/query?"
        );
        
        match self {
            AvFunctionCall::TimeSeries { 
                step, 
                symbol, 
                outputsize, 
                datatype, 
                api_key 
            } => {
    
                // append function call
                match step {
                    TimeSeriesStep::Daily => {
                        query_url.push_str("function=TIME_SERIES_DAILY");
                    },
                    TimeSeriesStep::Weekly => {
                        query_url.push_str("function=TIME_SERIES_WEEKLY");
                    },
                    TimeSeriesStep::Monthly => {
                        query_url.push_str("function=TIME_SERIES_MONTHLY");
                    }
                }
    
                // append query symbol
                let symbol_query = format!("&symbol={symbol}");
                query_url.push_str(&symbol_query);
    
                // append outputsize
                match step {
                    TimeSeriesStep::Daily => {
                        match outputsize {
                            Some(val) => {
                                let size_query = format!("&outputsize={val}");
                                query_url.push_str(&size_query);
                            }
                            None => {}
                        }
                    }
                    TimeSeriesStep::Weekly => {
                        match outputsize {
                            Some(_) => {
                                println!("Output size param not used for Weekly query.");
                            }
                            None => {}
                        }
                    }
                    TimeSeriesStep::Monthly => {
                        match outputsize {
                            Some(_) => {
                                println!("Output size param not used for Monthly query.");
                            }
                            None => {}
                        }
                    }
                }
                
                // append datatype
                match datatype {
                    Some(val) => {
                        let dtype_query = format!("&datatype={val}");
                        query_url.push_str(&dtype_query);
                    }
                    None => {}
                }
    
                // append apikey
                let key_query = format!("&apikey={api_key}");
                query_url.push_str(&key_query);
            }
        }
        Ok(query_url)
    }

    pub fn print_built_url(&self) {
        println!("{:?}", self.build_url())
    }
}


pub fn time_series_parser(csv_str: String) -> Result<DataFrame> {
    // df col initialization
    let mut timestamps_str = vec![];
    let mut opens:Vec<f64> = vec![];
    let mut highs:Vec<f64> = vec![];
    let mut lows:Vec<f64> = vec![];
    let mut closes:Vec<f64> = vec![];
    let mut volumns:Vec<i32> = vec![];

    let mut rdr = csv::Reader::from_reader(csv_str.as_bytes());
    for row in rdr.records() {
        match row {
            Ok(data) => {
                timestamps_str.push(data.get(0).unwrap().to_string());
                opens.push(data.get(1).unwrap().parse().unwrap());
                highs.push(data.get(2).unwrap().parse().unwrap());
                lows.push(data.get(3).unwrap().parse().unwrap());
                closes.push(data.get(4).unwrap().parse().unwrap());
                volumns.push(data.get(5).unwrap().parse().unwrap());
            }
            Err(e) => {
                println!("Err parsing csv to vecs");
            }
        }
    }

    // timestap to date objs
    let mut timestamps = vec![];
    let fmt = "%Y-%m-%d";
    for i in 0..timestamps_str.len() {
        if let Ok(date_obj) = NaiveDate::parse_from_str(timestamps_str.get(i).unwrap(), fmt) {
            timestamps.push(Some(date_obj))
        } else {
            timestamps.push(None)
        }
    }

    let timestamps = Series::new("timestamp", timestamps);
    let opens = Series::new("open", opens);
    let highs = Series::new("high", highs);
    let lows = Series::new("low", lows);
    let closes = Series::new("close", closes);
    let volumns = Series::new("volumn", volumns);

    let data = DataFrame::new(
        vec![timestamps, opens, highs, lows, closes, volumns],
    );

    // println!("{:?}", data.unwrap().sample_n(10, false, true, None));
    

    Ok(data.unwrap())
}
