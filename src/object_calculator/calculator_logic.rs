pub fn calculate_metrics(total_dist: f64, thickness: f64, number_of_elements: i32) -> (f64, f64) {
    if number_of_elements <= 1 {
        return (0.0, 0.0);
    }

    // Calculate edge-to-edge/gap distance
    let used_space = thickness * (number_of_elements as f64);
    let remaining_space = total_dist - used_space;
    let gap_distance = remaining_space / ((number_of_elements - 1) as f64) + thickness;

    // Calculate center-to-center distance
    let center_distance = total_dist / ((number_of_elements - 1) as f64);

    (gap_distance, center_distance)
}