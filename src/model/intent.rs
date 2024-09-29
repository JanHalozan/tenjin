pub struct Intent {
    action: fn(&str)
}

impl Default for Intent {
    fn default() -> Self {
        Self {
            action: |_| { println!("Implement action") }
        }
    }
}