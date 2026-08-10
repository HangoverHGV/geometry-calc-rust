pub fn try_calculate(total_str: &str, thickness_str: &str, count_str: &str) -> Result<(f64, f64), String>{
    let total_dist: f64 = total_str.parse().unwrap_or(0.0);
    let thickness: f64 = thickness_str.parse().unwrap_or(0.0);
    let number_of_elements: i32 = count_str.parse().unwrap_or(0);

    if thickness > total_dist {
        return Err("Object thickness cannot be bigger than total distance!".into());
    }

    let total_thickness = thickness * (number_of_elements as f64);
    if total_thickness > total_dist {
        return Err("Total thickness of all elements exceeds the total distance!".into());
    }

    Ok(calculate_metrics(total_dist, thickness, number_of_elements))
}

fn calculate_metrics(total_dist: f64, thickness: f64, number_of_elements: i32) -> (f64, f64) {
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