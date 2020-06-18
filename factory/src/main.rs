mod Enemy_Factory;

fn main() {
    let factory = EnemyFactory{};
    let koopa = factory.create_koopa();
    koopa.attack();
}
