use crate::backend::inc::{Accretion, ProfileStar, Gravity, AccretionType, Star};

#[allow(non_snake_case)]
pub fn initialize_accretion_rate(acc: &mut Accretion, prof: &mut ProfileStar, grav: &Gravity) {
    let mut m_dot1 = 0f64;
    let mut eta = 0f64;
    let mut X = 0f64;
    let mut opac = 0f64;
    let (mut F_Edd, L_Edd, mut M_Edd) = (0f64, 0f64, 0f64);

    if acc.m_dot0 > 1e+10 {         // m_dot in g/sec
        m_dot1 = acc.m_dot0;
    } else if acc.m_dot0 < 1e-6 {   // m_dot in Msun/yr
        m_dot1 = acc.m_dot0 * 2e+33 / 3.15576e+7;
    } else {                    // m_dot in unit of M_Edd
        eta = acc.eta_Edd;      // Efficiency of mass -> energy
        X = acc.X_Edd;          // Hydrogen fraction
        opac = 6.023e+23 * 0.665e-24 * (1f64 + X) / 2f64;
        F_Edd = 3e+10 * (1e+14 * grav.gs14) / opac;
        M_Edd = L_Edd / 3e+10f64.powi(2) / eta;
        m_dot1 = acc.m_dot0 * M_Edd;
    }

//    let mass_acc = 0f64;
    let tau_acc_r: f64;

    match acc.i_acc {
        AccretionType::None => (),
        AccretionType::TransientFRED => {
            // t_acc0 = beginning of accretion
            // t_acc1 = duration of each accretion cycle
            // t_acc2 = tau_acc_r = duration of exponential rise
            tau_acc_r = acc.t_acc2;
            if acc.m_dot0 < 1f64 {
                // In this case m_dot0 is actually DelM in Mo
                let coeff = 1f64 / (1f64.exp() - 1f64) + (1f64 - (acc.t_acc2 / acc.t_acc1).powf(acc.alpha_acc - 1f64)) / (acc.alpha_acc - 1f64);
                acc.m_dot0 /= acc.t_acc2 / coeff * 2e+33;
            }
            acc.m_dot_max = acc.m_dot0;
            acc.m_dot_ris = acc.m_dot_max / (1f64 - (-1f64).exp());
            let macc_r = acc.t_acc2 * acc.m_dot_ris / 1f64.exp();
            let macc_d = acc.t_acc2 * acc.m_dot_max / (acc.alpha_acc - 1f64) * (1f64 - (acc.t_acc2 / acc.t_acc1).powf(acc.alpha_acc - 1f64));
            acc.m_dot_av = (macc_r + macc_d) / acc.t_acc1;
            acc.m_dot_ini = acc.m_dot_av;
            acc.m_dot_quiet = 0f64;
            let coeff_ = 3.15e+7 / 2e+33;
        }
        AccretionType::TransientSTEP => {
            acc.m_dot_max = m_dot1;
            acc.m_dot_av = m_dot1 * acc.t_acc2 / acc.t_acc1;
            acc.m_dot_ini = acc.m_dot_av;
            acc.m_dot_quiet = 0f64;
            let coeff_ = 3.15e+7 / 2e+33;
        }
    }
}

pub fn accretion_rate(time: f64, dtime: f64, acc: &mut Accretion) -> f64 {
    let t_acc2 = acc.t_acc2;
    let m_dot_ris = acc.m_dot_ris;
    let m_dot_max = acc.m_dot_max;
    let alpha_acc = acc.alpha_acc;
    let fr = |t: f64| m_dot_ris * (t + t_acc2 * (-t / t_acc2).exp());
    let fd = |t: f64| m_dot_max * t / (1f64 - alpha_acc) * (t_acc2 / t).powf(alpha_acc);

    match acc.i_acc {
        AccretionType::None => 0f64,
        AccretionType::TransientFRED => {
            if (time - dtime) <= acc.t_acc0 {
                return acc.m_dot_ini;
            } else {
                if dtime >= acc.t_acc1 {
                    panic!("dtime larger than t_acc1!");
                }
                let time0 = time - dtime;
                let time1 = time;
                let i_cycle0 = ((time0 - acc.t_acc0) / acc.t_acc1) as usize;
                let t0 = time0 - acc.t_acc0 - (i_cycle0 as f64) * acc.t_acc1;
                let t1 = time1 - acc.t_acc0 - (i_cycle0 as f64) * acc.t_acc1;
                acc.i_cycle = i_cycle0;
                let delt_acc = t1;
                let m_acc = {
                    if t0 <= acc.t_acc2 {
                        if t1 <= acc.t_acc2 {
                            fr(t1) - fr(t0)
                        } else if t1 <= acc.t_acc1 {
                            let m_acc0 = fr(acc.t_acc2) - fr(t0);
                            let m_acc1 = fd(t1) - fd(acc.t_acc2);
                            m_acc0 + m_acc1
                        } else {
                            let m_acc0 = fr(acc.t_acc2) - fr(t0);
                            let m_acc1 = fd(acc.t_acc1) - fd(acc.t_acc2);
                            let m_acc2 = fr(t1 - acc.t_acc1) - fr(0f64);
                            m_acc0 + m_acc1 + m_acc2
                        }
                    } else {
                        if t1 <= acc.t_acc1 {
                            fd(t1) - fd(t0)
                        } else if t1 <= (acc.t_acc1 + acc.t_acc2) {
                            let m_acc0 = fd(acc.t_acc1) - fd(t0);
                            let m_acc1 = fr(t1 - acc.t_acc1) - fr(0f64);
                            m_acc0 + m_acc1
                        } else {
                            let m_acc0 = fd(acc.t_acc1) - fd(t0);
                            let m_acc1 = fr(acc.t_acc2) - fr(0f64);
                            let m_acc2 = fd(t1 - acc.t_acc1) - fd(acc.t_acc2);
                            m_acc0 + m_acc1 + m_acc2
                        }
                    }
                };
                m_acc / dtime
            }
        }
        AccretionType::TransientSTEP => {
            if (time + dtime) <= acc.t_acc0 {
                acc.m_dot_ini
            } else {
                if acc.t_burst <= acc.t_acc2 {
                    acc.m_dot_max
                } else {
                    acc.m_dot_quiet
                }
            }
        }
    }
}

pub fn accretion_velocity(m_dot: f64, acc: &mut Accretion, prof: &ProfileStar, star: &Star) {
    use std::f64::consts::PI;
    for i in 0 .. star.i_max + 1 {
        acc.v_acc[i] = - m_dot / (4f64 * PI * prof.rad[i].powi(2) * prof.rrho[i]);
    }
    acc.v_acc[0] = 0f64;
}