
mod lib;
mod url;
mod structs;

use crate::url::url::hist_price;
use structs::{HistPrice, HistPriceRecords};

fn main() {
    let m = HistPriceRecords{
        records: 200
    };
    let t = HistPrice{
        records : vec![m]
    };
    println!("{:?}", t)

}