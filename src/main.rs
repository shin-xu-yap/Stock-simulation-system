// extern crate bma_benchmark;
// use std::{collections::HashMap, sync::{Arc, Mutex}, thread};
// use bma_benchmark::benchmark;
// use core::hint::black_box;
// use crate::{benchmarking::Stock, order::StockManager, user::UserPreferences};
// mod benchmarking;
// mod stock;
// mod user;
// mod calculation;
// mod order;
// use crate::stock::Stock as OtherStock;
// use crate::user::get_preferred_stocks;

// fn main() {
//     let shared_stock = Arc::new(Mutex::new(
//         vec![
//             Stock { name: String::from("AEON"), value: 325.0 },
//             Stock { name: String::from("AFFIN"), value: 455.0 },
//             Stock { name: String::from("ALLIANCE"), value: 190.0 },
//             Stock { name: String::from("ASTRO"), value: 123.0 },
//             Stock { name: String::from("AXIS"), value: 342.0 },
//             Stock { name: String::from("BANK ISLAM"), value: 110.0 },
//             Stock { name: String::from("BERJAYA"), value: 287.0 },
//             Stock { name: String::from("BERMAZ"), value: 429.0 },
//             Stock { name: String::from("BOUSTEAD"), value: 319.0 },
//             Stock { name: String::from("BAT"), value: 311.0 },
//             Stock { name: String::from("BUMI"), value: 320.0 },
//             Stock { name: String::from("BURSA"), value: 208.0 },
//             Stock { name: String::from("CHIN HIN"), value: 482.0 },
//             Stock { name: String::from("CTOS"), value: 162.0 },
//             Stock { name: String::from("D&O"), value: 147.0 },
//             Stock { name: String::from("DAGANG"), value: 306.0 },
//             Stock { name: String::from("DAYANG"), value: 84.0 },
//             Stock { name: String::from("DIALOG"), value: 133.0 },
//             Stock { name: String::from("DRB-HICOM"), value: 142.0 },
//             Stock { name: String::from("DXN"), value: 341.0 },
//             Stock { name: String::from("ECO WORLD"), value: 350.0 },
//             Stock { name: String::from("FARM FRESH"), value: 347.0 },
//             Stock { name: String::from("FRASER & NEAVE"), value: 320.0 },
//             Stock { name: String::from("FRONTKEN"), value: 498.0 },
//             Stock { name: String::from("GAMUDA"), value: 277.0 },
//             Stock { name: String::from("GAS"), value: 317.0 },
//             Stock { name: String::from("GREATECH"), value: 285.0 },
//             Stock { name: String::from("GUAN CHONG"), value: 402.0 },
//             Stock { name: String::from("HARTALEGA"), value: 352.0 },
//             Stock { name: String::from("HEINEIKEN"), value: 470.0 },
//             Stock { name: String::from("HEXTAR"), value: 162.0 },
//             Stock { name: String::from("HIBISCUS"), value: 353.0 },
//             Stock { name: String::from("INARI AMERTRON"), value: 383.0 },
//             Stock { name: String::from("ITMAX"), value: 345.0 },
//             Stock { name: String::from("KOSSAN"), value: 215.0 },
//             Stock { name: String::from("KPJ"), value: 116.0 },
//             Stock { name: String::from("LOTTE"), value: 34.0 },
//             Stock { name: String::from("MAH SING"), value: 481.0 },
//             Stock { name: String::from("MALAYAN CEMENT"), value: 223.0 },
//             Stock { name: String::from("MALAYSIA AIRPORTS"), value: 221.0 },
//             Stock { name: String::from("MALAYSIA BUILDING"), value: 158.0 },
//             Stock { name: String::from("MALAYSIAN PACIFIC"), value: 165.0 },
//             Stock { name: String::from("MALAYSIAN RESOURCES"), value: 142.0 },
//             Stock { name: String::from("MATRIX CONCEPTS"), value: 350.0 },
//             Stock { name: String::from("MI"), value: 36.0 },
//             Stock { name: String::from("MY E.G."), value: 381.0 },
//             Stock { name: String::from("OSK"), value: 364.0 },
//             Stock { name: String::from("PADINI"), value: 41.0 },
//             Stock { name: String::from("PENTAMASTER"), value: 265.0 },
//             Stock { name: String::from("SARAWAK OIL PALMS"), value: 72.0 },
//             Stock { name: String::from("SCIENTEX"), value: 144.0 },
//             Stock { name: String::from("SP SETIA"), value: 491.0 },
//             Stock { name: String::from("SUNWAY"), value: 474.0 },
//             Stock { name: String::from("SUPERMAX"), value: 235.0 },
//             Stock { name: String::from("SYARIKAT TAKAFUL"), value: 483.0 },
//             Stock { name: String::from("TA ANN"), value: 416.0 },
//             Stock { name: String::from("TIME DOTCOM"), value: 253.0 },
//             Stock { name: String::from("TOP GLOVE"), value: 20.0 },
//             Stock { name: String::from("TSH RESOURCES"), value: 487.0 },
//             Stock { name: String::from("UCHI"), value: 121.0 },
//             Stock { name: String::from("UEM SURISE"), value: 332.0 },
//             Stock { name: String::from("UMW HOLDINGS"), value: 15.0 },
//             Stock { name: String::from("UNISEM(M)"), value: 162.0 },
//             Stock { name: String::from("UNITED PLANTATIONS"), value: 147300.0 },
//             Stock { name: String::from("UWC"), value: 63.0 },
//             Stock { name: String::from("V.S. INDUSTRY"), value: 447.0 },
//             Stock { name: String::from("VELESTO ENERGY"), value: 485.0 },
//             Stock { name: String::from("WCE HOLDINGS"), value: 307.0 },
//             Stock { name: String::from("WESTPORTS HOLDINGS"), value: 265.0 },
//             Stock { name: String::from("YINSON HOLDINGS"), value: 41.0 },
//         ])
//     );

//     // benchmark!(10_000,{
//     //     benchmarking::benchmark_serialization(&shared_stock);
//     // });

//     // benchmark!(10_000,{
//     //     let shared_stock = Arc::clone(&shared_stock);
//     //     thread::spawn(move || {
//     //         loop{
//     //             for i in 1..70{
//     //                 let stocks = shared_stock.lock().unwrap();             
//     //                 let stock = &stocks[i];
//     //                 let _ = benchmarking::benchmark_sending_stocks(stock);
//     //             }
//     //         }
//     //     });
//     // });
    
//     // // spawn sender thread
//     // let shared_stock = Arc::clone(&shared_stock);
//     // thread::spawn(move || {
//     //     loop{
//     //         for i in 1..70{
//     //             let stocks = shared_stock.lock().unwrap();             
//     //             let stock = &stocks[i];
//     //             let _ = benchmarking::benchmark_sending_stocks(stock);
//     //         }
//     //     }
//     // });

//     // benchmark!(5000,{
//     //     benchmarking::benchmark_receiving_stocks();        
//     // });

//     // pub fn generate_test_data() -> (&'static str, OtherStock, Arc<Mutex<StockManager>>, UserPreferences, f64) {
//     //     // Define sample data
//     //     let filename = "example.txt";
//     //     let stock = OtherStock { name: "Example Stock".to_string(), value: 100.0 };
//     //     let manager = Arc::new(Mutex::new(StockManager::new()));
//     //     let user_categories = vec!["technology", "energy", "banking"];
//     //     let owned_stocks = HashMap::new();
//     //     let user_pref = UserPreferences { username: "example_user".to_string(), initial_amount: 10000.0, owned_stocks: owned_stocks.clone(), preferred_stocks:  user_categories.iter().flat_map(|&category| get_preferred_stocks(category)).collect(), order_amount: 10.0, sell_amount: 5.0, transaction_cost: 0.075 };
//     //     let order_amount = 10.0; // Example order amount
    
//     //     // Return the sample data
//     //     (filename, stock, manager, user_pref, order_amount)
//     // }    
    
//     // let (filename, stock, manager, mut user_pref, order_amount) = generate_test_data();

//     // benchmark!(10000, {
//     //     benchmarking::benchmark_buying_stocks(filename, &stock, &manager, &mut user_pref, order_amount);
//     // });
// }

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
    thread,
};
use calculation::{MovingAverage, RSI, STO};
use order::StockManager;
use stock::{broker1, broker2, market_data};
use user::UserPreferences;
use bma_benchmark::Perf;

mod stock;
mod user;
mod calculation;
mod order;

fn main() {
    let moving_averages: Arc<RwLock<HashMap<String, MovingAverage>>> = Arc::new(RwLock::new(HashMap::new()));
    let relative_strength_index: Arc<RwLock<HashMap<String, RSI>>> = Arc::new(RwLock::new(HashMap::new()));
    let stochastic_oscillator: Arc<RwLock<HashMap<String, STO>>> = Arc::new(RwLock::new(HashMap::new()));
    let manager = Arc::new(Mutex::new(StockManager::new()));
    let user_preferences = UserPreferences::initialize_user_preferences();

    let mut perf = Perf::new();
    perf.start();
    market_data(moving_averages.clone(), relative_strength_index.clone(), stochastic_oscillator.clone());
    perf.print();

    // let moving_averages_clone = Arc::clone(&moving_averages);
    // let moving_averages_clone1 = moving_averages.clone();
    // let relative_strength_index_clone= Arc::clone(&relative_strength_index);
    // let stochastic_oscillator_clone = stochastic_oscillator.clone();
    // let manager_clone = manager.clone();
    // let user_preferences_clone = user_preferences.clone();
    // let user_preferences_clone1 = user_preferences.clone();


    // // stock market data
    // thread::spawn(move || {
    //     market_data(moving_averages, relative_strength_index, stochastic_oscillator);
    // });

    // // broker 1
    // thread::spawn(move || {
    //     broker1(moving_averages_clone, relative_strength_index_clone, manager, user_preferences_clone);
    // });

    // // broker 2
    // thread::spawn(move || {
    //     broker2(moving_averages_clone1, stochastic_oscillator_clone, manager_clone, user_preferences_clone1);
    // });

    // loop {}
}