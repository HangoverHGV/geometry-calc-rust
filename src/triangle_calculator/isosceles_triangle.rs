pub fn calculate_all_isosceles_elements(ab_is: f64, bc_is: f64, abc_is: f64, bac_is: f64)
                                        -> Option<(f64, f64, f64, f64)>
{
    let mut ab = ab_is;
    let mut bc = bc_is;
    let mut abc = abc_is;
    let mut bac = bac_is;

    // 1. Check if the angles make physical sense for an isosceles triangle
    let sum_of_angles = abc + abc + bac; // base_angle * 2 + vertex_angle
    if (sum_of_angles - 180.0).abs() > 0.001 {
        return None; // Invalid: Angles do not sum to 180 degrees
    }

    // 2. Ensure angles are within realistic bounds
    if abc <= 0.0 || abc >= 90.0 || bac <= 0.0 || bac >= 180.0 {
        return None; // Invalid: Geometric singularity
    }

    // Base Angle constraint
    if abc >= 90.0 {
        return None; // Impossible for a triangle
    }

    // Vertex Angle constraint
    if bac >= 180.0 || bac <= 0.0 {
        return None; // Impossible for a triangle
    }


    // Use a small epsilon for float comparison to avoid issues with 0.0
    let eps = 1e-6;

    if ab > eps && bc > eps {
        let cos_abc = (bc / (2.0 * ab)).clamp(-1.0, 1.0);
        abc = cos_abc.acos().to_degrees();
        bac = 180.0 - (2.0 * abc);
    }
    else if ab > eps && bac > eps && bac < 180.0 {
        let bac_rad = bac.to_radians();
        bc = 2.0 * ab * (bac_rad / 2.0).sin();
        abc = (180.0 - bac) / 2.0;
    }
    else if ab > eps && abc > eps && abc < 90.0 {
        let abc_rad = abc.to_radians();
        bc = 2.0 * ab * abc_rad.cos();
        bac = 180.0 - (2.0 * abc);
    }
    else if bc > eps && bac > eps && bac < 180.0 {
        let bac_rad = bac.to_radians();
        ab = bc / (2.0 * (bac_rad / 2.0).sin());
        abc = (180.0 - bac) / 2.0;
    }
    else if bc > eps && abc > eps && abc < 90.0 {
        let abc_rad = abc.to_radians();
        ab = bc / (2.0 * abc_rad.cos());
        bac = 180.0 - (2.0 * abc);
    }
    else {
        return None; // Input insufficient or invalid
    }


    // Final sanity check: Triangle inequality
    if ab <= (bc / 2.0) + eps {
        return None;
    }

    Some((ab, bc, abc, bac))
}