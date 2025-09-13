use std::f32::consts::PI;

use log::info;


#[derive(Clone, Copy, Debug, Default)]
pub enum StickDir {
    #[default]
    Central,
    North,
    West,
    South,
    East,
    Northeast,
    Southeast,
    Northwest,
    Southwest,
}

impl StickDir {
    /// Creates a direction from (x, y) coordinates on a unit circle.
    pub fn from_xy_coords(x: f32, y: f32) -> Self {
        // If we're at the center, return center!
        let epsilon: f32 = 0.001;
        if (x < epsilon && -epsilon < x) && (y < epsilon && -epsilon < y) {
            return Self::Central;
        } else {
            // Compute the atangant of the points and add 180 if the x is negative. 
            let mut arctangent = x.atan2(y);
            arctangent = (180.0/PI) * arctangent;
            if x < 0.0 {
                arctangent += 360.0;
            }
            arctangent = arctangent.round();
            // Match the directions to ranges.
            // Quick debug check:
            info!("Arctan results: {}", arctangent);
            // Check and return the right values.
            let pho = 10f32;
            // North, around 360 to 0 deg.deg.
            if 360.0 - pho >= x && x >= 0.0 + pho {
                info!("eeeee");
                return Self::North;
            }
        }

        Self::default()
    }
}


#[derive(Clone,Copy,Debug)]
pub struct Octant {

}