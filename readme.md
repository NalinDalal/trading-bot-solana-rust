# trading bot b/w jupiter and binance

Jupiter: Aggregates liquidity across Solana DEXs (like Raydium, Orca). Used for token swaps on-chain.

Binance: Centralized exchange with REST & WebSocket APIs for trading, price data, and order execution.

Goal: Automatically detect price differences (arbitrage or trading opportunities) and execute trades between Solana (on-chain via Jupiter) and Binance (off-chain).

[Price Monitor] --> [Decision Engine] --> [Execution Layer] --> [Logging]

.env file:

```
BINANCE_API_KEY=your_binance_key
BINANCE_SECRET_KEY=your_binance_secret
SOLANA_KEYPAIR_PATH=~/.config/solana/id.json
```
