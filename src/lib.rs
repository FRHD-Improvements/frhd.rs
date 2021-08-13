#![allow(dead_code)]

// NOTE: This isn't working right now. Why? Good question.
// TODO: Document my code so I can use it in the future.

mod encode;
mod entities;

pub struct Track {
    pub trackdata: String,
    pub physical: Vec<String>,
    pub scenery: Vec<String>,
    pub powerups: String,
}

impl Track {
    pub fn insert_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, line_type: char) {
        if line_type == 'p' {
            self.physical.push(entities::Line{ line_type, x1, y1, x2, y2 }.encode());
        } else {
            self.scenery.push(entities::Line{ line_type, x1, y1, x2, y2 }.encode());
        }
    }
    
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
