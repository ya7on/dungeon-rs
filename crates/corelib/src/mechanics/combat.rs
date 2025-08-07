use crate::Actor;

pub(crate) fn try_attack(attacker: &mut Actor, target: &mut Actor) -> u32 {
    let damage = (attacker.stats().attack() - target.stats().defense()).max(1);
    target.stats.hp = target.stats.hp.saturating_sub(damage);
    // TODO: Implement critical hit logic
    damage
}
