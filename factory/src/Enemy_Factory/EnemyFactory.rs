mod Boo;
mod Koopa;
mod Goomba;
use Boo::Boo;
use Koopa::Koopa;
use Goomba::Goomba;

pub struct EnemyFactory;

impl EnemyFactory {

    fn create_boo(&self) -> Boo {
        return Boo(damage : 10);
    }

    fn create_koopa(&self) -> Koopa {
        return Koopa(damage : 50);
    }

    fn create_goomba(&self) -> Goomba {
        return Goomba(damage : 15);
    }
}
