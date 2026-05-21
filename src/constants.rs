/// Speed of light, m/s
pub const C: f32 = 299_792_458.0;

/// Schwarzschild radius scale used in main (meters)
pub const BLACK_HOLE_RADIUS: f32 = 6.732e12;

/// Mass of Sgr A* scale (kg)
pub const BLACK_HOLE_MASS: f32 = 4.297e6 * 1.98847e30;

/// Half-width of the visible world region in multiples of Rs
pub const VIEW_RADIUS_MULT: f32 = 14.0;

/// Simulation timestep (seconds per engine step)
pub const SIM_DT: f32 = 600.0;
