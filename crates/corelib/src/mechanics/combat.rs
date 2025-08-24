use crate::{Actor, rng::MyRng};

pub(crate) fn try_attack(
    attacker: &mut Actor,
    target: &mut Actor,
    rng: &mut MyRng,
) -> u32 {
    let physical_damage = rng
        .range(attacker.stats().min_damage()..=attacker.stats().max_damage());
    let physical_defense = target.stats().defense();
    let total_damage =
        (physical_damage.saturating_sub(physical_defense)).max(1);

    target.stats.hp = target.stats.hp.saturating_sub(total_damage);
    // TODO: Implement critical hit logic
    total_damage
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{actors::{ActorKind}, position::Position};

    #[test]
    fn damage_respects_defense() {
        let mut attacker = Actor::create(Position::new(0, 0), ActorKind::Enemy);
        attacker.stats.min_damage = 5;
        attacker.stats.max_damage = 5;
        let mut target = Actor::create(Position::new(1, 0), ActorKind::Enemy);
        target.stats.defense = 1;
        let mut rng = MyRng::from_seed([1; 32]);
        let dmg = try_attack(&mut attacker, &mut target, &mut rng);
        assert_eq!(dmg, 4);
        assert_eq!(target.stats.hp, 16);
    }

    #[test]
    fn minimum_damage_is_one() {
        let mut attacker = Actor::create(Position::new(0, 0), ActorKind::Enemy);
        attacker.stats.min_damage = 3;
        attacker.stats.max_damage = 3;
        let mut target = Actor::create(Position::new(1, 0), ActorKind::Enemy);
        target.stats.defense = 10;
        let mut rng = MyRng::from_seed([2; 32]);
        let dmg = try_attack(&mut attacker, &mut target, &mut rng);
        assert_eq!(dmg, 1);
        assert_eq!(target.stats.hp, 19);
    }

    #[test]
    fn deterministic_for_same_seed() {
        let mut a1 = Actor::create(Position::new(0, 0), ActorKind::Enemy);
        let mut t1 = Actor::create(Position::new(1, 0), ActorKind::Enemy);
        let mut a2 = Actor::create(Position::new(0, 0), ActorKind::Enemy);
        let mut t2 = Actor::create(Position::new(1, 0), ActorKind::Enemy);
        let mut rng1 = MyRng::from_seed([3; 32]);
        let mut rng2 = MyRng::from_seed([3; 32]);
        let d1 = try_attack(&mut a1, &mut t1, &mut rng1);
        let d2 = try_attack(&mut a2, &mut t2, &mut rng2);
        assert_eq!(d1, d2);
    }
}
