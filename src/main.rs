fn render_logo() {
    // UNEXPLORED
    println!("  _    _ _   _ ________   _______  _      ____  _____  ______ _____           ");
    println!(
        " | |  | | \\ | |  ____\\ \\ / /  __ \\| |    / __ \\|  __ \\|  ____|  __ \\          "
    );
    println!(" | |  | |  \\| | |__   \\ V /| |__) | |   | |  | | |__) | |__  | |  | |         ");
    println!(" | |  | | . ` |  __|   > < |  ___/| |   | |  | |  _  /|  __| | |  | |         ");
    println!(" | |__| | |\\  | |____ / . \\| |    | |___| |__| | | \\ \\| |____| |__| |         ");
    println!(
        "  \\____/|_| \\_|______/_/ \\_\\_|    |______\\____/|_|  \\_\\______|_____/          "
    );

    // TERRITORIES
    println!("  _______ ______ _____  _____  _____ _______ ____  _____  _____ ______  _____ ");
    println!(" |__   __|  ____|  __ \\|  __ \\|_   _|__   __/ __ \\|  __ \\|_   _|  ____|/ ____|");
    println!("    | |  | |__  | |__) | |__) | | |    | | | |  | | |__) | | | | |__  | (___  ");
    println!("    | |  |  __| |  _  /|  _  /  | |    | | | |  | |  _  /  | | |  __|  \\___ \\ ");
    println!(
        "    | |  | |____| | \\ \\| | \\ \\ _| |_   | | | |__| | | \\ \\ _| |_| |____ ____) |"
    );
    println!(
        "    |_|  |______|_|  \\_\\_|  \\_\\_____|  |_|  \\____/|_|  \\_\\_____|______|_____/ "
    );
}

fn render_credits() {
    println!("----------------------");
    println!("dedicated to A.");
    println!("made with <3 by tanque");
    println!("----------------------");
}

fn main() {
    render_logo();
    render_credits();
}
