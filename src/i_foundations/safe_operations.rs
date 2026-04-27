
// C# - Runtime safety with overhead
/* public class RuntimeCheckedOperations
{
    public string? ProcessArray(int[] array)
    {
        // Runtime bounds checking on every access
        if (array.Length > 0)
        {
            return array[0].ToString(); // Safe — int is a value type, never null
        }
        return null; // Nullable return (string? with C# 8+ nullable reference types)
    }
    
    public void ProcessConcurrently()
    {
        var list = new List<int>();
        
        // Data races possible, requires careful locking
        Parallel.For(0, 1000, i =>
        {
            lock (list) // Runtime overhead
            {
                list.Add(i);
            }
        });
    }
} */

// Rust - Compile-time safety with zero overhead
pub struct SafeOperations;

impl SafeOperations {
    pub fn process_array(arr: &[i32]) -> Option<String> {
        arr.first().map(|x| x.to_string())
    }
    pub fn process_concurrently() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let data = Arc::new(Mutex::new(Vec::<i32>::new()));
        let handles: Vec<_> = (0..1000)
            .map(|i| {
                let data_clone = Arc::clone(&data);
                thread::spawn(move || {
                    // This is a easy and unsafe way to demostrate concurrent data proccesing
                    //data_clone.lock().unwrap().push(i);
                    // This is a safe way to demostrate concurrent data proccesing
                    match data_clone.lock() {
                        Ok(mut data) => {
                            data.push(i);
                        }
                        Err(e) => eprint!("Failed to acquire lock {}", e),
                    };
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
