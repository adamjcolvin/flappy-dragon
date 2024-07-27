use bevy::prelude::*;

//How frequently should the physics tick firs (ms)
const PHYSICS_TICK_TIME: u128 = 33;

#[derive(Default)]
pub struct PhysicsTimer(u128);

#[derive(Event)]
pub struct PhysicsTick;

pub fn physics_clock(
    mut clock: Local<PhysicsTimer>,
    time: Res<Time>,
    mut on_tick: EventWriter<PhysicsTick>,
    mut physics_position: Query<(&mut PhysicsPosition, &mut Transform)>,
) {
    let ms_since_last_call = time.delta().as_millis();
    clock.0 += ms_since_last_call;
    if clock.0 > PHYSICS_TICK_TIME {
        clock.0 = 0;
        physics_position.for_each_mut(|(mut pos, mut transform)| {
            transform.translation.x = pos.end_frame.x;
            transform.translation.y = pos.end_frame.y;
            pos.start_frame = pos.end_frame
        });
        on_tick.send(PhysicsTick);
    } else {
        let frame_progress = clock.0 as f32 / PHYSICS_TICK_TIME as f32;
        physics_position.for_each_mut(|(pos, mut transform)| {
            transform.translation.x =
                pos.start_frame.x + (pos.end_frame.x - pos.start_frame.x) * frame_progress;
            transform.translation.y =
                pos.start_frame.y + (pos.end_frame.y - pos.start_frame.y) * frame_progress;
        })
    }
}

#[derive(Component)]
pub struct Velocity(Vec3);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3 { x, y, z })
    }
}

#[derive(Event)]
pub struct Impulse {
    pub target: Entity,
    pub amount: Vec3,
    pub absolute: bool,
}

pub fn sum_impulses(mut impulses: EventReader<Impulse>, mut velocities: Query<&mut Velocity>) {
    for impulse in impulses.read() {
        if let Ok(mut velocity) = velocities.get_mut(impulse.target) {
            if impulse.absolute {
                velocity.0 = impulse.amount;
                return;
            } else {
                velocity.0 += impulse.amount;
            }
        }
    }
}

pub fn apply_velocity(
    mut tick: EventReader<PhysicsTick>,
    mut movement: Query<(&Velocity, &mut PhysicsPosition)>,
) {
    for _tick in tick.read() {
        movement.for_each_mut(|(velocity, mut position)| {
            position.end_frame += velocity.0.truncate();
        });
    }
}

#[derive(Component)]
pub struct ApplyGravity;

pub fn apply_gravity(
    mut tick: EventReader<PhysicsTick>,
    mut gravity: Query<&mut Velocity, With<ApplyGravity>>,
) {
    for _tick in tick.read() {
        gravity.for_each_mut(|mut velocity| {
            velocity.0.y -= 0.75;
        })
    }
}

#[derive(Component)]
pub struct PhysicsPosition {
    pub start_frame: Vec2,
    pub end_frame: Vec2,
}

impl PhysicsPosition {
    pub fn new(start: Vec2) -> Self {
        Self {
            start_frame: start,
            end_frame: start,
        }
    }
}
