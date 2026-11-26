use bevy::prelude::*;

use crate::shared::*;

pub struct CityPlugin;

impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<OrderCreated>()
            .insert_resource(OrderTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .insert_resource(SimpleRng(12345))
            .add_systems(Startup, (spawn_debug_scene, spawn_city))
            .add_systems(Update, create_orders);
    }
}

#[derive(Resource)]
struct OrderTimer(Timer);

/// Ein winziger Zufallsgenerator, damit wir kein extra Paket brauchen.
#[derive(Resource)]
struct SimpleRng(u64);

impl SimpleRng {
    fn next_range(&mut self, max: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 33) as usize) % max
    }
}

/// Kamera, Licht und Boden. Die Kamera gehört später der Kamera-Person.
fn spawn_debug_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 25.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(40.0, 1.0, 40.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.7, 0.4))),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
}

/// Häuser (Würfel), Landing Spots (gelbe Scheiben) und Villager (Kapseln).
fn spawn_city(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let house_mesh = meshes.add(Cuboid::new(4.0, 3.0, 4.0));
    let house_mat = materials.add(Color::srgb(0.9, 0.6, 0.4));
    let spot_mesh = meshes.add(Cylinder::new(1.2, 0.2));
    let spot_mat = materials.add(Color::srgb(1.0, 0.85, 0.2));
    let villager_mesh = meshes.add(Capsule3d::new(0.4, 1.0));
    let villager_mat = materials.add(Color::srgb(0.3, 0.4, 0.9));

    let positions = [(-12.0, -8.0), (-4.0, 6.0), (8.0, -6.0), (14.0, 8.0), (0.0, -14.0)];
    let names = ["Anna", "Ben", "Clara", "David", "Emma"];

    for (i, (x, z)) in positions.iter().enumerate() {
        commands.spawn((
            Name::new(format!("House_{i}")),
            Mesh3d(house_mesh.clone()),
            MeshMaterial3d(house_mat.clone()),
            Transform::from_xyz(*x, 1.5, *z),
        ));

        let spot = commands
            .spawn((
                Name::new(format!("LandingSpot_{i}")),
                LandingSpot { id: i as u32 },
                Mesh3d(spot_mesh.clone()),
                MeshMaterial3d(spot_mat.clone()),
                Transform::from_xyz(*x, 3.1, *z),
            ))
            .id();

        commands.spawn((
            Name::new(names[i]),
            Villager {
                name: names[i].to_string(),
                home: spot,
            },
            Mesh3d(villager_mesh.clone()),
            MeshMaterial3d(villager_mat.clone()),
            Transform::from_xyz(*x + 3.0, 0.9, *z),
        ));
    }
}

/// Alle 5 Sekunden entsteht ein zufälliger Auftrag.
fn create_orders(
    time: Res<Time>,
    mut timer: ResMut<OrderTimer>,
    mut rng: ResMut<SimpleRng>,
    villagers: Query<&Villager>,
    mut writer: MessageWriter<OrderCreated>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let list: Vec<&Villager> = villagers.iter().collect();
    if list.len() < 2 {
        return;
    }
    let a = rng.next_range(list.len());
    let mut b = rng.next_range(list.len());
    while b == a {
        b = rng.next_range(list.len());
    }
    let order = Order {
        from: list[a].home,
        to: list[b].home,
        weight: 1.0 + rng.next_range(10) as f32,
        fragile: rng.next_range(4) == 0,
        time_limit: 60.0,
    };
    info!(
        "Neuer Auftrag: {} -> {} ({} kg, zerbrechlich: {}, {} s)",
        list[a].name, list[b].name, order.weight, order.fragile, order.time_limit
    );
    writer.write(OrderCreated(order));
}