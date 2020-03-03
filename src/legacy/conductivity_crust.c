//
// Created by xteca on 2/14/20.
//

// lambda is mutable (return)
extern void con_crust_ee_(double* T, double* ne, double* kfe, double* lambda, double* debug);
// sigma, lambda, nu_e_s, nu_e_l is mutable (return)
extern void con_crust_e_phonon_BY_(double* Temp, double* rho, double* A, double* A1, double* Z, double* sigma, double* lambda, double* debug, double* nu_e_s, double* nu_e_l, double* rhodrip);
extern void con_env_e_phon_ion_PBHY_(double* T, double* rho, double* A, double* A1, double* Z, double* sigma, double* lambda, double* debug, double* nu_e_s, double* nu_e_l);
extern void con_e_phon_ion_GYP_(double* T, double* rho, double* A, double* A1, double* Z, int* ifs, double* sigma, double* lambda, double* debug, double* nu_e_s, double* nu_e_l, double* rhodrip);
// Lam1, Lam2 is mutable (return)
extern void get_lam_(double* s, double* w, double* Lam1, double* Lam2);

extern double exp_int_(double* x);

// Z, Anuc, tp, xnuc, xnuct are mutable (return)
extern void OYAFORM_(double* BARD, int* Index, double* Z, double* Anuc, double* A, double* xnuc, double* xnuct);

// sigma, lambda, nu_e_s, nu_e_l are mutable (return)
extern void con_crust_e_ion_Itoh_(double* T, double* rho, double* A, double* A1, double* Z, double* sigma, double* lambda, double* debug, double* nu_e_s, double* nu_e_l, double* rhodrip);
extern void con_crust_e_phonon_YU_(double* T, double* rho, double* A, double* A1, double* Z, double* sigma, double* lambda, double* debug, double* nu_e_s, double* nu_e_l);
// extern void con_crust_e_ion_YU_()
extern void con_crust_e_imp_YU_(double* T, double* rho, double* A, double* A1, double* Z, double* Q, double* sigma, double* lambda, double* debug, double* nu_e_s, double* nu_e_l);
extern double Coulomb_imp_YU_(double* x);