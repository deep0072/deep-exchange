// struct Order {
//     price:f32,
//     qty:f32,

// }

// struct OrderBook{
//     asks: Vec<Order>,
//     buy: Vec<Order>,
// }
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum Side {
    ASKS,
    BIDS,
}

struct Order {
    price: f32,
    quantity: f32,
    user_id: String,
    filled: f32,
    side: Side,
    order_id: u32,
}

struct Fills {
    price: f32,
    qty: f32,
    other_user_id: String,
    trade_id: u32,
    timestamp: u64,
    order_id: u32,
}

struct OrderBook {
    asks: Vec<Order>,
    bids: Vec<Order>,
    base_asset: String,
    quote_asset: String,
    last_trade_id: u32,
    current_price: f32,
}

impl OrderBook {
    fn new_order(
        bids: Vec<Order>,
        asks: Vec<Order>,
        last_trade_id: u32,
        current_price: f32,
    ) -> Self {
        Self {
            asks: Vec::new(),
            bids: Vec::new(),
            last_trade_id: 0,
            current_price: 0.0,
            base_asset: String::new(),
            quote_asset: String::new(),
        }
    }

    fn add_order(&mut self, order: &mut Order) {
        if order.side == Side::BIDS {
            let (fills, executed_qty) = self.match_bid(order);
        }
        if order.side == Side::ASKS {
            self.match_ask(order);
        }
    }

    // match bid
    fn match_bid(&mut self, order: &mut Order) -> (Vec<Fills>, f32) {
        let mut fills: Vec<Fills> = Vec::new();
        let mut executed_quantity = 0.0;
        for ask in self.asks.iter_mut() {
            if ask.price <= order.price && executed_quantity <= order.quantity {
                let mut filled_qty = (order.quantity - executed_quantity).min(ask.quantity);
                executed_quantity += filled_qty;
                ask.filled += filled_qty;
                self.last_trade_id += 1;

                fills.push(Fills {
                    price: ask.price,
                    qty: filled_qty,
                    trade_id: self.last_trade_id,
                    timestamp: 0,
                    other_user_id: ask.user_id.clone(),
                    order_id: ask.order_id,
                })
            }
        }

        // filter out filled asks id that present in orderbook
        let asks_to_remove: Vec<u32> = self
            .asks
            .iter()
            .filter(|ask| ask.filled == ask.quantity)
            .map(|ask| ask.order_id)
            .collect();

        // then remove those filled asks by removing from original array
        self.asks
            .retain(|ask| !asks_to_remove.contains(&ask.order_id));
        return (fills, executed_quantity);
    }

    // match asks

    fn match_ask(&mut self, order: &mut Order) -> (Vec<Fills>, f32) {
        let mut executed_qty = 0.0;
        let mut fills: Vec<Fills> = Vec::new();

        for bid in self.bids.iter_mut() {
            if bid.price >= order.price && executed_qty <= order.quantity {
                let mut filled_qty = (order.quantity - executed_qty).min(bid.quantity);
                bid.filled += filled_qty;
                executed_qty += filled_qty;
                self.last_trade_id += 1;

                fills.push(Fills {
                    price: bid.price,
                    qty: filled_qty,
                    trade_id: self.last_trade_id,
                    timestamp: 0, // Add a proper timestamp implementation
                    other_user_id: bid.user_id.clone(),
                    order_id: bid.order_id,
                })
            }
        }
        // while iterating, collect order_ids to remove

        let bids_to_remove: Vec<u32> = self
            .bids
            .iter()
            .filter(|bid| bid.filled == bid.quantity)
            .map(|bid| bid.order_id)
            .collect();

        // then ovveride all bids which do not contain bid that already filled
        self.bids
            .retain(|bid| !bids_to_remove.contains(&bid.order_id));

        return (fills, executed_qty);
    }

    // fn get_depth(&mut self) {
    //     // this function used to get
    //     // total quantity is available at each specific price level for both bids and asks
    //     let mut bids:Vec<>
    // }
    //
    fn get_depth(&mut self) {
        // let say there 5 order that sits in orderbook
        // each 2 order at 2000 $ and 3 order at $2100
        // so this gives output return ==>[(2000, 2),(2100,3)]
        let mut bids: Vec<(String, String)> = Vec::new();
        let mut asks: Vec<(String, String)> = Vec::new();
        // get aggregated total quanity of each bids at specific prices

        let mut bids_object: HashMap<String, f32> = HashMap::new();
        let mut asks_object: HashMap<String, f32> = HashMap::new();

        for bid in self.bids.iter() {
            *bids_object.entry((bid.price).to_string()).or_insert(0.0) += bid.quantity;
        }

        // get aggregated total quanity of each asks at specific prices
        for ask in self.asks.iter() {
            *asks_object.entry(ask.price.to_string()).or_insert(0.0) += ask.quantity;
        }

        for (price, value) in bids_object.iter() {
            bids.push((price.to_string(), value.to_string()));
        }

        for (price, value) in asks_object.iter() {
            asks.push((price.to_string(), value.to_string()));
        }
    }

    // get open order that sits on both side of asks and bids
    fn get_open_order(&mut self, user_id: String) -> (Vec<&Order>, Vec<&Order>) {
        let open_asks: Vec<&Order> = self
            .asks
            .iter()
            .filter(|ask| ask.user_id == user_id)
            .collect();

        let open_bids: Vec<&Order> = self
            .bids
            .iter()
            .filter(|bid| bid.user_id == user_id)
            .collect();
        (open_asks, open_bids)
    }

    // cancel bids and cancel asks
    fn cancel_bids(&mut self, order: Order) -> &Vec<Order> {
        self.bids.retain(|bid| bid.order_id != order.order_id);
        &self.bids
    }

    fn cancel_asks(&mut self, order: Order) -> &Vec<Order> {
        self.asks.retain(|ask| ask.order_id == order.order_id);
        &self.asks
    }
}
