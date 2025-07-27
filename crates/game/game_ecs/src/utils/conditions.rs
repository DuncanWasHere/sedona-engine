use crate::utils::constants::{DAWN, DUSK};

pub fn is_night(solar_time: f32) -> bool {
    if solar_time < DAWN || solar_time > DUSK {
        true
    } else {
        false
    }
}
