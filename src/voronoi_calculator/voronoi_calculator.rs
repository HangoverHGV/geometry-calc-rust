use voronoice::{BoundingBox, ClipBehavior, Point, VoronoiBuilder};
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::Path;


pub struct VoronoiParameters {
    pub width: f64,
    pub height: f64,
    pub seed: f32,
    pub space: f32,
    pub min_edge: f32,
}

/// A closed cell outline in mm, origin top-left, Y increasing downwards.
pub type Polygon = Vec<(f64, f64)>;

/// Computes the Voronoi cells and returns their raw outlines.
///
/// This is the single source of truth for the geometry — both the Slint
/// preview and the DXF export are derived from it, so they can never drift
/// apart. Deterministic for a given set of parameters.
pub fn voronoi_polygons(params: &VoronoiParameters) -> Vec<Polygon> {
    if params.width <= 0.0 || params.height <= 0.0 {
        return Vec::new();
    }

    // 1. Generate pseudo-random sites from the seed.
    //
    // NOTE: `seed` currently drives BOTH the RNG state and the number of points,
    // so moving the slider changes density as well as layout. If you want those
    // independent, add a separate `count` field to VoronoiParameters.
    let point_count = (params.seed.max(3.0)) as usize;
    let mut rng_seed = params.seed as u64;

    let mut next_random = || {
        rng_seed = rng_seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((rng_seed >> 16) & 0x7fff) as f64 / 32767.0
    };

    let mut sites = Vec::with_capacity(point_count);
    for _ in 0..point_count {
        let x = next_random() * params.width;
        let y = next_random() * params.height;
        sites.push(Point { x, y });
    }

    // 2. A true rectangular bounding box, centered on the canvas.
    let bounding_box = BoundingBox::new(
        Point {
            x: params.width / 2.0,
            y: params.height / 2.0,
        },
        params.width,
        params.height,
    );

    let voronoi = match VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(bounding_box)
        .set_clip_behavior(ClipBehavior::Clip) // clip cells to the rectangle
        .build()
    {
        Some(v) => v,
        None => return Vec::new(),
    };

    // 3. Inset each cell by `space / 2`.
    let inset = (params.space as f64) / 2.0;
    let mut polygons = Vec::new();

    for cell in voronoi.iter_cells() {
        let polygon: Vec<&Point> = cell.iter_vertices().collect();

        // `min_edge` is compared against VERTEX COUNT here. Real Voronoi cells
        // rarely exceed ~10 vertices, so keep the UI slider capped accordingly
        // or this silently filters everything out.
        if polygon.len() < 3 || polygon.len() < params.min_edge as usize {
            continue;
        }

        // Centroid (vertex average — good enough for insetting).
        let n = polygon.len() as f64;
        let cx = polygon.iter().map(|p| p.x).sum::<f64>() / n;
        let cy = polygon.iter().map(|p| p.y).sum::<f64>() / n;

        let mut inset_pts = Vec::with_capacity(polygon.len());
        let mut degenerate = false;

        for p in &polygon {
            let dx = p.x - cx;
            let dy = p.y - cy;
            let len = (dx * dx + dy * dy).sqrt();

            if len <= inset {
                // Cell is smaller than the requested gap — drop it entirely
                // rather than emitting an inverted polygon.
                degenerate = true;
                break;
            }

            let f = if len > 0.0 { (len - inset) / len } else { 0.0 };
            inset_pts.push((cx + dx * f, cy + dy * f));
        }

        if !degenerate {
            polygons.push(inset_pts);
        }
    }

    polygons
}

/// Converts the cells into SVG-style path strings ("M x y L x y ... Z") for
/// the Slint `Path` preview.
pub fn voronoi_calculator(params: VoronoiParameters) -> Vec<String> {
    voronoi_polygons(&params)
        .into_iter()
        .map(|poly| {
            let mut path_data = String::new();
            for (i, (px, py)) in poly.iter().enumerate() {
                if i == 0 {
                    path_data.push_str(&format!("M {:.2} {:.2} ", px, py));
                } else {
                    path_data.push_str(&format!("L {:.2} {:.2} ", px, py));
                }
            }
            path_data.push_str("Z");
            path_data
        })
        .collect()
}


const LAYER_NAME: &str = "VORONOI";

pub fn write_dxf(path: &Path, polygons: &[Polygon], canvas_width: f64, canvas_height: f64) -> Result<()> {
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);

    let flip = |y: f64| canvas_height - y;

    // ---- HEADER ----
    write_pair(&mut w, 0, "SECTION")?;
    write_pair(&mut w, 2, "HEADER")?;

    write_pair(&mut w, 9, "$ACADVER")?;
    write_pair(&mut w, 1, "AC1009")?;

    write_pair(&mut w, 9, "$EXTMIN")?;
    write_pair(&mut w, 10, "0.0")?;
    write_pair(&mut w, 20, "0.0")?;
    write_pair(&mut w, 30, "0.0")?;

    write_pair(&mut w, 9, "$EXTMAX")?;
    write_pair(&mut w, 10, &format!("{:.6}", canvas_width))?;
    write_pair(&mut w, 20, &format!("{:.6}", canvas_height))?;
    write_pair(&mut w, 30, "0.0")?;

    write_pair(&mut w, 0, "ENDSEC")?;

    // ---- TABLES (declare the layer so pickier readers don't complain) ----
    write_pair(&mut w, 0, "SECTION")?;
    write_pair(&mut w, 2, "TABLES")?;
    write_pair(&mut w, 0, "TABLE")?;
    write_pair(&mut w, 2, "LAYER")?;
    write_pair(&mut w, 70, "1")?;

    write_pair(&mut w, 0, "LAYER")?;
    write_pair(&mut w, 2, LAYER_NAME)?;
    write_pair(&mut w, 70, "0")?;
    write_pair(&mut w, 62, "7")?; // colour: white/black
    write_pair(&mut w, 6, "CONTINUOUS")?;

    write_pair(&mut w, 0, "ENDTAB")?;
    write_pair(&mut w, 0, "ENDSEC")?;

    // ---- ENTITIES ----
    write_pair(&mut w, 0, "SECTION")?;
    write_pair(&mut w, 2, "ENTITIES")?;

    for poly in polygons {
        if poly.len() < 3 {
            continue;
        }

        write_pair(&mut w, 0, "POLYLINE")?;
        write_pair(&mut w, 8, LAYER_NAME)?;
        write_pair(&mut w, 66, "1")?; // vertices follow
        write_pair(&mut w, 70, "1")?; // closed polyline
        write_pair(&mut w, 10, "0.0")?;
        write_pair(&mut w, 20, "0.0")?;
        write_pair(&mut w, 30, "0.0")?;

        for (x, y) in poly {
            write_pair(&mut w, 0, "VERTEX")?;
            write_pair(&mut w, 8, LAYER_NAME)?;
            write_pair(&mut w, 10, &format!("{:.6}", x))?;
            write_pair(&mut w, 20, &format!("{:.6}", flip(*y)))?;
            write_pair(&mut w, 30, "0.0")?;
        }

        write_pair(&mut w, 0, "SEQEND")?;
        write_pair(&mut w, 8, LAYER_NAME)?;
    }

    write_pair(&mut w, 0, "ENDSEC")?;
    write_pair(&mut w, 0, "EOF")?;

    w.flush()?;
    Ok(())
}

/// DXF is a stream of (group code, value) pairs, each on its own line.
fn write_pair<W: Write>(w: &mut W, code: i32, value: &str) -> Result<()> {
    writeln!(w, "{}", code)?;
    writeln!(w, "{}", value)?;
    Ok(())
}
