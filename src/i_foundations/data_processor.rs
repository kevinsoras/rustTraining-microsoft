// C# - Great productivity, runtime overhead
/* public class DataProcessor
{
    private List<int> data = new List<int>();
    
    public void ProcessLargeDataset()
    {
        // Allocations trigger GC
        for (int i = 0; i < 10_000_000; i++)
        {
            data.Add(i * 2); // GC pressure
        }
        // Unpredictable GC pauses during processing
    }
} */
// Runtime: Variable (50-200ms due to GC)
// Memory: ~80MB (including GC overhead)
// Predictability: Low (GC pauses)

/*
Rust - zero runtime overhead
*/

pub struct DataProcessor {
    pub data: Vec<i32>,
}

impl DataProcessor {
    pub fn process_large_dataset(&mut self) {
        for i in 0..10_000_000 {
            self.data.push(i * 2);
        }
    }
}
// Runtime: Variable (50-200ms due to GC)
// Memory: ~80MB (including GC overhead)
// Predictability: Low (GC pauses)