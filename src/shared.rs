use bevy::prelude::*;

/// Markiert einen Landeplatz auf einem Dach.
#[derive(Component)]
pub struct LandingSpot {
    pub id: u32,
}

/// Ein Dorfbewohner. `home` ist sein Landing Spot.
#[derive(Component)]
pub struct Villager {
    pub name: String,
    pub home: Entity,
}

/// Ein Lieferauftrag von einem Landing Spot zu einem anderen.
#[derive(Clone, Debug)]
pub struct Order {
    pub from: Entity,
    pub to: Entity,
    pub weight: f32,
    pub fragile: bool,
    pub time_limit: f32,
}

/// Wird gesendet, sobald ein neuer Auftrag entsteht.
#[derive(Message)]
pub struct OrderCreated(pub Order);