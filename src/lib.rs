use anyhow::Result;
use enigo::{Key, Enigo, Settings};
use gilrs::{Gilrs, Event, EventType, Button, Axis};
use log::info;

mod buttons;
mod octant;

use crate::buttons::Buttons;


pub fn run() -> Result<()> {
    // Make Enigo
    // Make gamepad object and find one and latch onto it
    // Make our buttons
    // Loop:
    // Read gamepad state and construct button pressed array
    // Send it to buttons
    // End.
    let mut enigo = Enigo::new(&Settings::default())?;

    let mut gilrs = Gilrs::new().unwrap();

    pretty_env_logger::init();

    // Find a gamepad
    for (id, gamepad) in gilrs.gamepads() {
        info!("Found a gamepad with ID {} called {}", id, gamepad.name());
    }

    let buttons_array: [Key; 6] = [
        Key::Unicode('a'),
        Key::Unicode('s'),
        Key::Unicode('d'),
        Key::Unicode('f'),
        Key::Unicode('g'),
        Key::Unicode('h'),
    ];
    let buttons = Buttons::from_array(buttons_array);
    let mut y = 0f32;
    let mut x = 0f32;
    let mut active_gamepad = None;
    loop {
        let mut actions_this_iter: [bool; 6] = [false, false, false, false, false, false];
        // Read the controller's state
        while let Some(Event { id, event, time, ..}) = gilrs.next_event() {
            info!("{:?} New event from {}: {:?}", time, id, event);
            // Examine the event.
            match event {
                EventType::ButtonPressed(Button::North,..) => actions_this_iter[0] = true,
                EventType::ButtonReleased(Button::North,..) => actions_this_iter[0] = false,
                EventType::ButtonPressed(Button::South,..) => actions_this_iter[1] = true,
                EventType::ButtonReleased(Button::South,..) => actions_this_iter[1] = false,
                EventType::ButtonPressed(Button::East,..) => actions_this_iter[2] = true,
                EventType::ButtonReleased(Button::East,..) => actions_this_iter[2] = false,
                EventType::ButtonPressed(Button::West,..) => actions_this_iter[3] = true,
                EventType::ButtonReleased(Button::West,..) => actions_this_iter[3] = false,
                EventType::ButtonPressed(Button::LeftTrigger2,..) => actions_this_iter[4] = true,
                EventType::ButtonReleased(Button::LeftTrigger2,..) => actions_this_iter[4] = false,
                EventType::ButtonPressed(Button::RightTrigger2,..) => actions_this_iter[5] = true,
                EventType::ButtonReleased(Button::RightTrigger2,..) => actions_this_iter[5] = false,
                EventType::AxisChanged(Axis::LeftStickX, amt, ..) => x = amt,
                EventType::AxisChanged(Axis::LeftStickY, amt, ..) => y = amt,
                _ => {},
            }
            active_gamepad = Some(id);
        }
        let stick_dir = octant::StickDir::from_xy_coords(x, y);
        match stick_dir {
            octant::StickDir::North => info!("29o3847928734"),
            _ => {},
        }
        // We have our actions, now do the funny.
        buttons.perform_actions(&mut enigo, actions_this_iter)?;
    }

    Ok(())
}