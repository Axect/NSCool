#[derive(Debug, Clone)]
pub struct ControlCon {
    pub c_con_str: f64,
    pub p_con_str: f64,
    pub i_opacity: usize,
    pub i_con_crust: usize,
    pub i_con_core: usize,
}

#[derive(Debug, Clone)]
pub struct RhoLimits {
    pub rho_drip: f64,
    pub rho_core: f64,
}

#[derive(Debug, Clone)]
pub struct ProfileStar {
    pub rad: Vec<f64>,
    pub rrho: Vec<f64>,
    pub pres: Vec<f64>,
    pub debar: Vec<f64>,
    pub dvol: Vec<f64>,
    pub emas: Vec<f64>,
    pub phi: Vec<f64>,
    pub rhod: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct Star {
    pub i_max: usize,
    pub i_core: usize,
    pub i_drip: usize,
    pub i_env: usize,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Accretion {
    pub m_dot0: f64,
    pub t_acc0: f64,
    pub t_acc1: f64,
    pub t_acc2: f64,
    pub alpha_acc: f64,
    pub m_dot_ini: f64,
    pub m_dot_max: f64,
    pub m_dot_av: f64,
    pub m_dot_quiet: f64,
    pub m_dot_ris: f64,
    pub t_burst: f64,
    pub eta_Edd: f64,
    pub X_Edd: f64,
    pub i_acc: AccretionType,
    pub i_cycle: usize,
    pub v_acc: Vec<f64>,
}

#[derive(Debug, Copy, Clone)]
pub enum AccretionType {
    None,
    TransientFRED,
    TransientSTEP,
}

#[derive(Debug, Clone)]
pub struct Gravity {
    pub gs14: f64,
    pub compactness: usize,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Bound {
    pub Tb_acc0: f64,
    pub f_TbTs: String,
}