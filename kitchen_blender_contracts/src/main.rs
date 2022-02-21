use contracts;
use contracts::invariant;

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

#[contracts::invariant(self.speed>=0)]
#[contracts::invariant(self.speed<10)]
impl FoodProcessor for Blender {
    fn get_speed(&self) -> i32 {
        return self.speed;
    }
    #[contracts::requires(i32::abs(self.get_speed() - speed) == 1 )]
    #[contracts::requires(self.is_full() || speed == 0)]
    #[contracts::ensures(self.get_speed() == speed)]
    fn set_speed(&mut self, speed: i32) {
        self.speed = speed;
    }
    fn is_full(&self) -> bool {
        return self.full;
    }
    #[contracts::requires(!self.is_full())]
    #[contracts::ensures(self.is_full())]
    fn fill(&mut self) {
        self.full = true;
    }
    #[contracts::requires(self.is_full())]
    #[contracts::ensures(!self.is_full())]
    fn empty(&mut self) {
        self.full = false;
    }
}

fn main() {
    let blender = &mut Blender {
        speed: 0,
        full: false,
    };
    blender.fill();
    blender.set_speed(1);
}
