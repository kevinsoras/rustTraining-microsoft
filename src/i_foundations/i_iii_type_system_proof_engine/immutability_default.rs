// C# — Everything is mutable by default
/* public class Config
{
    public string Host { get; set; }   // Mutable by default
    public int Port { get; set; }
} */

// "readonly" and "record" help, but don't prevent deep mutation:
/* public record ServerConfig(string Host, int Port, List<string> AllowedOrigins);
 */
/* var config = new ServerConfig("localhost", 8080, new List<string> { "*.example.com" });
 */// Records are "immutable" but reference-type fields are NOT:
/* config.AllowedOrigins.Add("*.evil.com"); // Compiles and mutates! ← bug
 */// The compiler gives you no warning.

// Rust — Immutable by default, mutation is explicit and visible
struct Config {
    host: String,
    port: u16,
    allowed_origins: Vec<String>,
}

pub fn main() {
    println!("Running immutability default... ");
    
    let config = Config {
        host: "localhost".into(),
        port: 8080,
        allowed_origins: vec!["*.example.com".into()],
    };

    // config.allowed_origins.push("*.evil.com".into()); // ERROR: cannot borrow as mutable

    // Mutation requires explicit opt-in:
    let mut config: Config = config;
    config.allowed_origins.push("*.safe.com".into()); // OK — visibly mutable
    add_origin(&mut config, "*.kevin.com".into());
    println!(
        "Host:{} , Port: {} , Allowed Origins: {:?}",
        config.host, config.port, config.allowed_origins
    );
}

// "mut" in the signature tells every reader: "this function modifies data"
fn add_origin(config: &mut Config, origin: String) {
    config.allowed_origins.push(origin);
}
