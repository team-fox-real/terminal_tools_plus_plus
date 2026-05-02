# terminal_tools_plus_plus

A collection of lightweight utilities for terminal manipulation and CLI development in Rust. Developed by Fox Team.

## Features

- Colorized Output: Integration with bright colors and text styles.
- Progress Bars: Real-time visual progress using carriage return (\r).
- Typewriter Effect: Letter-by-letter text animation for immersive CLI experiences.
- Utility Helpers: Screen clearing, stylized banners, and status logging (OK, ERROR, INFO).
- Enhanced Input: User data capture with formatted prompts.

## Installation

Add this to your Cargo.toml file:

```toml
[dependencies]
terminal_tools_plus_plus = "0.1.2"
```

## Usage Example

The following example demonstrates the basic functionality:

```rust
use terminal_tools_plus_plus::terminal;

fn main() {
    terminal::print_banner("CLI PROJECT");

    terminal::log_status("Starting services...", "info");

    terminal::typewriter_print("Connecting to Fox Team servers...", 50);

    for i in 0..=100 {
        terminal::show_progress(i, 100);
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    println!("\n");
    terminal::log_status("System ready", "ok");
}
```

## Development

To test the built-in functionalities, you can run the included demo:

```bash
cargo run --example demo
```

## License

This project is licensed under the MIT License. Feel free to use, modify, and contribute.

---
Developed by Fox Team Real
