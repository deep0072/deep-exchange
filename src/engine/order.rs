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
    BIDs
}

struct Order {
    price:f32,
    quantity:f32,
    userId:String,
    filled:f32,
    side:Side
}

struct OrderBook{
    asks: Vec<Order>,
    bids:Vec<Order>,
    baseAsset:String,
    quoteAsset:String,
    lastTradedId:u32,
    currentPrice:f32
}