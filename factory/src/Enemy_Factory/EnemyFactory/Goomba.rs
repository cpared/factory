const DAMAGE: i8 = 20;

pub struct Goomba;

impl Goomba {

    pub fn attack(&self){
        println!("My damage is {}", DAMAGE);
    }
}
