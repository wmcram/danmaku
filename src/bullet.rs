use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct Velocity(f32);

#[derive(Component)]
pub struct Acceleration(f32);

#[derive(Component)]
pub struct AngularVelocity(f32);

#[derive(Component)]
pub struct AngularAcceleration(f32);

enum BulletActionType {
    SetVelocity(f32),
    SetAcceleration(f32),
    SetAngularVelocity(f32),
    SetAngularAcceleration(f32),
    ChangeVelocity(f32),
    ChangeAcceleration(f32),
    ChangeAngularVelocity(f32),
    ChangeAngularAcceleration(f32),
    Rotate(f32),
    Move(Vec3),
    Despawn,
}

pub struct BulletAction(BulletActionType, f32);

#[derive(Component)]
pub struct BulletActions {
    actions: Vec<BulletAction>,
    action_index: usize,
    repeat: bool,
}

#[derive(Bundle)]
struct BulletBundle {
    sprite: SpriteBundle,
    velocity: Velocity,
    acceleration: Acceleration,
    angular_velocity: AngularVelocity,
    angular_acceleration: AngularAcceleration,
    actions: BulletActions,
    marker: Bullet,
}

pub fn spawn_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    transform.rotate_local_z(PI);
    commands.spawn(BulletBundle {
        sprite: SpriteBundle { 
            texture: asset_server.load("sprites/bullet.png"),
            transform,
            ..default()
        },
        velocity: Velocity(100.0),
        acceleration: Acceleration(0.0),
        angular_velocity: AngularVelocity(0.0),
        angular_acceleration: AngularAcceleration(0.0),
        actions: BulletActions {
            actions: vec![
                BulletAction(BulletActionType::Rotate(90.0), 1.0),
            ],
            action_index: 0,
            repeat: true,
        },
        marker: Bullet,
    });
}

pub fn move_bullets(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &Acceleration, &mut AngularVelocity, &AngularAcceleration), With<Bullet>>
) {
    for (mut transform, mut velocity, acceleration, mut angular_velocity, angular_acceleration) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds(); //Accelerate position
        angular_velocity.0 += angular_acceleration.0 * time.delta_seconds(); //Accelerate rotation
        transform.rotate_local_z(angular_velocity.0 * time.delta_seconds()); //Rotate
        let move_vec: Vec3 = transform.up(); //Get forward vector
        transform.translation += move_vec * velocity.0 * time.delta_seconds(); //Move
    }
}

pub fn bullet_actions(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Velocity, &mut Acceleration, &mut AngularVelocity, &mut AngularAcceleration, &mut Transform, &mut BulletActions), With<Bullet>>
) {
    for (entity, mut velocity, mut acceleration, mut angular_velocity, mut angular_acceleration, mut transform, mut actions) in query.iter_mut() {
        let idx = actions.action_index;
        actions.actions[idx].1 -= time.delta_seconds();
        if actions.actions[idx].1 < 0.0 {
            match actions.actions[idx].0 {
                BulletActionType::SetVelocity(v) => velocity.0 = v,
                BulletActionType::SetAcceleration(a) => acceleration.0 = a,
                BulletActionType::SetAngularVelocity(v) => angular_velocity.0 = v,
                BulletActionType::SetAngularAcceleration(a) => angular_acceleration.0 = a,
                BulletActionType::ChangeVelocity(v) => velocity.0 += v,
                BulletActionType::ChangeAcceleration(a) => acceleration.0 += a,
                BulletActionType::ChangeAngularVelocity(v) => angular_velocity.0 += v,
                BulletActionType::ChangeAngularAcceleration(a) => angular_acceleration.0 += a,
                BulletActionType::Rotate(r) => transform.rotate_local_z(r),
                BulletActionType::Move(m) => transform.translation += m,
                BulletActionType::Despawn => commands.entity(entity).despawn(),
            }
            actions.action_index += 1;
            if actions.action_index == actions.actions.len() {
                if actions.repeat {
                    actions.action_index = 0;
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}