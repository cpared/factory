
mod boo;
mod goomba;
mod koopa;
use boo::Boo;
use koopa::Koopa;
use goomba::Goomba;

pub struct Factory;

impl Factory {

    pub fn create_boo(&self) -> Boo {
        return Boo{};
    }

    pub fn create_koopa(&self) -> Koopa {
        return Koopa{};
    }

    pub fn create_goomba(&self) -> Goomba {
        return Goomba{};
    }
}
