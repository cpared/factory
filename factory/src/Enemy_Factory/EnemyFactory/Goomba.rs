struct Goomba{
    damage: u8
}

impl Goomba {

    fn attack(&self){
        println!("My damage is {}", self.damage);
    }
}
