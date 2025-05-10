// struct Order {
//     price:f32,
//     qty:f32,

// }

// struct OrderBook{
//     asks: Vec<Order>,
//     buy: Vec<Order>,
// }

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
    order_id: String,
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
            last_trade_id: 0.0,
            current_price: 0.0,
        }
    }

    fn match_bid(&mut self, order: Order) -> (Vec<Fills>, f32) {
        let mut fills: Vec<Fills> = Vec::new();
        let mut executed_quanity = 0.0;
        for ask in self.asks.iter_mut() {
            if ask.price <= order.price && executed_quantity <= order.quantity {
                let mut filled_qty = order.quantity.min(ask.quantity);
                executed_qty += filled_qty;
                ask.filled += filled_qty;
                self.last_trade_id += 1;

                fills.push(Fills {
                    price: ask.price,
                    qty: filled_qty,
                    trade_id: self.last_trade_id,
                    othet_user_id: ask.user_id.clone(),
                    order_id: ask.order_id,
                })
            }
        }

        for ask in self.asks.iter_mut() {
            if ask.filled == ask.quantity {
                // pop that asks from vec
                self.asks.retain(|x| x.id != ask.id);
            }
        }

        return (fills, executed_quanity);
    }
    fn add_order(&mut self, order: Order) {
        if (order.side == Side::BIDS) {
            let (fills, executed_qty) = self.match_bid(order);
        } else {
            self.match_ask()
        }
    }
}
