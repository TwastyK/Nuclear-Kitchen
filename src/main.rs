use bevy::prelude::*;

#[derive(Component, PartialEq)]
enum food_states {
    Raw,
    Cooked,
    Overcooked,
}

#[derive(Resource)]
struct Score {
    score: i8,
}

#[derive(Component)]
struct Customer {
    order: FoodType,
}
#[derive(Component, PartialEq, Clone, Copy)]
enum FoodType {
    Steak,
}

#[derive(Component)]
struct Player {
    speed: f32,
    taked_food: Option<FoodType>,
}

#[derive(Component)]
struct Oven;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        //.add_systems(Update, (cook_meat, move_player, check_oven, take_food, delivery_food))
        .insert_resource(Score { score: 0 })
        .run();
}
#[derive(Component)]
struct FoodTimer(f32);

const OVEN_SIZE: Vec2 = Vec2::new(500.0, 100.0);
const CUSTOMER_SIZE: Vec2 = Vec2::new(75.0, 75.0);

fn in_bounds(pos: Vec2, center: Vec2, size: Vec2) -> bool {
    pos.x > center.x - size.x / 2.0 && pos.x < center.x + size.x / 2.0
        && pos.y > center.y - size.y / 2.0 && pos.y < center.y + size.y / 2.0
}

fn setup(mut commands: Commands,
         mut materials: ResMut<Assets<StandardMaterial>>,
         mut meshes: ResMut<Assets<Mesh>>) {



    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 5.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, -5.0),
        Visibility::Visible,
    ));
}

fn cook_meat(
    time: Res<Time>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut FoodTimer, &mut food_states, &mut Sprite), With<FoodTimer>>,
    player: Query<&Transform, With<Player>>,
    oven: Query<&Transform, With<Oven>>,
) {
    for transform in oven {
        for player in player {
            if in_bounds(player.translation.truncate(), transform.translation.truncate(), OVEN_SIZE)
            {
                if keys.just_pressed(KeyCode::Space) {
                    commands.spawn((
                        Sprite {
                            color: Color::srgb_u8(255, 192, 203),
                            custom_size: Some(Vec2::new(100.0, 100.0)),
                            ..default()
                        },
                        Transform::from_translation(player.translation),
                        FoodTimer(0.0),
                        FoodType::Steak,
                        food_states::Raw,
                    ));
                }
            }
        }
        for (mut timer, mut states, mut sprite) in &mut query {
            timer.0 += time.delta_secs();
            match *states {
                food_states::Raw => {
                    if timer.0 >= 5.0 {
                        *states = food_states::Cooked;
                        sprite.color = Color::srgb(0.4, 0.2, 0.0);
                        println!("Cooked food");
                    }
                }
                food_states::Cooked => {

                }
                food_states::Overcooked => {}
            }
        }
    }
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    for (mut transform, player) in &mut query {
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.y += player.speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation.y -= player.speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += player.speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= player.speed * time.delta_secs();
        }
    }
}

fn check_oven(player: Query<&Transform, With<Player>>, oven: Query<&Transform, With<Oven>>) {
    for transform in oven {
        for player in player {
            if in_bounds(player.translation.truncate(), transform.translation.truncate(), OVEN_SIZE) {

            }
        }
    }
}

fn take_food(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>,
             food: Query<(&Transform, &food_states, &FoodType, Entity)>,
             mut player: Query<(&Transform, &mut Player)>) {
                for (food_transform, food_states, food, entity) in &food {
                    if *food_states == food_states::Cooked {
                        for mut transform in &mut player {
                            if in_bounds(transform.0.translation.truncate(), food_transform.translation.truncate(), CUSTOMER_SIZE){
                                    if keys.just_pressed(KeyCode::KeyE) {
                                        commands.entity(entity).despawn();
                                            transform.1.taked_food = Some(*food);
                                    }
                            }
                        }
                    }
                }
}

fn delivery_food(customer: Query<(&Transform, &Customer, Entity)>,
                 mut score: ResMut<Score>,
                 mut player: Query<(&Transform, &mut Player)>,
                 mut commands: Commands,
                 keys: Res<ButtonInput<KeyCode>>) {
    for (transform, customer, entity) in customer {
        for mut player in &mut player {
            if in_bounds(player.0.translation.truncate(), transform.translation.truncate(), CUSTOMER_SIZE) {
                if player.1.taked_food == Some(customer.order) {
                    if keys.just_pressed(KeyCode::KeyE) {
                        score.score += 1;
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
        }
    }
