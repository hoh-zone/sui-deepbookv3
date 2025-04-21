# Sui DeepBook v3 SDK

A Rust SDK for interacting with DeepBook - a decentralized exchange (DEX) protocol on the Sui network.

## Features

- Account management for orders and balances
- DEX operations (place/cancel orders, check order status)
- Liquidity pool interactions
- Admin operations for DeepBook
- Level2 orderbook data queries
- Flash loan capabilities
- Balance management utilities

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sui-deepbookv3 = { git = "https://github.com/hoh-zone/sui-deepbookv3" }
```

## Quick Start

Here are some example use cases:

### Check Account Open Orders
```rust
use sui_deepbookv3::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new().await;
    let orders = client.get_account_open_orders().await;
    println!("Open orders: {:?}", orders);
}
```

### Query Level2 Order Book
```rust 
use sui_deepbookv3::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new().await;
    let level2_data = client.get_level2_book_status(0, 10).await;
    println!("Order book depth: {:?}", level2_data);
}
```

## Examples

Check the `examples/` directory for more detailed usage:

- `account_open_orders.rs` - Query account's open orders
- `account_order_map.rs` - Map orders to account
- `balance.rs` - Check account balances
- `get_level2_range.rs` - Get order book depth

## Testing

Run the test suite:

```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/awesome-feature`)
3. Commit your changes (`git commit -am 'Add awesome feature'`)
4. Push to the branch (`git push origin feature/awesome-feature`)
5. Open a Pull Request

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

