/* // C# - GC can pause at any time
public class HighFrequencyTrader
{
    private List<Trade> trades = new List<Trade>();

    public void ProcessMarketData(MarketTick tick)
    {
        // Allocations can trigger GC at worst possible moment
        var analysis = new MarketAnalysis(tick);
        trades.Add(new Trade(analysis.Signal, tick.Price));

        // GC might pause here during critical market moment
        // Pause duration: 1-100ms depending on heap size
    }
}
 */
// Rust - Predictable, deterministic performance
pub struct HighFrequencyTrader {
    pub trades: Vec<Trade>,
}
impl HighFrequencyTrader {
    pub fn process_market_data(&mut self, tick: MarketTick) {
        // Zero allocations, predictable performance
        let price = tick.price;
        let analysis = MarketAnalysis::from(tick);
        let trade = Trade {
            signal: analysis.signal,
            price,
        };
        println!("Processing market data: price={}, signal={}", trade.price, trade.signal);
        self.trades.push(trade);
         // No GC pauses, consistent sub-microsecond latency
        // Performance guaranteed by type system
    }
}
pub struct Trade {
    signal: String,
    price: u64,
}
pub struct MarketTick {
    pub price: u64,
    pub volume: u64,
}
struct MarketAnalysis {
    signal: String,
}
// Implementing From trait for MarketAnalysis to convert from MarketTick
impl From<MarketTick> for MarketAnalysis {
    fn from(tick: MarketTick) -> Self {
        let signal = if tick.price > 1000 { "Sell" } else { "Buy" }.to_string();
        let signal = if tick.volume > 500 {
            "Strong".to_string()
        } else {
            signal
        };
        MarketAnalysis { signal }
    }
}
