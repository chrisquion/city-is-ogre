#[derive(Eq, PartialEq, Debug, Clone)]
pub enum AiAction {
    /// There's a lot going behind the scenes
    ZoneOut,

    /// Wander aimlessly
    Wander,

    /// Navigate to the destination for the given reason
    GottaGoTo {
        target: WorldPoint,
        reason: &'static str,
    },

    /// Go pickup the (1) best item
    GottaGoGet(ItemsToPickUp),

    /// Equip and eat the given entity, assuming it's already in the inventory
    EatHeldItem(Entity),

    /// Go break the given block
    GoChatWith(&Character),

    /*
    /// Follow the entity, keeping to the given distance
    Follow { target: Entity, radius: u8 },

    /// Haul the entity from the source to the destination target
    Haul(Entity, HaulTarget, HaulTarget),
    */
}


impl Default for AiAction {
    fn default() -> Self {
        AiAction::ZoneOut
    }
}