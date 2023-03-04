# final-fn
This crate provides **final_fn** macro, what executes given code when leaving code block.

# Examples
```rust
use final_fn::final_fn;

fn main() {
    final_fn!(
        println!("End of main!")
    );
    
    println!("Hello world!");
}
```

You can also pass variables
```rust
use final_fn::final_fn;

fn main() {
    let x = 56;
    
    final_fn(
        println!("{x}")
    );
    
    println!("Hello world");
}
```