use std::thread;
use std::time::Duration;
use uinput::event::relative::Position::{X, Y};
use uinput::event::relative::Relative::Position;
use uinput::event::Event::Relative;

pub fn simulate_mouse_move() -> uinput::Result<()> {
    //create a virtual device
    let mut device = uinput::default()?
        .name("GrabHand Virtual Mouse")?
        .event(uinput::event::controller::Mouse::Left)?
        .event(Relative(Position(X)))?
        .event(Relative(Position(Y)))?
        .create()?;
    
    //wait for device init
    thread::sleep(Duration::from_secs(1));

    //simulate move: move towards right 100 pixels
    device.send(X, 100)?;
    device.synchronize()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_logic(){
        //call the real hardware interface
        let result = simulate_mouse_move();
        assert!(result.is_ok(),"device init failed,please check the /dev/uinput access permission!")
    }
}
