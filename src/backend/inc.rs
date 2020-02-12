/// `control_con.inc.f`
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

#[derive(Debug, Clone)]
pub struct Superfluid {
    pub tcn: Vec<f64>,
    pub tcp: Vec<f64>,
    pub tcla: Vec<f64>,
    pub tcsm: Vec<f64>,
    pub tcs0: Vec<f64>,
    pub tcsp: Vec<f64>,
    pub tcuu: Vec<f64>,
    pub tcdd: Vec<f64>,
    pub tcss: Vec<f64>,
    pub tcud: Vec<f64>,
    pub tcus: Vec<f64>,
    pub tcds: Vec<f64>,
    pub tcu: Vec<f64>,
    pub tcd: Vec<f64>,
    pub tcs: Vec<f64>,
    pub tcu1: Vec<f64>,
    pub tcu2: Vec<f64>,
    pub tcu3: Vec<f64>,
    pub tcd1: Vec<f64>,
    pub tcd2: Vec<f64>,
    pub tcd3: Vec<f64>,
    pub tcs1: Vec<f64>,
    pub tcs2: Vec<f64>,
    pub tcs3: Vec<f64>,
    pub sfn1s0: f64,
    pub sfn3p2: f64,
    pub sfp1s0: f64,
    pub sfl1s0: f64,
    pub sfquark: f64,
    pub fn1s0: f64,
    pub fn3p2: f64,
    pub fp1s0: f64,
    pub fl1s0: f64,
    pub kfmax_n3p2: f64,
    pub delkf_n3p2: f64,
    pub tcmax_n3p2: f64,
    pub i_sf: usize,
}

/// Profile Comp
///
/// theta_k and theta_p are the chiral angles for Kaon and Pion condensates
///
/// z_ion  is the charge number of the nuclei
/// a_ion  is the mass number of the nuclei
/// a_cell is the number of nucleons per Wigner-Seitz cell
///        (i.e., a_cell=a_ion + # dripped neutrons)
#[derive(Debug, Clone)]
pub struct ProfileComp {
    pub bar: Vec<f64>,
    pub yneutr: Vec<f64>,
    pub yprot: Vec<f64>,
    pub yelect: Vec<f64>,
    pub ymuon: Vec<f64>,
    pub ylambda: Vec<f64>,
    pub ysminus: Vec<f64>,
    pub yszero: Vec<f64>,
    pub ysplus: Vec<f64>,
    pub yquarku: Vec<f64>,
    pub yquarkd: Vec<f64>,
    pub yquarks: Vec<f64>,
    pub fhad: Vec<f64>,
    pub theta_k: Vec<f64>,
    pub theta_p: Vec<f64>,
    pub a_cell: Vec<f64>,
    pub a_ion: Vec<f64>,
    pub z_ion: Vec<f64>,
    pub v_ion: Vec<f64>,
    pub i_strange: usize,
}

#[derive(Debug, Clone)]
pub struct Fermi {
    pub mstp: Vec<f64>,
    pub mstn: Vec<f64>,
    pub mstla: Vec<f64>,
    pub mstsm: Vec<f64>,
    pub msts0: Vec<f64>,
    pub mstsp: Vec<f64>,
    pub kfe: Vec<f64>,
    pub kfm: Vec<f64>,
    pub kfp: Vec<f64>,
    pub kfn: Vec<f64>,
    pub kfla: Vec<f64>,
    pub kfsm: Vec<f64>,
    pub kfs0: Vec<f64>,
    pub kfsp: Vec<f64>,
    pub kfqu: Vec<f64>,
    pub kfqd: Vec<f64>,
    pub kfqs: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct EffMass {
    pub emnco: f64,
    pub emncr: f64,
    pub emp: f64,
}

//#[allow(non_snake_case)]
//#[derive(Debug, Clone)]
//pub struct Bound {
//    pub Tb_acc0: f64,
//    pub f_TbTs: String,
//}