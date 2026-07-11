/// Primește datele introduse și calculează TOATE valorile lipsă ale triunghiului dreptunghic.
/// Returnează un tuplu complet: Some((ipotenuză, catetă1, catetă2, unghi_abc, unghi_acb))
pub fn calculate_all_triangle_elements(
    hyp: f64,
    cat1: f64,
    cat2: f64,
    angle_abc: f64,
    angle_acb: f64
) -> Option<(f64, f64, f64, f64, f64)> {

    let mut h = hyp;
    let mut c1 = cat1;
    let mut c2 = cat2;
    let mut a_abc = angle_abc;
    let mut a_acb = angle_acb;

    // Pasul 1: Dacă se dau ambele unghiuri ascuțite dar nicio latură -> Imposibil de calculat dimensiunile
    if h == 0.0 && c1 == 0.0 && c2 == 0.0 {
        return None;
    }

    // Pasul 2: Determinăm unghiurile dacă unul este cunoscut
    if a_abc > 0.0 && a_acb == 0.0 { a_acb = 90.0 - a_abc; }
    if a_acb > 0.0 && a_abc == 0.0 { a_abc = 90.0 - a_acb; }

    // Pasul 3: Rezolvarea laturilor în funcție de ce avem disponibil
    // Cazul A: Avem două catete
    if c1 > 0.0 && c2 > 0.0 && h == 0.0 {
        h = (c1.powi(2) + c2.powi(2)).sqrt();
    }
    // Cazul B: Avem ipotenuza și o catetă
    else if h > 0.0 && c1 > 0.0 && c2 == 0.0 {
        if c1 >= h { return None; }
        c2 = (h.powi(2) - c1.powi(2)).sqrt();
    }
    else if h > 0.0 && c2 > 0.0 && c1 == 0.0 {
        if c2 >= h { return None; }
        c1 = (h.powi(2) - c2.powi(2)).sqrt();
    }
    // Cazul C: Avem Ipotenuza și un unghi (Folosim unghiul ABC)
    else if h > 0.0 && a_abc > 0.0 && c1 == 0.0 && c2 == 0.0 {
        let rad = a_abc.to_radians();
        c2 = h * rad.sin(); // Cateta AC (opusă lui B/ABC)
        c1 = h * rad.cos(); // Cateta AB (alăturată lui B/ABC)
    }
    // Cazul D: Avem o catetă și un unghi
    else if c1 > 0.0 && a_abc > 0.0 && h == 0.0 && c2 == 0.0 {
        let rad = a_abc.to_radians();
        h = c1 / rad.cos();
        c2 = c1 * rad.tan();
    }
    else if c2 > 0.0 && a_abc > 0.0 && h == 0.0 && c1 == 0.0 {
        let rad = a_abc.to_radians();
        h = c2 / rad.sin();
        c1 = c2 / rad.tan();
    }

    // Pasul 4: Dacă am aflat laturile dar nu aveam unghiurile inițial, le calculăm acum
    if h > 0.0 && c1 > 0.0 && c2 > 0.0 && a_abc == 0.0 {
        // În desenul tău, unghiul ABC are ca catetă opusă pe AC (catetă 2)
        a_abc = (c2 / h).asin().to_degrees();
        a_acb = 90.0 - a_abc;
    }

    // Validare finală de siguranță structurală
    if h > 0.0 && (c1 >= h || c2 >= h) {
        return None;
    }

    Some((h, c1, c2, a_abc, a_acb))
}