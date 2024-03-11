pub mod url {
    fn base() -> String {
        String::from("https://cdn.tsetmc.com/api")
    }

    pub fn hist_price(ins_code: String) -> String {
        format!(
            "{}/ClosingPrice/GetClosingPriceDailyList/{ins_code}", base()
        )
    }
}
