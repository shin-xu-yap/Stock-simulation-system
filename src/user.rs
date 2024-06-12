use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct UserPreferences {
    pub username: String,
    pub initial_amount: f64,
    pub preferred_stocks: Vec<String>,
    pub order_amount: f64,
    pub sell_amount: f64,
    pub transaction_cost: f64,
    pub owned_stocks: HashMap<String, f64>
}

impl UserPreferences{
    pub fn initialize_user_preferences() -> Vec<UserPreferences> {
        let owned_stocks = HashMap::new();
        let user1_categories = vec!["technology", "energy", "banking"];
        let user2_categories = vec!["technology", "energy", "banking"];
        vec![
            UserPreferences {
                username: String::from("Ryan"),
                initial_amount: 20000.0,
                preferred_stocks: user1_categories.iter().flat_map(|&category| get_preferred_stocks(category)).collect(),
                order_amount: 10.0,
                sell_amount: 5.0,
                transaction_cost: 0.075,
                owned_stocks: owned_stocks.clone()
            },
            UserPreferences {
                username: String::from("Irene"),
                initial_amount: 25000.0,
                preferred_stocks: user2_categories.iter().flat_map(|&category| get_preferred_stocks(category)).collect(),
                order_amount: 20.0,
                sell_amount: 10.0,
                transaction_cost: 0.075,
                owned_stocks: owned_stocks.clone()
            },
        ]
    }
}

pub fn get_preferred_stocks(category: &str) -> Vec<String> {
    match category {
        "technology" => vec![
            String::from("CTOS"),
            String::from("D&O"),
            String::from("FRONTKEN"),
            String::from("INARI AMERTRON"),
            String::from("ITMAX"),
            String::from("MI"),
            String::from("MY E.G."),
            String::from("PENTAMASTER"),
            String::from("UNISEM(M)"),
            String::from("UCHI"),
            String::from("UWC"),
        ],
        "banking" => vec![
            String::from("AFFIN"),
            String::from("ALLIANCE"),
            String::from("AXIS"),
            String::from("BANK ISLAM"),
            String::from("BURSA"),
            String::from("MALAYSIAN PACIFIC"),
            String::from("OSK"),
        ],
        "consumer goods" => vec![
            String::from("BAT"),
            String::from("DXN"),
            String::from("FARM FRESH"),
            String::from("FRASER & NEAVE"),
            String::from("GUAN CHONG"),
            String::from("HEINEIKEN"),
            String::from("LOTTE"),
            String::from("SCIENTEX"),
        ],
        "healthcare" => vec![
            String::from("HARTALEGA"),
            String::from("KOSSAN"),
            String::from("KPJ"),
            String::from("SUPERMAX"),
            String::from("TOP GLOVE"),
        ],
        "real estate" => vec![
            String::from("ECO WORLD"),
            String::from("MAH SING"),
            String::from("MALAYSIA BUILDING"),
            String::from("MATRIX CONCEPTS"),
            String::from("SP SETIA"),
            String::from("SUNWAY"),
            String::from("UEM SURISE"),
            String::from("WCE HOLDINGS"),
        ],
        "energy" => vec![
            String::from("BUMI"),
            String::from("DAYANG"),
            String::from("DIALOG"),
            String::from("HIBISCUS"),
            String::from("GAS"),
            String::from("VELESTO ENERGY"),
            String::from("YINSON HOLDINGS"),
        ],
        "automotive" => vec![
            String::from("BERMAZ"),
            String::from("DRB-HICOM"),
            String::from("UMW HOLDINGS"),
        ],
        "telecommunication" => vec![
            String::from("ASTRO"),
            String::from("AXIS"),
            String::from("TIME DOTCOM"),
        ],
        "construction" => vec![
            String::from("GAMUDA"),
            String::from("MALAYAN CEMENT"),
        ],
        "industrial goods" => vec![
            String::from("CHIN HIN"),
            String::from("FRONTKEN"),
            String::from("V.S. INDUSTRY"),
        ],
        "financial services" => vec![
            String::from("AFFIN"),
            String::from("ALLIANCE"),
            String::from("AXIS"),
            String::from("BURSA"),
            String::from("MALAYSIAN PACIFIC"),
            String::from("OSK"),
            String::from("TA ANN"),
        ],
        "retail" => vec![
            String::from("AEON"),
            String::from("BAT"),
            String::from("PADINI"),
        ],
        "plantation" => vec![
            String::from("SARAWAK OIL PALMS"),
            String::from("TSH RESOURCES"),
            String::from("UNITED PLANTATIONS"),
        ],
        _ => vec![]
    }
}