// We need some more machinery to write a proper exercise for destructors.
// We'll pick the concept up again in a later chapter after covering traits and
// interior mutability.
fn outro() -> &'static str {
    "I have a basic understanding of destructors!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}

fn compute(t: String) {
    // Do something [...]
    drop(t); // <-- Assuming `t` wasn't dropped or moved
    //     before this point, the compiler will call
    //     `drop` here, when it goes out of scope
}

fn main() {
    let s = "Hello".to_string();
    compute(s);
}
