use crate::backend::inc::ControlCon;

/// Opacity
///
/// # Description
/// * temperature in K  density in gm/cm3  op given in cm2/gm
/// * Uses Kramer opacity only, from Shapiro & Teukolsky
#[allow(non_snake_case)]
pub fn opacity(T: f64, Rho: f64, A: &mut f64, Z: &mut f64, control_con: ControlCon) -> f64 {
    let i_opacity = control_con.i_opacity;
    if i_opacity == 0 {
        return 1e+200;
    } else {
        if Rho >= 1e+14 {
            *A = 100f64;
            *Z = 32f64;
        }
        return 0.645e+23 * Z.powi(3) / A.powi(2) * Rho / T.powf(3.5);
    }
}