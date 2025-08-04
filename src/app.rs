use anyhow::Result;
use enigo::{Enigo, Settings};
use gilrs::Gamepad;

/// The main state of the application. 
/// Holds the main components of everything, and runs the main logic.
pub struct App<'a> {
    /// A instance of Enigo to use as our input handler
    enigo: Enigo,

    /// The handle to the current gamepad. May be empty if a gamepad is not found or not initialized. 
    gamepad: Option<Gamepad<'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self> {
        let enigo = Enigo::new(&Settings::default())?;
        return Ok(Self {
            gamepad: None,
            enigo
        })
    }

    pub fn run(&self) -> Result<()> {
        // The basic structure should be something like this:
        // There should be a few tasks: 
        // One that scans for connected gamepads and adds them to the `gamepad` field of the struct
        // And one that translates inputs and outputs them.
        // This will necessitate multiple channels, one to communicate with the gamepad task and one to
        // communicate with the input task.
        // IDFK how to do this.
        // TODO: how tf do i do this

        Ok(())
    }
}