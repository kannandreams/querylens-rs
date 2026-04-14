pub fn show_examples() {
    println!("QueryLens Examples:\n");
    println!("Basic usage:");
    println!("  querylens-rs lint queries/");
    println!("  querylens-rs report app.sql --format markdown");
    println!("  querylens-rs scan . --deep");
    println!("  querylens-rs score queries/ --min-score 75");
    println!("  querylens-rs diff old.sql new.sql --mode both\n");
    println!("Use `querylens-rs init` to create a default config file.");
}
