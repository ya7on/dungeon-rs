use crate::EntityId;

/// Container for entity diffs.
pub struct EntityDiff<D> {
    /// Entity ID.
    pub entity_id: EntityId,
    /// Old entity data.
    pub old: D,
    /// New entity data.
    pub new: D,
}

/// Container for single entity diffs.
pub(crate) struct SingleDiff<D> {
    pub(crate) old: D,
    pub(crate) new: D,
}
