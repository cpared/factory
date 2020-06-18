struct Koopa {
    damage: u8
}

impl Koopa {

    fn attack(&self){
        println!("My damage is {}", self.damage);
    }
}
