use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
struct CryptoInfo {
    symbol: String,
    price: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    response: Vec<CryptoInfo>
}

#[derive(Debug, Deserialize)]
struct BuyOrder {
    quant: f64,
    price: f64
}

#[derive(Debug, Deserialize)]
struct SellOrder {
    quant: f64,
    price: f64
}

struct OrderBook {
    buy_order: Vec<BuyOrder>,
    sell_order: Vec<SellOrder>
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            buy_order: Vec::new(),
            sell_order: Vec::new()
        }
    }

    fn add_buy_order(&mut self, order: BuyOrder) {
        self.buy_order.push(order);
        self.buy_order.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    }

    fn add_sell_order(&mut self, order: SellOrder) {
        self.sell_order.push(order);
        self.sell_order.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    }

    fn match_orders(&mut self) {
        while !self.buy_order.is_empty() && !self.sell_order.is_empty() {
            let best_buy = &self.buy_order[0];
            let best_sell = &self.sell_order[0];
            if best_buy.price >= best_sell.price {

                let buy_qty = best_buy.quant;
                let sell_qty = best_sell.quant;
                let matched_qty = if buy_qty < sell_qty { buy_qty } else { sell_qty };
                
                println!("- Trade: {} BTC and at {}", matched_qty, best_sell.price);
                
                if buy_qty > sell_qty {
                    self.buy_order[0].quant -= sell_qty;
                    self.sell_order.remove(0);
                } else if buy_qty < sell_qty {
                    self.sell_order[0].quant -= buy_qty;
                    self.buy_order.remove(0);
                } else {
                    self.buy_order.remove(0);
                    self.sell_order.remove(0);
                }
            } else {
                break;
            }
        }
    }
}

async fn fetchPriceApi() -> Result<CryptoInfo, Box<dyn Error>> {
    println!("Getting api...");
    let req = reqwest::get("https://api.binance.com/api/v3/ticker/price?").await?.json::<Vec<CryptoInfo>>().await?;
    // let data = req.f
    let data = req.iter().find(|info| info.symbol == "BTCUSDT");
    if let Some(res) = data {
        Ok(res.clone())
    } else {
        Err("BTCUSDT not found in Binance response".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let info = fetchPriceApi().await?;
    println!("Current BTC/USDT price: {}", info.price);
    println!("Symbol: {} \n", info.symbol);

    let mut order_book = OrderBook::new();

    order_book.add_buy_order(BuyOrder {
        quant: 1.5,
        price: 83570.0,
    });
    order_book.add_buy_order(BuyOrder {
        quant: 1.0,
        price: 83566.0,
    });
    
    order_book.add_sell_order(SellOrder {
        quant: 1.0,
        price: 83565.0,
    });
    order_book.add_sell_order(SellOrder {
        quant: 2.0,
        price: 83568.0,
    });

    println!("Buy orders: {:?}", order_book.buy_order);
    println!("Sell orders: {:?} \n", order_book.sell_order);

    println!("Matched Orders:");
    order_book.match_orders();

    
    println!("\nRemanining orders: ");
    println!("Buy orders: {:?}", order_book.buy_order);
    println!("Dell orders: {:?}", order_book.sell_order);

    Ok(())
}
