use anyhow::Result;
use enigo::{Key, Enigo, Keyboard, Settings};
use gilrs::{Gilrs, Event, EventType, Button, ev::filter::Repeat, Filter};
use log::info;

#[derive(Clone, Copy, Debug)]
pub struct Buttons {
    pub north_key: Key,
    pub south_key: Key,
    pub east_key: Key,
    pub west_key: Key,
    pub right_trigger_key: Key,
    pub left_trigger_key: Key,
}

impl Buttons {
    pub fn new(north_key: Key, south_key: Key, east_key: Key, west_key: Key, right_trigger_key: Key, left_trigger_key: Key) -> Self{
        Self {
            north_key,
            south_key,
            east_key,
            west_key,
            right_trigger_key,
            left_trigger_key,
        }
    }

    /// Constructs a `Buttons` based off of an array of key values in this order: North, South, East, West, RightTrigger, LeftTrigger.
    pub fn from_array(keys: [Key; 6]) -> Self {
        Self {
            north_key: keys[0],
            south_key: keys[1],
            east_key: keys[2],
            west_key: keys[3],
            right_trigger_key: keys[4],
            left_trigger_key: keys[5],
        }
    }

    //Performs an action using Enigo based off of gamepad button states. These are provided in a 6-element array like `from_array`.
    pub fn perform_actions(&self, enigo: &mut Enigo, gamepad_state: [bool; 6]) -> Result<()> {
        if gamepad_state[0] == true {
            enigo.key(self.north_key, enigo::Direction::Click)?;
        }
        if gamepad_state[1] == true {
            enigo.key(self.south_key, enigo::Direction::Click)?;
        }
        if gamepad_state[2] == true {
            enigo.key(self.east_key, enigo::Direction::Click)?;
        }
        if gamepad_state[3] == true {
            enigo.key(self.west_key, enigo::Direction::Click)?;
        }
        if gamepad_state[4] == true {
            enigo.key(self.right_trigger_key, enigo::Direction::Click)?;
        }
        if gamepad_state[5] == true {
            enigo.key(self.left_trigger_key, enigo::Direction::Click)?;
        }
        Ok(())
    }
}


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
                _ => {},
            }
            active_gamepad = Some(id);
        }
        // We have our actions, now do the funny.
        buttons.perform_actions(&mut enigo, actions_this_iter)?;
    }

    Ok(())
}