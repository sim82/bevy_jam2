// tunables
pub const LINEAR_DAMPING: f32 = 0.7;
pub const BALL_LINEAR_DAMPING: f32 = 1.0;
pub const FRICTION: f32 = 0.5;
pub const BALL_FRICTION: f32 = 0.3;
pub const RESTITUTION: f32 = 0.3;
pub const BALL_RESTITUTION: f32 = 2.0;

pub const WALK_IMPULSE_GROUND: f32 = 0.3;
pub const WALK_IMPULSE_AIR: f32 = 0.05;
pub const JUMP_IMPULSE: f32 = 4.0;

pub const JUMP_IMPULSE_BALL: f32 = 4.0;

pub const _ROT_IMPULSE: f32 = 0.00005;

pub const JUMP_TIMEOUT: f32 = 0.3;
pub const LETHAL_VELOCITY: f32 = -150.0;

pub const WALKING: bool = true;

pub const MAX_WALK_VEL: f32 = 90.0;

pub const FERRIS_Z: f32 = 4.0;
pub const BUBBLE_Z: f32 = 8.0; // WTF: why is ferris at z 7.0?
