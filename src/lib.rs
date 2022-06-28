mod encode;
mod entities;

// TODO: Use vectors of types for the physical, scenery, and powerups fields. (Encode in the named step, not everytime something is added)
pub struct Track {
    pub trackdata: String,
    pub physical: Vec<String>,
    pub scenery: Vec<String>,
    pub powerups: String,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            trackdata: String::new(),
            physical: vec![],
            scenery: vec![],
            powerups: String::new(),
        }
    }
}

impl Track {
    // Merge another track into this one
    pub fn merge(&mut self, track: &mut Self) {
        self.trackdata.push_str(&track.trackdata);
        self.physical.append(&mut track.physical);
        self.scenery.append(&mut track.scenery);
        self.powerups.push_str(&track.powerups);
    }

    // Straight Line
    pub fn insert_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, line_type: char) {
        if line_type == 'p' {
            self.physical.push(
                entities::Line {
                    line_type,
                    x1,
                    y1,
                    x2,
                    y2,
                }
                .encode(),
            );
        } else {
            self.scenery.push(
                entities::Line {
                    line_type,
                    x1,
                    y1,
                    x2,
                    y2,
                }
                .encode(),
            );
        }
    }

    // Quadratic Bezier Curve
    pub fn insert_quadratic_curve(
        &mut self,
        p0: [i32; 2],
        p1: [i32; 2],
        p2: [i32; 2],
        line_type: char,
    ) {
        let mut points: Vec<[i32; 2]> = vec![];

        for i in 0..10 {
            let t: f32 = i as f32 / 10.0;
            let mut point: [i32; 2] = [0; 2];

            point[0] = ((1.0 - t).powf(2.0) * p0[0] as f32
                + 2.0 * (1.0 - t) * t * p1[0] as f32
                + t.powf(2.0) * p2[0] as f32) as i32;

            point[1] = ((1.0 - t).powf(2.0) * p0[1] as f32
                + 2.0 * (1.0 - t) * t * p1[1] as f32
                + t.powf(2.0) * p2[1] as f32) as i32;

            points.push(point);
        }

        let mut first_index = 0;

        for i in 1..points.len() - 1 {
            if (points[first_index][1] - points[i][1] < 2
                && points[first_index][1] - points[i][1] > -2)
                || (points[first_index][0] - points[i][0] < 2
                    && points[first_index][0] - points[i][0] > -2)
            {
                break;
            }

            self.insert_line(
                points[first_index][0],
                points[first_index][1],
                points[i][0],
                points[i][1],
                line_type,
            );
            first_index += 1;
        }
    }

    // Powerups
    pub fn insert_check(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: 'C',
            x,
            y,
            rotation: 999,
        }
        .encode();
    }

    pub fn insert_star(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: 'T',
            x,
            y,
            rotation: 999,
        }
        .encode();
    }

    pub fn insert_slow_mo(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: 'S',
            x,
            y,
            rotation: 999,
        }
        .encode();
    }

    pub fn insert_bomb(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: 'O',
            x,
            y,
            rotation: 999,
        }
        .encode();
    }

    pub fn insert_gravity(&mut self, x: i32, y: i32, rot: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: 'G',
            x,
            y,
            rotation: rot,
        }
        .encode();
    }

    pub fn insert_boost(&mut self, x: i32, y: i32, rot: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: 'B',
            x,
            y,
            rotation: rot,
        }
        .encode();
    }

    pub fn insert_anti_gravity(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: 'A',
            x,
            y,
            rotation: 999,
        }
        .encode();
    }

    pub fn insert_teleport(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.powerups += &entities::Teleport { x1, y1, x2, y2 }.encode();
    }

    // Vehicles
    pub fn insert_helicopter(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: '1',
            x,
            y,
            rotation: 1000,
        }
        .encode();
    }

    pub fn insert_truck(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: '2',
            x,
            y,
            rotation: 1000,
        }
        .encode();
    }

    pub fn insert_balloon(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: '3',
            x,
            y,
            rotation: 1000,
        }
        .encode();
    }

    pub fn insert_blob(&mut self, x: i32, y: i32) {
        self.powerups += &entities::Powerup {
            powerup_type: '4',
            x,
            y,
            rotation: 1000,
        }
        .encode();
    }

    // Clear all of the information in the track
    pub fn clear(&mut self) {
        self.trackdata = String::new();
        self.physical = vec![];
        self.scenery = vec![];
        self.powerups = String::new();
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

// I might as well provide a method to simply encode numbers
pub fn encode(target: i32) -> String {
    encode::base32_encode(target)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_merge() {
        let mut t_a: crate::Track = Default::default();
        let mut t_b: crate::Track = Default::default();
        t_a.merge(&mut t_b);
    }
}
