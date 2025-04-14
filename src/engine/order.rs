
// export interface Order {
//     price: number;      // The price at which the user wants to trade
//     quantity: number;   // The total amount of the base asset to trade
//     orderId: string;    // A unique identifier for this order
//     filled: number;     // The amount of the quantity that has already been traded/filled
//     side: "buy" | "sell"; // Indicates if it's a buy or sell order
//     userId: string;     // Identifier for the user who placed the order
// }

enum Side {
    BIDS,
    ASKs

}

struct Order{
    price:f64, // price at which user wanted to trade
    quanity: f64, // total amount of base asset
    orderId:String, // a unique identifier for this order
    filled:f64, // the amount of order already filled 
    side:Side, // is it bids or asks
    userId:String // user who placed this order


}


// export interface Fill {
//     price: string;     
//     qty: number;        // The quantity traded in this specific fill
//     tradeId: number;    // A unique identifier for this specific trade execution
//     otherUserId: string;// The userId of the counterparty (maker order)
//     markerOrderId: string; // The orderId of the resting order (maker) that was matched against
// }


struct Fill {
    price:f64,  // The price at which the trade occurred (as string)
    qty:f64,  // The quantity traded in this specific fill
    tradeId:u32,  // A unique identifier for this specific trade execution
    otherUserId: String, // The userId of the counterparty (maker order)
    makerOrderID:String // The orderId of the resting order (maker) that was matched against
    
}
struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>
}