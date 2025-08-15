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
