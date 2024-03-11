use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct HistPriceRecords{
    pub records: u64
}

#[derive(Debug, Serialize)]
pub struct HistPrice{
    #[serde(rename(serialize ="closingPriceDaily"))]
   pub records: Vec<HistPriceRecords>
}

