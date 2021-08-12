# FRHD.rs

FRHD.rs is a tool used to create FreeRider HD tracks using the rust programming language.


For JavaScript look at [ObeyLordGoomy's tool](https://github.com/ObeyLordGoomy/frhd.js).


For Python look at [Gaetgu's tool](https://pypi.org/project/frhd-python/).


## Usage

Using the library is pretty straightforward. Here are a few examples:

```rust
use frhd::*;

// Here is an example of implementing a new track
fn main() {
    let mut my_track = Track {
        trackdata: String::new(),
        physical: Vec::new(),
        scenery: Vec::new(),
        powerups: String::new(),
    };
}

// Want to create a new line? Simple! `track_type` should be a 'p' or 's', for physical or scenery.
my_track.insLine(x1, y1, x2, y2, track_type);

// Here are the powerup types. Teleport and vehicles are coming soon!
my_track.insert_check(x, y);
my_track.insert_star(x, y);
my_track.insert_slow_mo(x, y);
my_track.insert_bomb(x, y);
my_track.insert_gravity(x, y, rotation);
my_track.insert_boost(x, y, rotation);

// To get the track code as a string, run this method:
my_track.generate_code();
```
