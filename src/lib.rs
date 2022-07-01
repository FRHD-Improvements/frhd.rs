use entities::{Powerup, PowerupTypes, RotatedPowerup, Teleport, Vehicle};

mod encode;
mod entities;

pub struct Track {
    pub lines: Vec<entities::Line>,
    pub powerups: Vec<entities::PowerupTypes>,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            lines: vec![],
            powerups: vec![],
        }
    }
}

impl Track {
    // Merge another track into this one
    pub fn merge(&mut self, track: &mut Self) {
        self.lines.append(&mut track.lines);
        self.powerups.append(&mut track.powerups);
    }

    // Straight Line
    pub fn insert_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, line_type: char) {
        self.lines
            .push(entities::Line::new(line_type, x1, y1, x2, y2));
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
            let t = i as f32 / 10.0;

            let mut control_point_1: [f32; 2] = p0.map(|x| x as f32 * (1.0 - t));
            control_point_1[0] += t * p1[0] as f32;
            control_point_1[1] += t * p1[1] as f32;

            let mut control_point_2: [f32; 2] = p1.map(|x| x as f32 * (1.0 - t));
            control_point_2[0] += t * p2[0] as f32;
            control_point_2[1] += t * p2[1] as f32;

            let mut main_control_point_float: [f32; 2] = control_point_1.map(|x| x * (1.0 - t));
            main_control_point_float[0] += t * control_point_2[0];
            main_control_point_float[1] += t * control_point_2[1];

            let main_control_point: [i32; 2] = [
                main_control_point_float[0].round() as i32,
                main_control_point_float[1].round() as i32,
            ];

            points.push(main_control_point);
        }

        for i in 1..points.len() - 1 {
            self.insert_line(
                points[i - 1][0],
                points[i - 1][1],
                points[i][0],
                points[i][1],
                line_type,
            );
        }
    }

    // Cubic Bezier Curve
    pub fn insert_cubic_curve(
        &mut self,
        p0: [i32; 2],
        c0: [i32; 2],
        c1: [i32; 2],
        p1: [i32; 2],
        line_type: char,
    ) {
        let mut points: Vec<[i32; 2]> = vec![];

        let accuracy = 35;

        for i in 0..accuracy {
            let t = i as f32 / accuracy as f32;

            // -- Nesting Level One
            let mut control_point_1: [f32; 2] = p0.map(|x| x as f32 * (1.0 - t));
            control_point_1[0] += t * c0[0] as f32;
            control_point_1[1] += t * c0[1] as f32;

            let mut control_point_2: [f32; 2] = c0.map(|x| x as f32 * (1.0 - t));
            control_point_2[0] += t * c1[0] as f32;
            control_point_2[1] += t * c1[1] as f32;

            let mut control_point_3: [f32; 2] = c1.map(|x| x as f32 * (1.0 - t));
            control_point_3[0] += t * p1[0] as f32;
            control_point_3[1] += t * p1[1] as f32;

            // -- Nesting Level Two
            let mut control_point_4: [f32; 2] = control_point_1.map(|x| x * (1.0 - t));
            control_point_4[0] += t * control_point_2[0];
            control_point_4[1] += t * control_point_2[1];

            let mut control_point_5: [f32; 2] = control_point_2.map(|x| x * (1.0 - t));
            control_point_5[0] += t * control_point_3[0];
            control_point_5[1] += t * control_point_3[1];

            // -- Nesting Level Three (last level)
            let mut main_control_point_float: [f32; 2] = control_point_4.map(|x| x * (1.0 - t));
            main_control_point_float[0] += t * control_point_5[0];
            main_control_point_float[1] += t * control_point_5[1];

            let main_control_point: [i32; 2] = [
                main_control_point_float[0].round() as i32,
                main_control_point_float[1].round() as i32,
            ];

            points.push(main_control_point);
        }

        for i in 1..points.len() - 1 {
            self.insert_line(
                points[i - 1][0],
                points[i - 1][1],
                points[i][0],
                points[i][1],
                line_type,
            );
        }
    }

    // Powerups
    pub fn insert_check(&mut self, x: i32, y: i32) {
        self.powerups
            .push(PowerupTypes::Pow(Powerup::new('C', x, y)));
    }

    pub fn insert_star(&mut self, x: i32, y: i32) {
        self.powerups
            .push(PowerupTypes::Pow(Powerup::new('T', x, y)));
    }

    pub fn insert_slow_mo(&mut self, x: i32, y: i32) {
        self.powerups
            .push(PowerupTypes::Pow(Powerup::new('S', x, y)));
    }

    pub fn insert_bomb(&mut self, x: i32, y: i32) {
        self.powerups
            .push(PowerupTypes::Pow(Powerup::new('O', x, y)));
    }

    pub fn insert_gravity(&mut self, x: i32, y: i32, rot: i32) {
        self.powerups
            .push(PowerupTypes::Row(RotatedPowerup::new('G', x, y, rot)));
    }

    pub fn insert_boost(&mut self, x: i32, y: i32, rot: i32) {
        self.powerups
            .push(PowerupTypes::Row(RotatedPowerup::new('B', x, y, rot)));
    }

    pub fn insert_anti_gravity(&mut self, x: i32, y: i32) {
        self.powerups
            .push(PowerupTypes::Pow(Powerup::new('A', x, y)));
    }

    pub fn insert_teleport(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.powerups
            .push(PowerupTypes::Tp(Teleport::new(x1, y1, x2, y2)));
    }

    // Vehicles
    pub fn insert_helicopter(&mut self, x: i32, y: i32) {
        self.powerups
            .push(PowerupTypes::Veh(Vehicle::new('1', x, y)));
    }

    pub fn insert_truck(&mut self, x: i32, y: i32) {
        self.powerups
            .push(PowerupTypes::Veh(Vehicle::new('2', x, y)));
    }

    pub fn insert_balloon(&mut self, x: i32, y: i32) {
        self.powerups
            .push(PowerupTypes::Veh(Vehicle::new('3', x, y)));
    }

    pub fn insert_blob(&mut self, x: i32, y: i32) {
        self.powerups
            .push(PowerupTypes::Veh(Vehicle::new('4', x, y)));
    }

    // Clear all of the information in the track
    pub fn clear(&mut self) {
        self.lines = vec![];
        self.powerups = vec![];
    }

    // Track code generation
    pub fn generate_code(&mut self) -> String {
        let mut physical_data: Vec<String> = vec![];
        let mut scenery_data: Vec<String> = vec![];
        let mut powerup_data: Vec<String> = vec![];

        for line in &self.lines {
            if line.line_type == 'p' {
                physical_data.push(line.encode());
            } else {
                scenery_data.push(line.encode());
            }
        }

        for powerup in &self.powerups {
            match powerup {
                PowerupTypes::Row(a) => powerup_data.push(a.encode()),
                PowerupTypes::Pow(a) => powerup_data.push(a.encode()),
                PowerupTypes::Veh(a) => powerup_data.push(a.encode()),
                PowerupTypes::Tp(a) => powerup_data.push(a.encode()),
            }
        }

        format!(
            "{}#{}#{}",
            physical_data.join(","),
            scenery_data.join(","),
            powerup_data.join(",")
        )
    }
}

// I might as well provide a public method to simply encode numbers
pub fn encode(target: i32) -> String {
    encode::base32_encode(target)
}
