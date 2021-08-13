use crate::encode;

pub struct Powerup {
    pub powerup_type: char,
    pub x: i32,
    pub y: i32,
    pub rotation: i32,
}

impl Powerup {
    pub fn encode(self) -> String {
        if self.rotation == 999 {
            return format!("{} {} {},", 
                           self.powerup_type.to_string(), 
                           encode::base32_encode(self.x), 
                           encode::base32_encode(self.y * -1)
            );
        } else if self.rotation == 1000 {
            return format!("V {} {} {} a,",
                            encode::base32_encode(self.x),
                            encode::base32_encode(self.y * -1),
                            self.powerup_type.to_string()
            );
        }
        
        return format!("{} {} {} {},", 
                       self.powerup_type, 
                       encode::base32_encode(self.x), 
                       encode::base32_encode(self.y * -1), 
                       encode::base32_encode(self.rotation)
        );
    }
}


pub struct Teleport {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Teleport {
    pub fn encode(self) -> String {
        format!("W {} {} {} {}",
                encode::base32_encode(self.x1),
                encode::base32_encode(self.y1 * -1),
                encode::base32_encode(self.x2),
                encode::base32_encode(self.y2 * -1)
        )
    }
}


pub struct Line {
    pub line_type: char,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Line {
    pub fn encode(self) -> String {
        return format!("{} {} {} {},",
                        encode::base32_encode(self.x1),
                        encode::base32_encode(self.y1 * -1),
                        encode::base32_encode(self.x2),
                        encode::base32_encode(self.y2 * -1)
        );
    }
}
