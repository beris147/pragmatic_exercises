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

impl FoodProcessor for Blender {
    fn get_speed(&self) -> i32 {
        return self.speed;
    }
    fn set_speed(&mut self, speed: i32) {
        assert!(i32::abs(self.get_speed() - speed) == 1);
        assert!(self.is_full() || speed == 0);
        self.speed = speed;
        assert!(self.get_speed() == speed);
    }
    fn is_full(&self) -> bool {
        return self.full;
    }
    fn fill(&mut self) {
        assert!(self.is_full() == false);
        self.full = true;
        assert!(self.is_full() == true);
    }
    fn empty(&mut self) {
        assert!(self.is_full() == true);
        self.full = false;
        assert!(self.is_full() == false);
    }
}

fn main() {
    let blender = &mut Blender {
        speed: 0,
        full: false,
    };
    blender.fill();
}
