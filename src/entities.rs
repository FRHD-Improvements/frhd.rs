use crate::encode;

pub enum PowerupTypes {
    Row(RotatedPowerup),
    Pow(Powerup),
    Veh(Vehicle),
    Tp(Teleport),
}

#[derive(Clone, Copy)]
pub struct RotatedPowerup {
    powerup_type: char,
    pub x: i32,
    pub y: i32,
    pub rotation: i32,
}

impl RotatedPowerup {
    pub fn new(powerup_type: char, x: i32, y: i32, rotation: i32) -> Self {
        Self {
            powerup_type,
            x,
            y,
            rotation,
        }
    }

    pub fn encode(self) -> String {
        format!(
            "{} {} {} {}",
            self.powerup_type,
            encode::base32_encode(self.x),
            encode::base32_encode(self.y * -1),
            encode::base32_encode(self.rotation)
        )
    }
}

#[derive(Clone, Copy)]
pub struct Powerup {
    powerup_type: char,
    x: i32,
    y: i32,
}

impl Powerup {
    pub fn new(powerup_type: char, x: i32, y: i32) -> Self {
        Self { powerup_type, x, y }
    }

    pub fn encode(self) -> String {
        format!(
            "{} {} {}",
            self.powerup_type,
            encode::base32_encode(self.x),
            encode::base32_encode(self.y)
        )
    }
}

#[derive(Clone, Copy)]
pub struct Vehicle {
    vehicle_type: char,
    pub x: i32,
    pub y: i32,
}

impl Vehicle {
    pub fn new(vehicle_type: char, x: i32, y: i32) -> Self {
        Self { vehicle_type, x, y }
    }

    pub fn encode(self) -> String {
        format!(
            "V {} {} {} a",
            self.vehicle_type,
            encode::base32_encode(self.x),
            encode::base32_encode(self.y)
        )
    }
}

#[derive(Clone, Copy)]
pub struct Teleport {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Teleport {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn encode(self) -> String {
        format!(
            "W {} {} {} {}",
            encode::base32_encode(self.x1),
            encode::base32_encode(self.y1 * -1),
            encode::base32_encode(self.x2),
            encode::base32_encode(self.y2 * -1)
        )
    }
}

#[derive(Clone, Copy)]
pub struct Line {
    pub line_type: char,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Line {
    pub fn new(line_type: char, x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            line_type,
            x1,
            y1,
            x2,
            y2,
        }
    }

    pub fn encode(self) -> String {
        return format!(
            "{} {} {} {}",
            encode::base32_encode(self.x1),
            encode::base32_encode(self.y1 * -1),
            encode::base32_encode(self.x2),
            encode::base32_encode(self.y2 * -1)
        );
    }
}
