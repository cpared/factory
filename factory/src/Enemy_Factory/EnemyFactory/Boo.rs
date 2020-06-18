struct Boo {
    damage: u8
}

impl Boo {

    fn attack(&self){
        println!("My damage is {}", self.damage);
    }
}
