/// Trait for casting DTO protocol types into corelib native types
pub trait ToCorelib<T> {
    /// Convert type to `corelib` types
    fn to_corelib(self) -> T;
}

impl ToCorelib<corelib::PlayerAction> for protocol::PlayerAction {
    fn to_corelib(self) -> corelib::PlayerAction {
        match self {
            Self::Move(direction) => {
                corelib::PlayerAction::Move(direction.to_corelib())
            },
        }
    }
}

impl ToCorelib<corelib::Direction> for protocol::Direction {
    fn to_corelib(self) -> corelib::Direction {
        match self {
            Self::North => corelib::Direction::North,
            Self::South => corelib::Direction::South,
            Self::East => corelib::Direction::East,
            Self::West => corelib::Direction::West,
        }
    }
}

/// Trait for casting corelib native types into DTO protocol types
pub trait FromCorelib<T> {
    /// Convert type from `corelib` types
    fn from_corelib(from: T) -> Self;
}

impl FromCorelib<corelib::GameEvent> for protocol::GameEvent {
    fn from_corelib(from: corelib::GameEvent) -> Self {
        match from {
            corelib::GameEvent::PlayerSkippedMove => Self::PlayerSkippedMove,
            corelib::GameEvent::PlayerDied => Self::PlayerDied,
            corelib::GameEvent::PlayerMoved { from, to } => Self::PlayerMoved {
                from: protocol::Position::from_corelib(from),
                to: protocol::Position::from_corelib(to),
            },
            corelib::GameEvent::PlayerBumped { position, direction } => {
                Self::PlayerBumped {
                    position: protocol::Position::from_corelib(position),
                    direction: protocol::Direction::from_corelib(direction),
                }
            },
            corelib::GameEvent::PlayerAttacked { target, damage } => {
                Self::PlayerAttacked { target: target.into(), damage }
            },
            corelib::GameEvent::PlayerAttackMissed => Self::PlayerAttackMissed,
            corelib::GameEvent::PlayerEquippedItem { item_id, slot } => {
                Self::PlayerEquippedItem { item_id, slot }
            },
            corelib::GameEvent::PlayerUnequippedItem { slot } => {
                Self::PlayerUnequippedItem { slot }
            },
            corelib::GameEvent::EntityMoved { id, from, to } => {
                Self::EntityMoved {
                    id: id.into(),
                    from: protocol::Position::from_corelib(from),
                    to: protocol::Position::from_corelib(to),
                }
            },
            corelib::GameEvent::EntityAttacked { id, target, damage } => {
                Self::EntityAttacked {
                    id: id.into(),
                    target: protocol::Position::from_corelib(target),
                    damage,
                }
            },
            corelib::GameEvent::EffectTick { entity_id, effect_id } => {
                Self::EffectTick { entity_id: entity_id.into(), effect_id }
            },
            corelib::GameEvent::EffectExpired { entity_id, effect_id } => {
                Self::EffectExpired { entity_id: entity_id.into(), effect_id }
            },
        }
    }
}

impl FromCorelib<corelib::Position> for protocol::Position {
    fn from_corelib(from: corelib::Position) -> Self {
        Self { x: from.x, y: from.y }
    }
}

impl FromCorelib<corelib::Direction> for protocol::Direction {
    fn from_corelib(from: corelib::Direction) -> Self {
        match from {
            corelib::Direction::North => Self::North,
            corelib::Direction::South => Self::South,
            corelib::Direction::East => Self::East,
            corelib::Direction::West => Self::West,
        }
    }
}

impl FromCorelib<corelib::EntityId> for protocol::EntityId {
    fn from_corelib(from: corelib::EntityId) -> Self {
        Self(from.into_inner())
    }
}

impl<T, P> FromCorelib<corelib::EntityDiff<P>> for protocol::EntityDiff<T>
where
    T: FromCorelib<P>,
{
    fn from_corelib(from: corelib::EntityDiff<P>) -> Self {
        Self {
            entity_id: protocol::EntityId::from_corelib(from.entity_id),
            new: T::from_corelib(from.new),
            old: T::from_corelib(from.old),
        }
    }
}

impl FromCorelib<corelib::StateDiff> for protocol::StateDiff {
    fn from_corelib(from: corelib::StateDiff) -> Self {
        Self {
            positions: from
                .positions
                .into_iter()
                .map(protocol::EntityDiff::from_corelib)
                .collect(),
        }
    }
}
