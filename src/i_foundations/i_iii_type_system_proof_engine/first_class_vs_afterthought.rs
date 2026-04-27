// C# — FP bolted on; LINQ is expressive but the language fights you
/* public IEnumerable<Order> GetHighValueOrders(IEnumerable<Order> orders)
{
    return orders
        .Where(o => o.Total > 1000)   // Func<Order, bool> — heap-allocated delegate
        .Select(o => new OrderSummary  // Anonymous type or extra class
        {
            Id = o.Id,
            Total = o.Total
        })
        .OrderByDescending(o => o.Total);
    // No exhaustive matching on results
    // Null can sneak in anywhere in the pipeline
    // Can't enforce purity — any lambda might have side effects
} */

// Rust — FP is first-class; the language supports it with zero boilerplate
use itertools::Itertools;

pub struct Order {
    pub id: u32,
    pub total: i64,
}
#[derive(Debug)]
pub struct OrderSummary {
    total: i64, // Cents to avoid floating-point issues
}
pub fn get_high_value_orders(orders: &[Order]) -> Vec<OrderSummary> {
    orders
        .iter()
        .filter(|order| order.id > 0 && order.total > 1000) // No heap allocation for closures; zero-cost iterators
        .map(|order| OrderSummary { total: order.total }) // Type check structured results
        .sorted_by(|a, b| a.total.cmp(&b.total)) // Itertools provides sorted_by for sorting iterators
        .collect()
}
