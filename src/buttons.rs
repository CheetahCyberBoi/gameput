use anyhow::Result;
use enigo::{Enigo, Key, Keyboard};

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