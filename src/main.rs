use bevy::{prelude::*, window::PresentMode, sprite::collide_aabb::collide};
fn main() {
    App::new().
        add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Danmaku Dynasty".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        })).
        add_systems(Startup, setup).
        add_systems(Update, (
            player_shoot, 
            player_movement, 
            move_bullets, 
            collision_system::<PlayerProjectile, Enemy>,
            collision_system::<EnemyProjectile, Player>,
        )).
        run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Weapon {
    fire_rate: f32,
    last_fired: f32,
}

#[derive(Component)]
struct Bullet {
    direction: Vec3,
    speed: f32,
    angular_velocity: f32,
    lifetime: f32,
}

#[derive(Component)]
struct PlayerProjectile;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct EnemyProjectile;

#[derive(Component)]
struct BulletSpawner {
    
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player,
        Health(3),
        SpriteBundle {
            texture: asset_server.load("player.png"),
            ..Default::default()
        },
        Weapon {
            fire_rate: 0.2,
            last_fired: 0.0,
        },
    ));
    commands.spawn((
        Enemy,
        Health(10),
        SpriteBundle {
            texture: asset_server.load("enemy.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..default()
        },
    ));
}

fn player_movement(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    const SPEED: f32 = 250.0;
    let mut player = query.single_mut();
    let mut velocity = Vec3::new(0.0, 0.0, 0.0);
    if keys.pressed(KeyCode::Left) {
        velocity.x -= 1.0;
    }
    if keys.pressed(KeyCode::Right) {
        velocity.x += 1.0;
    }
    if keys.pressed(KeyCode::Up) {
        velocity.y += 1.0;
    }
    if keys.pressed(KeyCode::Down) {
        velocity.y -= 1.0;
    }
    if velocity.length() > 0.0 {
        velocity = velocity.normalize();
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        velocity /= 3.0;
    }
    player.translation += velocity * time.delta_seconds() * SPEED;
}

fn player_shoot(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Weapon, &Transform), With<Player>>, time: Res<Time>, mut commands: Commands, asset_server: Res<AssetServer>) {
    let (mut weapon, transform) = query.single_mut();
    if keys.pressed(KeyCode::X) && weapon.last_fired >= weapon.fire_rate {
        weapon.last_fired = 0.0;
        commands.spawn((
            PlayerProjectile,
            Bullet {
                direction: Vec3::new(0.0, 1.0, 0.0),
                speed: 500.0,
                angular_velocity: 0.0,
                lifetime: 3.0,
            },
            SpriteBundle {
                texture: asset_server.load("bullet.png"),
                transform: transform.clone(),
                ..Default::default()
            }
        ));
    }
    weapon.last_fired += time.delta_seconds();
}

fn move_bullets(mut query: Query<(Entity, &mut Transform, &mut Bullet)>, time: Res<Time>, mut commands: Commands) {
    for (entity, mut transform, mut bullet) in query.iter_mut() {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
        transform.rotate(Quat::from_rotation_z(bullet.angular_velocity * time.delta_seconds()));
        bullet.lifetime -= time.delta_seconds();
        if bullet.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn collision_system<B: Component, C: Component>(mut commands: Commands, mut bullet_query: Query<(Entity, &Transform), With<B>>, mut enemy_query: Query<(Entity, &Transform, &mut Health), With<C>>) {
    for (bullet_entity, bullet_transform) in bullet_query.iter_mut() {
        for (enemy_entity, enemy_transform, mut health) in enemy_query.iter_mut() {
            if collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate(),
                enemy_transform.translation,
                enemy_transform.scale.truncate(),
            ).is_some() {
                commands.entity(bullet_entity).despawn();
                health.0 -= 1;
                if health.0 <= 0 {
                    commands.entity(enemy_entity).despawn();
                }
            }
        }
    }
}

