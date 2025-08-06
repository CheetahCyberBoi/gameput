# Design for Gameput

## Features

The overall idea of `gameput` is to offer a method for people to interact with their machines via a controller.
The main features of `gameput` should be as follows:
- The ability to input various characters and symbols
- The ability to move the mouse cursor around and interact via the mouse
- The ability to overlay an "on-screen keyboard" of sorts to help with input.

Additionally, most things should be configurable, namely the layers and their content, and the colorschemes and styling of the overlay. This will be done
in two configuration files: `$XDG_CONFIG_HOME/gameput/layers.toml` and `$XDG_CONFIG_HOME/gameput/theme.toml`, respectively. `layers.toml` also holds general configuration settings, like mouse sensitivity, default layer preference, and such.


## Terminology and Usage

### Keyboard mode and Mouse mode
There are two main "modes" of `gameput`: mouse mode and keyboard mode.

- **Mouse mode**: Left stick moves the cursor, while left and right trigger control the left and right mouse buttons (configurable, can be swapped to a more console-style layout)
- **Keyboard mode** uses the Left Stick as an octant selector, while the 4 top-right buttons and the 2 triggers as inputs. 
This is toggleable with the Right Stick's button.


### Keyboard input

The main method of keyboard input in `gameput` can be split into five components:

1. The **config file**, by default `$XDG_CONFIG_HOME/gameput/layers.toml`
2. Any number of **layers**, or a set of two modes containing octants.
3. Two **modes**, one for when the mode toggle (defaults to Left Stick Button) is pressed and one for when it's not
4. 9 **octants**, or a set of buttons that have a given action depending on the current mode toggle and left stick position
5. 6 buttons, **North, South, East, West, LeftTrigger, and RightTrigger**

#### Usage

When in keyboard mode, you can use the **Left Stick** to select an octant given your current mode
Using the **Left and Right Bumpers** will switch the current mode (Right switches to the next mode, Left to the previous)
While an octant is selected, you can press any of the **buttons** (usually YXAB or TriangleSquareCrossCircle) to input a character. 
Pressing **Left Stick** will switch the mode. This allows for more characters to be input compared to just one mode. This can be used for things like extra punctuation or capitalized letters

### Mouse input

Mouse input in `gameput` is fairly simple. While in mouse mode, you may:

- Use the **Left Stick** to move the mouse cursor around, depending on the sensitivity it may be fast or slow (configurable)
- Use the **Left and Right Bumpers** to do left and right click, respectively (this can be configured)
- Scrolling is done via the **Right Stick**, with up scrolling up, down scrolling down, and left and right doing guess what.

## Design

# TODO: Fix the diagram, make it an image, and rewrite it to suit the above.

Overall, `gameput` should be designed such that it can still run in the background even if the controller disconnects.
Alongside this, it should also be able to function in tandem with the keyboard+mouse, i.e. not taking over input.

The program flow of `gameput` is as follows:

```
Entrypoint (aka `main`)
        |
        |
Parse command-line arguments.
(using `clap`)
        |
        |
Read from configuration (required, does not work without at least a `layers.toml`) --------> Initialize layermap
        |                                                                                              |
        |                                                                                              |
Construct application state <--------------------------------------------------------------------------+
        |
        |
Dispatch gamepad listener thread -------------> SEE "Gamepad Listener Flowchart" below. 
        |
        |
Enter main loop
        |
        |
Listen for `GamepadEvents`s from the gamepad listener thread ---None-----------------------------------------------------------------------------------------------------------------------------------------+
        |                                                                                                                                                                                                    |
      Some                                                                                                                                                                                                   |
        |                                                                                                                                                                                                    |
Match against the current layer's `Action`s given current joystick state ------ Some(AxisChanged(Axis::Left)) ----------------------- Some(AxisChanged(Axis::Right)) --------------------- None -------------+
        |                                                                                |                                                          |                                       |                |
       Some(ButtonPressed(any of the XYAB))                                              |                                                      Update internal mouse cursor position by `dir*sensitivity`   |
        |                                                                            Update internal selected-octant state                                                                                   |
Given currently selected octant, set the output to be performed after listening block is done.                                                                                                               |
        |                                                                                                                                                                                                    |
        |                                                                                                                                                                                                    |
        |                                                                                                                                                                                                    |
Apply action from listening block, depending on what's selected. If none selected, do nothing. <-------------------------------------------------------------------------------------------------------------+ 
        |
        |
If `overlay` is set in the config, display overlay (whole nother can of worms)
        |
        |
Go back to start of loop
```

For the gamepad listener thread:

```
Entrypoint
        |
        |
Initialize `gilrs` and create a list of connected controllers
        |
        |
Enter main loop
        |
        |
Loop over all gamepads currently connected. If one that isn't in the list of connected controllers is there, add it. If vice versa, remove it form the list of connected controllers.
        |
Loop over all gamepads
        |
See if they have any events ------------------ None-------------------------+
        |                                                                   |
        Some                                                                |
        |                                                                   |
Convert to `GamepadAction` and send down gamepad listener channel.          |
        |                                                                   |
Go to main loop. <----------------------------------------------------------+
```