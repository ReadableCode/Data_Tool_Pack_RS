pub struct Config {
    pub priv_key: String,
    pub sheet_id: String,
    pub deposit_range_input: String,
    pub deposit_range_output: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            priv_key: String::from("priv_key.json"),
            sheet_id: String::from("1pvmIGeanVd0mjIO4-y53OY-z-ueLIY1AF7e-KZGAMzI"),
            deposit_range_input: String::from("Deposits Stream!A2:D"),
            deposit_range_output: String::from("Deposits Stream!O1"),
        }
    }
}
