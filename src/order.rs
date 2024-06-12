use chrono::prelude::*;
use crate::stock::Stock;
use std::fmt;

#[derive(Debug)]
pub struct Order{
    pub stock: Stock,
    pub quantity: f64,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub timestamp: DateTime<Utc>
}

#[derive(Debug)]
pub enum OrderType{
    Buy,
    Sell
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OrderType::Buy => write!(f, "Buy"),
            OrderType::Sell => write!(f, "Sell"),
        }
    }
}

#[derive(Debug)]
pub enum OrderStatus{
    Pending,
    Completed,
}

impl Order {
    pub fn new(stock: &Stock, order_type: OrderType, quantity: f64) -> Self {
        Order{
            stock: stock.clone(),
            order_type,
            quantity,
            timestamp: chrono::offset::Utc::now(),
            status: OrderStatus::Pending
        }
    }

    pub fn complete(&mut self) {
        self.status = OrderStatus::Completed;
    }
}

pub struct StockManager {
    pub orders: Vec<Order>,
}

impl StockManager {
    pub fn new() -> Self {
        StockManager { orders: Vec::new() }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn complete_order(&mut self, index: usize) {
        if let Some(order) = self.orders.get_mut(index) {
            order.complete();
        }
    }

    pub fn cancel_order(&mut self, index: usize){
        if index < self.orders.len() {
            self.orders.remove(index);
        }
    }
}