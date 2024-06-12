extern crate rand;
extern crate crossbeam_channel;
extern crate scheduled_thread_pool;
use rand::Rng;
use scheduled_thread_pool::ScheduledThreadPool;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use std::thread::{self, sleep};
use crossbeam_channel::unbounded;
use crate::calculation::{MovingAverage, RSI, STO};
use crate::order::{Order, OrderType, StockManager};
use crate::user::UserPreferences;
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, QueueDeclareOptions, Result};
use serde::{Serialize, Deserialize};
use serde_json;
use colored::*;
use bma_benchmark::Perf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock{
    pub name: String,
    pub value: f64,
}

#[derive(Debug)]
struct CustomError(String);

// displat custom error
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError{}

impl From<amiquip::Error> for CustomError {
    fn from(error: amiquip::Error) -> Self {
        CustomError(format!("AMQP Error: {}", error))
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(error: serde_json::Error) -> Self {
        CustomError(format!("JSON Error: {}", error))
    }
}

// stock market simulation
pub fn market_data(
    moving_averages: Arc<RwLock<HashMap<String, MovingAverage>>>,
    relative_strength_index: Arc<RwLock<HashMap<String, RSI>>>,
    stochastic_oscillator: Arc<RwLock<HashMap<String, STO>>>,
) {
    let mut perf = Perf::new();

    let (tx, rx) = unbounded();
    let (sel_s, sel_r) = unbounded();
    let sched = ScheduledThreadPool::new(5);
    let shared_stock = Arc::new(Mutex::new(
        vec![
            Stock { name: String::from("AEON"), value: 325.0 },
            Stock { name: String::from("AFFIN"), value: 455.0 },
            Stock { name: String::from("ALLIANCE"), value: 190.0 },
            Stock { name: String::from("ASTRO"), value: 123.0 },
            Stock { name: String::from("AXIS"), value: 342.0 },
            Stock { name: String::from("BANK ISLAM"), value: 110.0 },
            Stock { name: String::from("BERJAYA"), value: 287.0 },
            Stock { name: String::from("BERMAZ"), value: 429.0 },
            Stock { name: String::from("BOUSTEAD"), value: 319.0 },
            Stock { name: String::from("BAT"), value: 311.0 },
            Stock { name: String::from("BUMI"), value: 320.0 },
            Stock { name: String::from("BURSA"), value: 208.0 },
            Stock { name: String::from("CHIN HIN"), value: 482.0 },
            Stock { name: String::from("CTOS"), value: 162.0 },
            Stock { name: String::from("D&O"), value: 147.0 },
            Stock { name: String::from("DAGANG"), value: 306.0 },
            Stock { name: String::from("DAYANG"), value: 84.0 },
            Stock { name: String::from("DIALOG"), value: 133.0 },
            Stock { name: String::from("DRB-HICOM"), value: 142.0 },
            Stock { name: String::from("DXN"), value: 341.0 },
            Stock { name: String::from("ECO WORLD"), value: 350.0 },
            Stock { name: String::from("FARM FRESH"), value: 347.0 },
            Stock { name: String::from("FRASER & NEAVE"), value: 320.0 },
            Stock { name: String::from("FRONTKEN"), value: 498.0 },
            Stock { name: String::from("GAMUDA"), value: 277.0 },
            Stock { name: String::from("GAS"), value: 317.0 },
            Stock { name: String::from("GREATECH"), value: 285.0 },
            Stock { name: String::from("GUAN CHONG"), value: 402.0 },
            Stock { name: String::from("HARTALEGA"), value: 352.0 },
            Stock { name: String::from("HEINEIKEN"), value: 470.0 },
            Stock { name: String::from("HEXTAR"), value: 162.0 },
            Stock { name: String::from("HIBISCUS"), value: 353.0 },
            Stock { name: String::from("INARI AMERTRON"), value: 383.0 },
            Stock { name: String::from("ITMAX"), value: 345.0 },
            Stock { name: String::from("KOSSAN"), value: 215.0 },
            Stock { name: String::from("KPJ"), value: 116.0 },
            Stock { name: String::from("LOTTE"), value: 34.0 },
            Stock { name: String::from("MAH SING"), value: 481.0 },
            Stock { name: String::from("MALAYAN CEMENT"), value: 223.0 },
            Stock { name: String::from("MALAYSIA AIRPORTS"), value: 221.0 },
            Stock { name: String::from("MALAYSIA BUILDING"), value: 158.0 },
            Stock { name: String::from("MALAYSIAN PACIFIC"), value: 165.0 },
            Stock { name: String::from("MALAYSIAN RESOURCES"), value: 142.0 },
            Stock { name: String::from("MATRIX CONCEPTS"), value: 350.0 },
            Stock { name: String::from("MI"), value: 36.0 },
            Stock { name: String::from("MY E.G."), value: 381.0 },
            Stock { name: String::from("OSK"), value: 364.0 },
            Stock { name: String::from("PADINI"), value: 41.0 },
            Stock { name: String::from("PENTAMASTER"), value: 265.0 },
            Stock { name: String::from("SARAWAK OIL PALMS"), value: 72.0 },
            Stock { name: String::from("SCIENTEX"), value: 144.0 },
            Stock { name: String::from("SP SETIA"), value: 491.0 },
            Stock { name: String::from("SUNWAY"), value: 474.0 },
            Stock { name: String::from("SUPERMAX"), value: 235.0 },
            Stock { name: String::from("SYARIKAT TAKAFUL"), value: 483.0 },
            Stock { name: String::from("TA ANN"), value: 416.0 },
            Stock { name: String::from("TIME DOTCOM"), value: 253.0 },
            Stock { name: String::from("TOP GLOVE"), value: 20.0 },
            Stock { name: String::from("TSH RESOURCES"), value: 487.0 },
            Stock { name: String::from("UCHI"), value: 121.0 },
            Stock { name: String::from("UEM SURISE"), value: 332.0 },
            Stock { name: String::from("UMW HOLDINGS"), value: 15.0 },
            Stock { name: String::from("UNISEM(M)"), value: 162.0 },
            Stock { name: String::from("UNITED PLANTATIONS"), value: 147300.0 },
            Stock { name: String::from("UWC"), value: 63.0 },
            Stock { name: String::from("V.S. INDUSTRY"), value: 447.0 },
            Stock { name: String::from("VELESTO ENERGY"), value: 485.0 },
            Stock { name: String::from("WCE HOLDINGS"), value: 307.0 },
            Stock { name: String::from("WESTPORTS HOLDINGS"), value: 265.0 },
            Stock { name: String::from("YINSON HOLDINGS"), value: 41.0 },
        ])
    );
    let previous_values = Arc::new(Mutex::new(HashMap::new()));

    // Stock selector
    let stock_arc = shared_stock.clone();
    sched.execute_at_fixed_rate(Duration::from_micros(0), Duration::from_secs(1), move || {
        let mut rng = rand::thread_rng();
        let r_stock = rng.gen_range(0..69);
        let mut stocks = stock_arc.lock().unwrap();
        let stock = &mut stocks[r_stock];
        // Send to incrementor
        sel_s.send(stock.clone()).unwrap();
        perf.checkpoint("select stock");
    });

    // Incrementor
    for _i in 1..70 {
        let receiver = sel_r.clone();
        let tx1 = tx.clone();
        let moving_averages_clone = Arc::clone(&moving_averages);
        let relative_strength_index = Arc::clone(&relative_strength_index);
        let stochastic_oscillator = Arc::clone(&stochastic_oscillator);
        let previous_values = Arc::clone(&previous_values);

        thread::spawn(move || {
            loop {
                let mut stock: Stock = receiver.recv().unwrap();
                let mut rng = rand::thread_rng();
                // Geometric Brownian Motion (GBM) to simulate stock price changes
                let r: f64 = 0.05;
                let sigma: f64 = 0.2;
                let dt: f64 = 1.0 / 252.0;
                let epsilon: f64 = rng.gen();
                let d_s = r * stock.value * dt + sigma * stock.value * epsilon * (dt.sqrt());
                stock.value += d_s;
                // perf.checkpoint("increment stock value");

                let mut previous_values = previous_values.lock().unwrap();
                previous_values.insert(stock.name.clone(), stock.value);

                let mut moving_averages = moving_averages_clone.write().unwrap();
                let moving_average = moving_averages
                    .entry(stock.name.clone())
                    .or_insert_with(|| MovingAverage::new(5));
                moving_average.add(stock.value);

                let mut relative_strength_index = relative_strength_index.write().unwrap();
                let relative_strength_index = relative_strength_index
                    .entry(stock.name.clone())
                    .or_insert_with(|| RSI::new(14));
                relative_strength_index.update(stock.value);

                let mut stochastic_oscillator = stochastic_oscillator.write().unwrap();
                let stochastic_oscillator = stochastic_oscillator
                    .entry(stock.name.clone())
                    .or_insert_with(|| STO::new(14));
                stochastic_oscillator.update(stock.value);

                // Send to broadcaster
                tx1.send(stock.clone()).unwrap();
            }
        });
    }

    // Sender / broadcaster thread
    thread::spawn(move || {
        loop {
            let stock = rx.recv().unwrap();
            send_stock(&stock);
        }
    });

    // Receiver
    loop {
        display_stock();
    }
}

// broker 1 (1st priority: moving average | 2nd priority: RSI)
pub fn broker1(moving_averages: Arc<RwLock<HashMap<String, MovingAverage>>>, relative_strength_index: Arc<RwLock<HashMap<String, RSI>>>, manager:Arc<Mutex<StockManager>>,  mut user_preferences: Vec<UserPreferences>){
    thread::spawn(move ||{ 
        let manager = Arc::clone(&manager);
        loop {
            let stock = receive_stock().unwrap();
            if let Some(moving_averages) = moving_averages.read().unwrap().get(&stock.name) {
                let moving_average = moving_averages.average();
                let rs_index = relative_strength_index.read().unwrap().get(&stock.name).unwrap().rsi();

                if stock.value > moving_average {
                    // println!("**BUY SIGNAL** Current moving average for {}: {}", stock.name, moving_average);
                    for user_pref in &mut user_preferences {
                        if user_pref.username == "Ryan" && user_pref.preferred_stocks.contains(&stock.name){
                            buy_stock("Data/orders.txt", &stock, &manager, user_pref, user_pref.order_amount);
                        }
                    } 
                }
                else if stock.value < moving_average{
                    // println!("**SELL SIGNAL** Current moving average for {}: {}", stock.name, moving_average);
                    for user_pref in &mut user_preferences {
                        if user_pref.username == "Ryan" {
                            sell_stock("Data/orders.txt", &stock, &manager, user_pref, user_pref.sell_amount);
                        }
                    }
                }
                else if let Some(rsi) = rs_index{
                    if rsi < 30.0{
                        // println!("**STRONG BUY SIGNAL** The current RSI for {} is {}, indicating oversold conditions!", stock.name, rsi);
                        for user_pref in &mut user_preferences {
                            if user_pref.username == "Ryan" && user_pref.preferred_stocks.contains(&stock.name) {
                                buy_stock("Data/orders.txt", &stock, &manager, user_pref, user_pref.order_amount);
                            }
                        }
                    }else if rsi > 70.0{
                        // println!("**Strong SELL SIGNAL** The current RSI for {} is {}, indicating overbuy conditions!", stock.name, rsi);
                        for user_pref in &mut user_preferences {
                            if user_pref.username == "Ryan" {
                                sell_stock("Data/orders.txt", &stock, &manager, user_pref, user_pref.sell_amount);
                            }
                        }
                    }
                }
            } 
        }
    }); 
}

// broker 2
pub fn broker2(moving_averages: Arc<RwLock<HashMap<String, MovingAverage>>>, stochastic_oscillator: Arc<RwLock<HashMap<String, STO>>>, manager: Arc<Mutex<StockManager>>, mut user_preferences: Vec<UserPreferences>) {
    thread::spawn(move ||{ 
        let manager = Arc::clone(&manager);
        loop {
            let stock = receive_stock().unwrap();
            if let Some(moving_averages) = moving_averages.read().unwrap().get(&stock.name) {
                let moving_average = moving_averages.average();
            
                if let Some(stochastic_oscillator) = stochastic_oscillator.read().unwrap().get(&stock.name){
                    if let Some(sto) = stochastic_oscillator.sto(){
                        if moving_average > stock.value && sto > 80.0 {
                            // println!("**STRONG BUY SIGNAL** Moving average: {}, STO: {} for stock {}", moving_average, sto, stock.name);
                            for user_pref in &mut user_preferences{
                                if user_pref.username == "Irene" && user_pref.preferred_stocks.contains(&stock.name){
                                    buy_stock("Data/orders.txt", &stock, &manager, user_pref, user_pref.order_amount); 
                                }
                            } 
                        }else if moving_average < stock.value && sto < 20.0{
                            // println!("**Broker 2: STRONG SELL SIGNAL** Moving average: {}, STO: {} for stock {}", moving_average, sto, stock.name);
                            for user_pref in &mut user_preferences{
                                if user_pref.username == "Irene" {
                                    sell_stock("Data/orders.txt", &stock, &manager, user_pref, user_pref.sell_amount);
                                }     
                            } 
                        }else{
                            // println!("**Broker 2: HOLD SIGNAL** Current moving average for {}: {}", stock.name, moving_average);
                        }
                    }else{
                        if moving_average > stock.value{
                            // println!("**STRONG BUY SIGNAL** Moving average: {} for stock {}", moving_average, stock.name);
                            for user_pref in &mut user_preferences{
                                if user_pref.username == "Irene" && user_pref.preferred_stocks.contains(&stock.name){
                                    buy_stock("Data/orders.txt", &stock, &manager, user_pref, user_pref.order_amount); 
                                }
                            } 
                        }else if moving_average < stock.value{
                            // println!("**Broker 2: STRONG SELL SIGNAL** Moving average: {} for stock {}", moving_average, stock.name);
                            for user_pref in &mut user_preferences{
                                if user_pref.username == "Irene" {
                                    sell_stock("Data/orders.txt", &stock, &manager, user_pref, user_pref.sell_amount);
                                }     
                            } 
                        }
                    }   
                }
            }
        }
    });
}

// send stock
pub fn send_stock(stock: &Stock) -> Result<()>{
    let stock_string = serde_json::to_string(&stock).unwrap();

    let _sender = thread::spawn(move || -> Result<()> {
        let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
        let channel = connection.open_channel(None)?;
        let exchange = Exchange::direct(&channel);
        exchange.publish(Publish::new(stock_string.as_bytes(), "stocks"))?;
        connection.close()
    });
    Ok(())
}

// receive stock
fn receive_stock() -> Result<Stock, CustomError> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let queue = channel.queue_declare("stocks", QueueDeclareOptions::default())?;
    let consumer = queue.consume(ConsumerOptions::default())?;
    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body_str = std::str::from_utf8(&delivery.body).unwrap();
                let received_stock: Stock = serde_json::from_str(body_str)?;
                return Ok(received_stock); // Return the received stock
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
    Err(CustomError("No message received".to_string()))
}

// display stock
fn display_stock() -> Result<()> {
    let receiver = thread::spawn(move || -> Result<()> {
        let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
        let channel = connection.open_channel(None)?;
        let queue = channel.queue_declare("stocks", QueueDeclareOptions::default())?;
        let consumer = queue.consume(ConsumerOptions::default())?;
        let mut previous_values: HashMap<String, f64> = HashMap::new();

        println!("{:<20} | {:<15} | {:<10} | {:<10}", "Stock Name", "Current Value", "Change", "% Change");
        let mut day = 1;
        let mut second_counter = 0;
        for (_i, message) in consumer.receiver().iter().enumerate() {
            if second_counter % 15 == 0 {
                println!("Day {}", day);
                day += 1;
            }
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body_str = std::str::from_utf8(&delivery.body).unwrap();
                    let received_stock: Stock = serde_json::from_str(body_str).unwrap();
                    // write stock data to file
                    let _ = write_stock_to_file(&received_stock);
                    let formatted_value = format!("{:.2}", received_stock.value);
                    let change = match previous_values.get(&received_stock.name) {
                        Some(prev_value) => received_stock.value - prev_value,
                        None => 0.0,
                    };

                    let percent_change = if change != 0.0 {
                        (change / received_stock.value) * 100.0
                    } else {
                        0.0
                    };
                    
                    if percent_change >= 0.0 {
                        let formatted_change = format!("{:.2}", change);
                        let formatted_percent_change = format!("{:.2}", percent_change);
                        println!("{:<20} | {:<15} | {:<10} | {:<10}%", 
                            received_stock.name.black(), 
                            formatted_value.black(), 
                            formatted_change.green(), 
                            formatted_percent_change.green()
                        ); 
                    } else {
                        let formatted_change = format!("{:.2}", change);
                        let formatted_percent_change = format!("{:.2}", percent_change);
                        println!("{:<20} | {:<15} | {:<10} | {:<10}%", 
                            received_stock.name.black(), 
                            formatted_value.black(), 
                            formatted_change.red(), 
                            formatted_percent_change.red()
                        ); 
                    }
                    previous_values.insert(received_stock.name, received_stock.value);
                    std::thread::sleep(Duration::from_secs(1));  
                },
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
            second_counter += 1;
        }
        connection.close()?;
        Ok(())
    });
    receiver.join().unwrap()
}

// write stock data into text file
fn write_stock_to_file(stock: &Stock) -> std::io::Result<()> {
    let path = "Data/stocks.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&path)?;

    // Check if file is empty
    if file.metadata()?.len() == 0 {
        // If it is, write the header
        writeln!(file, "{:<20} | {:<15}", "Stock Name", "Current Value")?;
    }

    // Then append the new data
    let formatted_value = format!("{:.2}", stock.value);
    let data = format!("{:<20} | {:<15}", stock.name, formatted_value);
    writeln!(file, "{}", data)?;

    Ok(())
}

// write orders into text file
pub(crate) fn write_order_to_file(filename: &str, order: &Order, username: &str) -> std::io::Result<()> {
    let path = filename;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&path)?;

    // Check if file is empty
    if file.metadata()?.len() == 0 {
        // If it is, write the header
        writeln!(file, "{:<10} | {:<25} | {:<10} | {:<10} | {:<45} | {:<20}" , "User", "Stock Name", "Quantity", "Price", "Timestamp", "Order Type")?;
    }

    // Then append the new data
    let formatted_value = format!("{:.2}", order.stock.value);
    let data = format!("{:<10} | {:<25} | {:<10} | {:<10} | {:<45} | {:<20}", 
        username, 
        order.stock.name, 
        format!("{:<10.2}", order.quantity), 
        formatted_value, 
        format!("{:<45}", order.timestamp.to_rfc3339()), 
        format!("{:<20}", order.order_type));
    writeln!(file, "{}", data)?;

    Ok(())
}

// buying stocks
fn buy_stock(filename: &str, stock: &Stock, manager: &Arc<Mutex<StockManager>>, user_pref: &mut UserPreferences, order_amount: f64) {
    let order = Order::new(stock, OrderType::Buy, order_amount);
    let mut manager = manager.lock().unwrap();
    let total_value = stock.value * order_amount;
    if user_pref.initial_amount >= total_value {
        user_pref.initial_amount -= total_value;
        manager.add_order(order); 
        let cur_index = manager.orders.len() - 1; 
        manager.complete_order(cur_index); // manager completes the order
        user_pref.owned_stocks.entry(stock.name.to_string()).and_modify(|e| *e += order_amount).or_insert(order_amount);
        sleep(Duration::from_secs(5));
        let _ = write_order_to_file(filename, &manager.orders[cur_index], &user_pref.username); // write the current order to file
    } else {
        let cur_index = manager.orders.len() - 1; 
        manager.cancel_order(cur_index); // manager cancels the order
    }
}

// selling stocks
fn sell_stock(filename: &str, stock: &Stock, manager: &Arc<Mutex<StockManager>>, user_pref: &mut UserPreferences, order_amount: f64) {
    if let Some(quantity) = user_pref.owned_stocks.get(&stock.name) {
        if *quantity >= order_amount {
            let order = Order::new(stock, OrderType::Sell, order_amount);
            let mut manager = manager.lock().unwrap();
            manager.add_order(order);
            let total_value = stock.value * order_amount;
            user_pref.initial_amount += total_value;
            if let Some(quantity) = user_pref.owned_stocks.get_mut(&stock.name) {
                *quantity -= order_amount;
            }            
            let cur_index = manager.orders.len() - 1;
            manager.complete_order(cur_index);
            sleep(Duration::from_secs(5));
            let _ = write_order_to_file(filename, &manager.orders[cur_index], &user_pref.username);
        } 
    } 
}