const DAMAGE: i8 = 50;

pub struct Koopa;

impl Koopa {

    pub fn attack(&self){
        println!("My damage is {}", DAMAGE);
    }
}
