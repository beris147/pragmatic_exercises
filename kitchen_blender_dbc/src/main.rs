use dbc;
use dbc::dbc_panic;

trait FoodProcessor {
    fn get_speed(&self) -> i32;
    fn set_speed(&mut self, speed: i32);
    fn is_full(&self) -> bool;
    fn fill(&mut self);
    fn empty(&mut self);
}

struct Blender {
    speed: i32,
    full: bool,
}

impl dbc::Invariant for Blender {
    fn invariant(&self) -> bool {
        return self.speed >= 0 && self.speed <= 9;
    }
}

impl FoodProcessor for Blender {
    fn get_speed(&self) -> i32 {
        return self.speed;
    }
    fn set_speed(&mut self, speed: i32) {
        dbc::require!(i32::abs(self.get_speed() - speed) == 1);
        dbc::require!(self.is_full() || speed == 0);
        self.speed = speed;
        dbc::ensure!(self.get_speed() == speed);
    }
    fn is_full(&self) -> bool {
        return self.full;
    }
    fn fill(&mut self) {
        dbc::require!(self.is_full() == false);
        self.full = true;
        dbc::ensure!(self.is_full() == true);
    }
    fn empty(&mut self) {
        dbc::require!(self.is_full() == true);
        self.full = false;
        dbc::ensure!(self.is_full() == false);
    }
}

fn main() {
    let blender = &mut Blender {
        speed: 0,
        full: false,
    };
    blender.fill();
}
