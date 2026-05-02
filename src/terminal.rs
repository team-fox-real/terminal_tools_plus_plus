use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

/// Estados posibles para logs
pub enum Status {
    Ok,
    Error,
    Info,
}

/// Limpia la pantalla de la terminal (ANSI estándar)
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

/// Imprime un mensaje con estado coloreado
pub fn log_status(message: &str, status: Status) {
    let status_colored = match status {
        Status::Ok => "SUCCESS".green().bold(),
        Status::Error => "ERROR".red().bold(),
        Status::Info => "INFO".blue().bold(),
    };

    println!("[{}] {}", status_colored, message);
}

/// Muestra una barra de progreso simple usando retorno de carro
pub fn show_progress(current: u32, total: u32) {
    if total == 0 {
        print!("\rProgreso: [0.0%]");
        let _ = io::stdout().flush();
        return;
    }

    let percentage = (current as f32 / total as f32) * 100.0;
    print!("\rProgreso: [{:.1}%] ", percentage);
    let _ = io::stdout().flush();
}

/// Efecto de escritura tipo máquina
pub fn typewriter_print(text: &str, speed_ms: u64) {
    for c in text.chars() {
        print!("{}", format!("{}", c).cyan());
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(speed_ms));
    }
    println!();
}

/// Muestra un banner centrado
pub fn print_banner(title: &str) {
    println!("{}", "=".repeat(40).yellow());
    println!("{:^40}", title.bright_magenta().bold());
    println!("{}", "=".repeat(40).yellow());
}

/// Lee entrada del usuario con prompt coloreado
pub fn get_input(prompt: &str) -> String {
    print!("{} ", prompt.bright_white().italic());
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer entrada");

    input.trim().to_string()
}

/// Spinner simple (animación de carga)
pub fn show_spinner(message: &str, duration_secs: u64) {
    let symbols = ["|", "/", "-", "\\"];
    let start = Instant::now();
    let mut i = 0;

    while start.elapsed().as_secs() < duration_secs {
        print!("\r{} {}", symbols[i % 4].bright_cyan(), message);
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(100));
        i += 1;
    }

    // Limpia la línea al terminar
    println!("\r{} {}     ", "✔".green(), message);
}
