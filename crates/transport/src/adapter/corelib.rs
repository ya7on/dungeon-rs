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
            _ => unimplemented!(),
        }
    }
}
