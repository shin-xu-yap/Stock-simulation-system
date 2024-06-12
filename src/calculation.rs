use std::collections::VecDeque;

pub struct MovingAverage {
    pub period: usize,
    pub prices: VecDeque<f64>,
    pub sum: f64,
}

impl MovingAverage {
    pub fn new(period: usize) -> Self {
        MovingAverage {
            period,
            prices: VecDeque::with_capacity(period),
            sum: 0.0,
        }
    }

    pub fn add(&mut self, price: f64) {
        if self.prices.len() == self.period {
            // pop the oldest value
            self.sum -= self.prices.pop_front().unwrap();
        }
        // insert new value
        self.prices.push_back(price);
        self.sum += price;
    }

    pub fn average(&self) -> f64 {
        self.sum / self.prices.len() as f64
    }
}    

pub struct RSI {
    pub avg_gain: f64,
    pub avg_loss: f64,
    pub prices: Vec<f64>,
    pub period: usize
}

impl RSI {
    pub fn new(period: usize) -> Self {
        RSI {
            avg_gain: 0.0,
            avg_loss: 0.0,
            prices: Vec::with_capacity(14),
            period
        }
    }

    pub fn update(&mut self, price: f64) {
        if self.prices.len() == 15 {
            self.prices.remove(0);
        }
        self.prices.push(price);
        if self.prices.len() < 15 {
            return;
        }

        let mut gain = 0.00;
        let mut loss = 0.00;
        for i in 1..15 {
            let change = self.prices[i] - self.prices[i - 1];
            if change >= 0.0 {
                gain += change;
            } else {
                loss -= change; 
            }
        }
        self.avg_gain = gain/14.0;
        self.avg_loss = loss/14.0;
    }

    pub fn rsi(&self) -> Option<f64> {
        if self.prices.len() < 15 {
            return None;
        }
        let rs = self.avg_gain / self.avg_loss;
        Some(100.0 - (100.0 / (1.0 + rs)))
    }
}

#[allow(dead_code)]
pub struct STO{
    period: usize,
    lowest: f64,
    highest: f64,
    prices: Vec<f64>
}

impl STO{
    pub fn new(period: usize) -> Self {
        STO {
            period,
            lowest: 0.0,
            highest: 0.0,
            prices: Vec::with_capacity(14)
        }
    }

    pub fn update(&mut self, price: f64){
        if self.prices.len() == 5 {
            self.prices.remove(0);
        }
        self.prices.push(price);
        if self.prices.len() < 5 {
            return;
        }

        let mut max = self.prices[0];
        for i in 1..5 {
            if self.prices[i] > max{
                max = self.prices[i];
            }
        }

        let mut min = self.prices[0];
        for i in 1..5 {
            if self.prices[i] < min{
                min = self.prices[i];
            }
        }
        self.highest = max;
        self.lowest = min;
    }

    pub fn sto(&self)-> Option<f64> {
        if self.prices.len() < 5 {
            return None;
        }
        let sto = (self.prices[3] - self.lowest)/(self.highest-self.prices[3])*100.0;
        Some(sto)
    }
}