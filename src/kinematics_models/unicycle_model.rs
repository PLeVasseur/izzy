/// unicycle model state
struct UnicycleModelState {
    /// state evolving with no process noise
    ground_truth: ModelState,
    /// state evolving with process noise applied
    noisy_state:  ModelState,
    /// seed to random number generator to get consistent results
    rng_seed:     u64
}

/// unicycle model pose and velocity
struct ModelState {
    /// x position in 2D plane
    x:     f64,
    /// y position in 2D plane
    y:     f64,
    /// orientation in 2D plane (radians)
    theta: f64,
    /// longitudinal velocity of vehicle in vehicle frame
    a_dot: f64,
    /// lateral velocity of vehicle in vehicle frame
    b_dot: f64,
    /// angular velocity of vehicle in vehicle frame (rad/s)
    omega: f64
}
