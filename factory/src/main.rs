mod Enemy_Factory;
use Enemy_Factory::EnemyFactory::Factory;

fn main() {
    let factory = Factory{};
    let koopa = factory.create_koopa();
    koopa.attack();
}
