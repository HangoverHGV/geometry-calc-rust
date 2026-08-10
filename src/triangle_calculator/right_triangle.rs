pub fn calculate_all_triangle_elements(
    hyp: f64,
    cat1: f64,
    cat2: f64,
    angle_abc: f64,
    angle_acb: f64
) -> Result<Option<(f64, f64, f64, f64, f64)>, String> {

    let mut h = hyp;
    let mut c1 = cat1;
    let mut c2 = cat2;
    let mut a_abc = angle_abc;
    let mut a_acb = angle_acb;

    let eps = 1e-6; // Small epsilon to handle floating point zeros

    // If everything is blank, return empty results smoothly instead of crashing
    if h <= eps && c1 <= eps && c2 <= eps && a_abc <= eps && a_acb <= eps {
        return Ok(None);
    }

    // --- PRE-CALCULATION VALIDATION ---
    if a_abc >= 90.0 || a_acb >= 90.0 {
        return Err("In a right triangle, non-right angles must be strictly less than 90 degrees.".into());
    }

    if a_abc > eps && a_acb > eps && (a_abc + a_acb - 90.0).abs() > 0.01 {
        return Err("The two non-right angles must sum exactly to 90 degrees.".into());
    }

    if (h > eps && c1 > eps && c1 >= h) || (h > eps && c2 > eps && c2 >= h) {
        return Err("The hypotenuse must be strictly greater than either cathetus.".into());
    }
    // ----------------------------------

    // Step 1: Immediately resolve complementary angles if one is given
    if a_abc > eps && a_abc < 90.0 && a_acb <= eps { a_acb = 90.0 - a_abc; }
    if a_acb > eps && a_acb < 90.0 && a_abc <= eps { a_abc = 90.0 - a_acb; }

    // Step 2: Solve sides based on what combination of inputs is present
    // Case A: Two sides are known (Pythagorean Theorem)
    if c1 > eps && c2 > eps && h <= eps {
        h = (c1.powi(2) + c2.powi(2)).sqrt();
    }
    else if h > eps && c1 > eps && c2 <= eps {
        // Pre-validation caught c1 >= h, so it's safe to subtract
        c2 = (h.powi(2) - c1.powi(2)).sqrt();
    }
    else if h > eps && c2 > eps && c1 <= eps {
        // Pre-validation caught c2 >= h, so it's safe to subtract
        c1 = (h.powi(2) - c2.powi(2)).sqrt();
    }
    // Case B: One side and an angle are known (Trigonometry)
    else if h > eps && a_abc > eps && c1 <= eps && c2 <= eps {
        let rad = a_abc.to_radians();
        c2 = h * rad.sin();
        c1 = h * rad.cos();
    }
    else if c1 > eps && a_abc > eps && h <= eps {
        let rad = a_abc.to_radians();
        h = c1 / rad.cos();
        // Calculate missing cathetus if it hasn't been filled
        if c2 <= eps { c2 = c1 * rad.tan(); }
    }
    else if c2 > eps && a_abc > eps && h <= eps {
        let rad = a_abc.to_radians();
        h = c2 / rad.sin();
        if c1 <= eps { c1 = c2 / rad.tan(); }
    }

    // Step 3: If all sides are now known, but angles are missing, calculate them
    if h > eps && c1 > eps && c2 > eps && a_abc <= eps {
        a_abc = (c2 / h).asin().to_degrees();
        a_acb = 90.0 - a_abc;
    }

    // If any value is still zero at this point, we didn't have enough input to finish.
    // The user is likely still typing, so return Ok(None) to wait silently.
    if h <= eps || c1 <= eps || c2 <= eps || a_abc <= eps || a_acb <= eps {
        return Ok(None);
    }

    // Validation check for geometric anomalies post-calculation
    if h > eps && (c1 >= h || c2 >= h) {
        return Err("Invalid geometry: Computed hypotenuse is smaller than a cathetus.".into());
    }

    Ok(Some((h, c1, c2, a_abc, a_acb)))
}