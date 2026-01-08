fn main() {
    println!("Attempting to simulate mouse movement...");

    // Call the function from our library
    match grab_hand::uinput_handler::simulate_mouse_move() {
        Ok(_) => {
            println!("Mouse movement simulated successfully!");
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to simulate mouse movement: {}", e);
            eprintln!("Please ensure you have the correct permissions for /dev/uinput.");
            eprintln!("You might need to run this program with 'sudo' or set up a udev rule.");
        }
    }
}