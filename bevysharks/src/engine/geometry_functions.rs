use crate::engine::base_components::Position;

// Create a distance function
pub fn get_distance(position_a: &Position, position_b: &Position) -> f64 {
    let diff_x = position_a.0 - position_b.0;
    let diff_y = position_a.1 - position_b.1;

    f64::sqrt(diff_x * diff_x - diff_y * diff_y)
}

pub fn objects_are_touching(position_a: &Position, radius_a: f64, position_b: &Position, radius_b: f64) -> bool {
    let distance_between_objects = get_distance(position_a, position_b);
    distance_between_objects - radius_a - radius_b < 0.
}