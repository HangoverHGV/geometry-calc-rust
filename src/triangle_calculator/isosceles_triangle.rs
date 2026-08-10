pub fn calculate_all_isosceles_elements(
    ab_is: f64,
    bc_is: f64,
    abc_is: f64,
    bac_is: f64
) -> Result<Option<(f64, f64, f64, f64)>, String> { 

    let mut ab = ab_is;
    let mut bc = bc_is;
    let mut abc = abc_is;
    let mut bac = bac_is;
    let eps = 1e-6;

    if ab > eps && bc > eps {
        if bc >= 2.0 * ab {
            return Err("Impossible triangle: Base must be smaller than the sum of the two legs".into());
        }
        let cos_abc = (bc / (2.0 * ab)).clamp(-1.0, 1.0);
        abc = cos_abc.acos().to_degrees();
        bac = 180.0 - (2.0 * abc);
    }
    else if ab > eps && bac > eps {
        if bac >= 180.0 { return Err("Vertex angle must be less than 180 degrees".into()); }
        let bac_rad = bac.to_radians();
        bc = 2.0 * ab * (bac_rad / 2.0).sin();
        abc = (180.0 - bac) / 2.0;
    }
    else if ab > eps && abc > eps {
        if abc >= 90.0 { return Err("Base angle must be strictly less than 90 degrees".into()); }
        let abc_rad = abc.to_radians();
        bc = 2.0 * ab * abc_rad.cos();
        bac = 180.0 - (2.0 * abc);
    }
    else if bc > eps && bac > eps {
        if bac >= 180.0 { return Err("Vertex angle must be less than 180 degrees".into()); }
        let bac_rad = bac.to_radians();
        ab = bc / (2.0 * (bac_rad / 2.0).sin());
        abc = (180.0 - bac) / 2.0;
    }
    else if bc > eps && abc > eps {
        if abc >= 90.0 { return Err("Base angle must be strictly less than 90 degrees".into()); }
        let abc_rad = abc.to_radians();
        ab = bc / (2.0 * abc_rad.cos());
        bac = 180.0 - (2.0 * abc);
    }
    else {
        // The user is still typing! Return Ok(None) to silently wait.
        return Ok(None);
    }

    let sum_of_angles = abc + abc + bac;
    if (sum_of_angles - 180.0).abs() > 0.001 {
        return Err("Invalid geometry: Computed angles do not sum to 180 degrees".into());
    }

    // Return the successful calculation wrapped in Some
    Ok(Some((ab, bc, abc, bac)))
}