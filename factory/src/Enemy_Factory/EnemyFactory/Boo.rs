const DAMAGE: i8 = 10;

pub struct Boo;

impl Boo {

    pub fn attack(&self){
        println!("My damage is {}", DAMAGE);
    }
}
