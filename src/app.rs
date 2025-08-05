use std::{thread, sync::mpsc::channel};

use anyhow::Result;
use enigo::{Enigo, Settings, Mouse, Coordinate};
use gilrs::{Gamepad, Gilrs, Axis, Event, EventType, Button, ev::filter::Repeat, Filter};
use log::{info, debug};

/// The main state of the application. 
/// Holds the main components of everything, and runs the main logic.
pub struct App<'a> {
    /// A instance of Enigo to use as our input handler
    enigo: Enigo,

    /// The handle to the current gamepad. May be empty if a gamepad is not found or not initialized. 
    gamepad: Option<Gamepad<'a>>,
    
    /// Whether or not the application should exit.
    should_exit: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self> {
        let enigo = Enigo::new(&Settings::default())?;
        return Ok(Self {
            gamepad: None,
            enigo,
            should_exit: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        // The basic structure should be something like this:
        // There should be a few tasks: 
        // One that scans for connected gamepads and adds them to the `gamepad` field of the struct
        // And one that translates inputs and outputs them.
        // This will necessitate multiple channels, one to communicate with the gamepad task and one to
        // communicate with the input task.
        // IDFK how to do this.
        // TODO: how tf do i do this
        // Idea: Make a seperate `InputHandler` and `GamepadLocator` structs that accept any necessary parameters and one end of a channel
        // and have a `dispatch` method that dispatches the task to simplify things
        // For now we do the stupid approach of just sticking everything in a big loop before we scrunch it out

        // Create a channel for the gamepad thread to send messages to the other thread

        

        Ok(())
    }
}