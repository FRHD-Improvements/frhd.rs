#![allow(dead_code)]

mod encode;
mod entities;

pub struct Track {
    pub trackdata: String,
    pub physical: Vec<String>,
    pub scenery: Vec<String>,
    pub powerups: String,
}

impl Track {
    // Lines
    pub fn insert_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, line_type: char) {
        if line_type == 'p' {
            self.physical.push(entities::Line{ line_type, x1, y1, x2, y2 }.encode());
        } else {
            self.scenery.push(entities::Line{ line_type, x1, y1, x2, y2 }.encode());
        }
    }

    // Insert diagonal stripes
    pub fn insert_stripes(&mut self, size: i32, separation: i32, x: i32, y: i32, line_type: char) {
        // top/left
        for i in 0..size*2 {
            if i % separation == 0 {
                let x1 = x - size;
                let y1 = y + (size - 1 - i);
                let x2 = x - (size - 1 - i);
                let y2 = y + size;

                if line_type == 'p' {
                    self.physical.push(entities::Line{ line_type, x1, y1, x2, y2 }.encode());
                } else {
                    self.scenery.push(entities::Line{ line_type, x1, y1, x2, y2 }.encode());
                }
            }
        }

        // bottom/right
        for i in 0..(size*2)-1 {
            if i % separation == 0 {
                let x1 = x + size;
                let y1 = y - (size - 1 - i);
                let x2 = x + (size - 1 - i);
                let y2 = y - size;

                if line_type == 'p' {
                    self.physical.push(entities::Line{ line_type, x1, y1, x2, y2 }.encode());
                } else {
                    self.scenery.push(entities::Line{ line_type, x1, y1, x2, y2 }.encode());
                }
            }
        }
    }

    // Fill a box (just use stripes with no space between them)
    pub fn insert_box(&mut self, size: i32, x: i32, y: i32, line_type: char) {
        self.insert_stripes(size, 1, x, y, line_type);
    }


    // Powerups
    pub fn insert_check(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup { powerup_type: 'C', x, y, rotation: 999 }.encode();
    }

    pub fn insert_star(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup { powerup_type: 'T', x, y, rotation: 999 }.encode();
    }

    pub fn insert_slow_mo(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup { powerup_type: 'S', x, y, rotation: 999 }.encode();
    }

    pub fn insert_bomb(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup { powerup_type: 'O', x, y, rotation: 999 }.encode();
    }

    pub fn insert_gravity(&mut self, x: i32, y: i32, rot: i32) {
        self.powerups += &entities::Powerup { powerup_type: 'G', x, y, rotation: rot }.encode();
    }

    pub fn insert_boost(&mut self, x: i32, y: i32, rot: i32) {
        self.powerups += &entities::Powerup { powerup_type: 'B', x, y, rotation: rot }.encode();
    }

    pub fn insert_anti_gravity(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup { powerup_type: 'A', x, y, rotation: 999 }.encode();
    }

    pub fn insert_teleport(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.powerups += &entities::Teleport { x1, y1, x2, y2 }.encode();
    }

    // Vehicles
    pub fn insert_helicopter(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup { powerup_type: '1', x, y, rotation: 1000 }.encode();
    }

    pub fn insert_truck(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup { powerup_type: '2', x, y, rotation: 1000 }.encode();
    }

    pub fn insert_balloon(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup { powerup_type: '3', x, y, rotation: 1000 }.encode();
    }

    pub fn insert_blob(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup { powerup_type: '4', x, y, rotation: 1000 }.encode();
    }

    // Track code generation
    pub fn generate_code(&mut self) -> String {
        for physical_line in &self.physical {
            self.trackdata += physical_line;
        }

        for scenery_line in &self.scenery {
            self.trackdata += scenery_line;
        }

        let mut final_data = String::new();

        for line in &self.physical {
            final_data += line;
        }

        final_data += "#";

        for line in &self.scenery {
            final_data += line;
        }

        final_data += "#";
        final_data += &self.powerups;
        final_data += "#";

        return final_data;
    }
}
