// C# — The fragile base class problem
/* public class Animal
{
    public virtual string Speak() => "...";
    public void Greet() => Console.WriteLine($"I say: {Speak()}");
}

public class Dog : Animal
{
    public override string Speak() => "Woof!";
}

public class RobotDog : Dog
{
    // Which Speak() does Greet() call? What if Dog changes?
    // Diamond problem with interfaces + default methods
    // Tight coupling: changing Animal can break RobotDog silently
} */

// Common C# anti-patterns:
// - God base classes with 20 virtual methods
// - Deep hierarchies (5+ levels) nobody can reason about
// - "protected" fields creating hidden coupling
// - Base class changes silently altering derived behavior

// Rust — Composition over inheritance, enforced by the language
pub trait Speak {
    fn speak(&self) -> &str;
}
// Greet is "Saludar" in Spanish, just to show we can have multiple traits
pub trait Greet: Speak {
    fn greet(&self) {
        println!("I say : {}", self.speak())
    }
}

pub struct Dog;
impl Speak for Dog {
    fn speak(&self) -> &str {
        "Woof!"
    }
}
impl Greet for Dog {} // Uses default greet()

pub struct RobotDog {
    pub voice: String, // Composition: owns its own data
}
impl Speak for RobotDog {
    fn speak(&self) -> &str {
        &self.voice
    }
}
impl Greet for RobotDog {
    fn greet(&self) {
        println!("I say robot Dog: {}", self.speak())
    }
    // Clear, explicit behavior
}

// No fragile base class problem — no base classes at all
// No hidden coupling — traits are explicit contracts
// No diamond problem — trait coherence rules prevent ambiguity
// Adding a method to Speaker? Compiler tells you everywhere to implement it.

/* Key insight: In C#, correctness is a discipline — you hope developers follow conventions, write tests, and catch edge cases in code review. In Rust, correctness is a property of the type system — entire categories of bugs (null derefs, forgotten variants, accidental mutation, data races) are structurally impossible. */