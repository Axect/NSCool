! *********************************************************************
! *********************************************************************
! *********************************************************************
!              Electron-electron scattering contribution
! *********************************************************************
! *********************************************************************
! *********************************************************************
subroutine con_crust_ee (T, ne, kfe, lambda, debug)
    ! *********************************************************************
    ! * Calculates the conductivity due to electron-electron scattering   *
    ! * in the crust.                                                     *
    ! * Units are cgs-K, except kfe in fm^-1                              *
    ! *                                                                   *
    ! * From Shternin & Yakovlev, Phys. Rev. D74, 043004 (2006)           *
    ! *                                                                   *
    ! * Checked on August 28, 2009, against Fig 2 of the paper.           *
    ! *********************************************************************
    implicit real*8 (a-h, k-z)
    real*8 I_l, I_t, I_lt
    parameter (pi = 4.d0 * atan(1.d0))
    parameter (Na = 6.022d23, kb = 1.380d-16, Mu = 1.d0 / Na)
    parameter (me = 9.109d-28, e = 4.803206d-10)
    parameter (hbar = 1.054572d-27, cl = 2.99792458d10)
    parameter (hbc = 197.327d0, MeV = 1.602177d-6)
    if (debug.eq.1.2d0) print *, 'Entering con_crust_ee:', &
            ' T, ne =', T, ne
    ! ***
    xe = 197.3d0 * kfe / 0.511d0
    gammae = dsqrt(1.d0 + xe**2)
    u = xe / gammae
    mste = gammae * me
    om_pe = dsqrt(4.d0 * pi * e**2 * ne / mste)
    T_pe = hbar * om_pe / kb
    th = dsqrt(3.d0) * T_pe / T
    ! ***
    I_l = (0.1587d0 - 0.02538d0 / (1.d0 + 0.0435d0 * th)) * &
            log(1.d0 + 128.56d0 / (37.1d0 * th + 10.83d0 * th**2 + th**3)) / u
    ! ***
    A = 20.d0 + 450d0 * u**3
    C1 = 0.05067d0 + 0.03216d0 * u**2
    C2 = 0.0254d0 + 0.04127d0 * u**4
    C = A * dexp(C1 / C2)
    I_t = u**3 * (2.404d0 / C + (C2 - 2.404d0 / C) / (1.d0 + 0.1d0 * th * u)) * &
            log(1.d0 + C / (A * th * u + th**2 * u**2))
    ! ***
    A = 12.2d0 + 25.2d0 * u**3
    B = 1.d0 - 0.75d0 * u
    C1 = 0.123636d0 + 0.016234d0 * u**2
    C2 = 0.0762d0 + 0.05714d0 * u**4
    C = A * dexp(C1 / C2)
    I_lt = u * &
            (18.52d0 * u**2 / C + (C2 - 18.52d0 * u**2 / C) / (1.d0 + 0.1558d0 * th**B)) * &
            log(1.d0 + C / (A * th + 10.83d0 * th**2 * u**2 + (th * u)**(8. / 3.)))
    ! ***
    alpha = 1.d0 / 137.036d0
    lambda = pi**3 * kb**3 * T**2 / &
            (108.d0 * alpha**2 * hbar**2 * cl * (I_l + I_t + I_lt))

    if (debug.eq.1.2d0) print *, 'Exiting con_crust_ee'
    return
end

! *********************************************************************
! *********************************************************************
! *********************************************************************
!                   Baiko & Yakovlev calculations
! *********************************************************************
! *********************************************************************
! *********************************************************************
subroutine con_crust_e_phonon_BY(Temp, rho, A, A1, Z, &
        sigma, lambda, debug, &
        nu_e_s, nu_e_l, &
        rhodrip)
    !     a = nucleons per cell   a1= nucleons per nucleus
    ! *********************************************************************
    ! * Calculates the conductivity due to electron-phonon scattering by  *
    ! * umklapp processes in the crust in the crystalline phase.          *
    ! * Units are cgs-K                                                   *
    ! *                                                                   *
    ! * From Baiko & Yakovlev, Astron. Lett. 22 (1996): 708               *
    ! * Checked on JULY 19, 2000                                          *
    ! *********************************************************************
    implicit real*8 (a-h, k-z)
!    INCLUDE 'rho_limits.inc.f'
    parameter (pi = 3.14159265)
    parameter(u_1 = 2.80, u_2 = 13.00, a_0 = 0.01740, a_2 = 0.01180)     ! BCC lattice
    !       parameter(u_1=4.03,u_2=28.80,a_0=0.00505,a_2=0.00461)     ! FCC lattice
    ! ***************************
    ei(q) = dexp(-q**4 / (q**3 + 0.1397d0)) * &
            (dlog(1.d0 + 1.d0 / q) - 0.5772d0 / (1.d0 + 2.2757d0 * q**2))
    ! ***************************
    if (debug.eq.1.2d0) print *, 'Entering con_crust_e_phonon:', &
            ' T, rho=', Temp, rho
    rho6 = rho / 1.d6
    T8 = temp / 1.d8
    n_i = rho / A * 6.022d23
    a_WS = (3.d0 / 4.d0 / pi / n_i)**(1.d0 / 3.d0)
    if (rho.lt.rhodrip) then
        r_nucl = 1.15d-13 * A**(1.d0 / 3.d0)
    else
        r_nucl = 1.83d-13 * Z**(1.d0 / 3.d0)
    end if
    g2 = (r_nucl / a_WS)**2
    ! ***************************
    x = 1.0088d0 * (rho6 * Z / A)**(1.d0 / 3.d0)                 ! = p_F/mc
    beta = x / (1.d0 + x**2)**(1.d0 / 2.d0)                    ! = v_F/c
    gam = 7.832d-2 * Z / T8 * (rho6 / A / A1)**(1.d0 / 2.d0)      ! = T_p/T
    ! ***************************
    G_0 = u_2 / (1.d0 + a_0 * gam**2)**(1.d0 / 2.d0)
    G_2 = gam**2 / pi**2 / (1.d0 + a_2 * gam**2)**(3.d0 / 2.d0)
    ! ***************************
    ue = 1.d0 / 137.d0 / pi / beta
    u1 = 1.d0 / (4.d0 * Z)**(2.d0 / 3.d0) + ue
    alpha0 = 1.683d0 * dsqrt(x / A1 / Z)
    alpha = alpha0 * (0.5d0 * u_1 * dexp(-9.1d0 / gam) + u_2 / gam)
    w = alpha * u1
    S__1 = ei(w) - ei(alpha)
    S_0 = (dexp(-w) - dexp(-alpha)) / alpha
    S_1 = (dexp(-w) * (w + 1.d0) - dexp(-alpha) * (alpha + 1.d0)) / alpha**2
    S_2 = (dexp(-w) * (w**2 + 2.d0 * w + 2.d0) - &
            dexp(-alpha) * (alpha**2 + 2.d0 * alpha + 2.d0)) / alpha**3
    Phi_0 = S__1 - beta**2 * S_0
    Phi_1 = S_0 - beta**2 * S_1
    Phi_2 = S_1 - beta**2 * S_2
    P0 = 4.787d0 - 0.0346d0 * Z
    P2 = 2.729d0 - 0.0204d0 * Z
    K_0 = 2.0d0 * Phi_1 / &
            (1.d0 + (18.d0 * pi * Z)**(2.d0 / 3.d0) * g2 * Phi_2 / 5.d0 / Phi_1 / P0)**P0
    K_2 = 0.5d0 * Phi_0 / &
            (1.d0 + (18.d0 * pi * Z)**(2.d0 / 3.d0) * g2 * Phi_1 / 5.d0 / Phi_0 / P2)**P2
    ! ***************************
    F_s = G_0 * K_0
    F_k = G_0 * K_0 + G_2 * (3.d0 * K_2 - 0.5d0 * K_0)
    nu_s = 0.9554d17 * T8 / beta * F_s
    nu_k = 0.9554d17 * T8 / beta * F_k
    ! ***************************
    sigma = 1.49d22 * x**2 * beta * (1.d16 / nu_s)
    lambda = 4.04d15 * x**2 * beta * T8 * (1.d18 / nu_k)
    nu_e_s = nu_s
    nu_e_l = nu_k

    if (debug.eq.1.2d0) print *, 'Exiting con_crust_e_phonon:', &
            ' sigma, lambda=', sigma, lambda
    return
end

! *********************************************************************
! *********************************************************************
! *********************************************************************
!         Potekhin, Baiko, Haensel & Yakovlev calculations
! *********************************************************************
! *********************************************************************
! *********************************************************************
subroutine con_env_e_phon_ion_PBHY(T, rho, A, A1, Z, &
        sigma, lambda, debug, &
        nu_e_s, nu_e_l)
    !     a = nucleons per cell   a1= nucleons per nucleus
    ! *********************************************************************
    ! * Calculates the conductivity due to electron-phonon scattering by  *
    ! * umklapp processes in the crust in the crystalline phase AND       *
    ! * the electron-ion contribution in the liquid phase !               *
    ! *                                                                   *
    ! * Really valid only at Rho < 10^10 g/cm^3                           *
    ! *                                                                   *
    ! * Units are cgs-K                                                   *
    ! *                                                                   *
    ! * From Potekhin et al, A&A 346 (1999): 345                          *
    ! * Checked on Dec. 4, 2003                                           *
    ! *********************************************************************
    implicit real*8 (a-h, k-z)
    parameter (pi = 3.14159265d0)
    parameter (c = 2.99d10, kb = 1.380d-16, hb = 1.054d-27, a_f = 1. / 137.)
    parameter (e = 4.803d-10, me = 9.109d-28, mu = 1.66d-24)
    parameter(u_1 = 2.80d0, u_2 = 13.00d0, a_0 = 0.01740, a_2 = 0.01180)     ! BCC lattice
    !       parameter(u_1=4.03d0,u_2=28.80d0,a_0=0.00505,a_2=0.00461)     ! FCC lattice
    ! ***************************
    if (debug.eq.1.2d0) print *, 'Entering con_crust_e_phon_ion:', &
            ' T, rho=', T, rho
    if (rho.gt.1.d10) then
        pause 'Subroutine con_env_e_phon_ion: rho > 1e10 !'
    end if
    ! ***************************
    n_i = rho / A * 6.022d23
    n_e = Z * n_i
    kf = (3. * pi**2 * n_e)**(1. / 3.)
    pf = hb * kf
    m_st = sqrt(me**2 + (pf / c)**2)
    Ef = m_st * c**2
    vf = pf / m_st
    ! ******
    omega_p = sqrt(4.d0 * pi * e**2 * Z**2 * n_i / A1 / mu)
    T_p = hb / kb * omega_p
    eta = T / T_p
    beta = pi * a_f * Z * vf / c
    ai = (3.d0 / 4.d0 / pi / n_i)**(1. / 3.)
    Gamma = e**2 / kb * Z**2 / T / ai
    q_D2 = 3.d0 * Gamma / ai**2
    k_TF2 = 4.d0 * a_f / pi * c / vf * kf**2
    ! ******
    q_i2 = q_D2 * (1.0d0 + 0.06d0 * Gamma) * dexp(-dsqrt(Gamma))
    q_s2 = (q_i2 + k_TF2) * dexp(-beta)
    s = q_s2 / (2.d0 * kf)**2
    w = u_2 * (2.d0 * kf)**2 / q_D2 * (1.0d0 + beta / 3.0d0)
    ! ******
    eta_02 = (0.19)**2 / Z**(1. / 3.)
    G_s = eta / sqrt(eta**2 + eta_02) * (1.0d0 + 0.122d0 * beta**2)
    G_l = G_s + &
            0.0105d0 * (1.0d0 - 1.0d0 / Z) * (1.d0 + (vf / c)**3 * beta) * &
                    eta / (eta**2 + 0.0081d0)**1.5
    ! ***************************
    if ((s.le.0.01).and.(s * w.le.0.01)) then
        Lam1 = 0.5d0 * (exp_int(w) + dlog(w) + 0.5772156d0)
        Lam2 = (dexp(-w) - 1.0d0 + w) / (2.0d0 * w)
    else if (w.le.0.01) then
        Lam1 = w * ((2.0 * s + 1.0) / (2.0 * s + 2.) - s * log((s + 1.0) / s))
        Lam2 = w * ((1.0 - 3.0 * s - 6.0 * s * 2) / (4.0 * s + 4.0) + &
                1.5 * log((s + 1.0) / s))
    else if (w.gt.100.) then
        Lam1 = 0.5 * (log((s + 1.) / s) - 1.0 / (s + 1.0))
        Lam2 = ((2.0 * s + 1.0) / (2.0 * s + 2.0) - s * log((s + 1.) / s))
    else
        Lam1 = dlog((s + 1.0d0) / s) + s / (s + 1.0d0) * (1.0d0 - dexp(-w)) - &
                (1.0d0 + s * w) * dexp(s * w) * (exp_int(s * w) - exp_int(s * w + w))
        Lam1 = 0.5d0 * Lam1
        Lam2 = (dexp(-w) - 1.0d0 + w) / w - s**2 / (s + 1.0d0) * (1.0d0 - dexp(-w)) - &
                2.0d0 * s * dlog((s + 1.0d0) / s) + &
                s * (2.0d0 + s * w) * dexp(s * w) * (exp_int(s * w) - exp_int(s * w + w))
        Lam2 = 0.5 * Lam2
    end if
    Lam = Lam1 - (vf / c)**2 * Lam2
    ! Coulomb logarithms:
    Lam_s = Lam * G_s
    Lam_l = Lam * G_l
    ! Collisional frequencies:
    nu0 = 4.0 * pi * Z**2 * e**4 * n_i / pf**2 / vf
    nu_s = nu0 * Lam_s
    nu_l = nu0 * Lam_l
    sigma = n_e * e**2 / (m_st * nu_s)
    lambda = pi**2 * kb**2 * T * n_e / (3.d0 * m_st * nu_l)
    nu_e_s = nu_s
    nu_e_l = nu_l
    if (debug.eq.1.2d0) print *, 'Exiting con_crust_e_phon_ion:', &
            ' sigma, lambda=', sigma, lambda
    return
end

! *********************************************************************
! *********************************************************************
! *********************************************************************
!         Gnedin, Yakovlev & Potekhin modification of
!         Potekhin, Baiko, Haensel & Yakovlev calculations
!         now applicable throughout the whole crust
! *********************************************************************
! *********************************************************************
! *********************************************************************
subroutine con_e_phon_ion_GYP(T, rho, A_in, A1_in, Z_in, ifs, &
        sigma, lambda, debug, &
        nu_e_s, nu_e_l, &
        rhodrip)
    !     A_in = nucleons per cell   A1_in = nucleons per nucleus
    !                                 Z_in = protons per nucleus
    !     these being values defined in all other parts of this code, BUT
    !     ----->   THEY ARE RECALCULATED IN THE SUBROUTINE OYAFORM  <------
    !     Finite nuclear size: ifs :
    !     If ifs=1  xnuc & xnuct are calculated in OYAFORM
    !               by calling a subroutine from Oleg's code !
    !     If ifs=0  xnuc=xnuct=0
    ! **********************************************************************
    ! * Calculates the conductivity due to electron-phonon scattering by   *
    ! * umklapp processes in the crust in the crystalline phase AND        *
    ! * the electron-ion contribution in the liquid phase !                *
    ! *                                                                    *
    ! * Units are cgs-K                                                    *
    ! *                                                                    *
    ! * From Appendix of Gnedin et al, MNRAS 324 (2001): 725               *
    ! * modified from Potekhin et al, A&A 346 (1999): 345                  *
    ! * Checked on Jan. 9, 2010: (almost) exactly reproduces Fig 4 and A.1 *
    ! * of the paper                                                       *
    ! **********************************************************************
    implicit real*8 (a-h, k-z)
    parameter (pi = 3.14159265d0)
    parameter (c = 2.99d10, kb = 1.380d-16, hb = 1.054d-27, a_f = 1.d0 / 137.d0)
    parameter (e = 4.803d-10, me = 9.109d-28, mu = 1.66d-24)
    parameter(u_1 = 2.80d0, u_2 = 13.00d0, a_0 = 0.01740d0, a_2 = 0.01180d0)  ! BCC lattice
    !       parameter(u_1=4.03d0,u_2=28.80d0,a_0=0.00505d0,a_2=0.00461d0) ! FCC lattice
!    INCLUDE 'rho_limits.inc.f'
    ! ***************************
    if (debug.eq.1.2d0) then
        print *, 'Entering con_e_phon_ion_GYP:', ' T, rho=', T, rho
    end if
    ! ***************************
    !       Get the type and size of nuclei using Gnedin et al subroutine:
    BARD = rho / mu * 1.d-39
    if (rho.gt.rhodrip) then
        Index = 3
    else
        Index = 30
    end if
    call OYAFORM(BARD, Index, Z, A1, A, xnuc, xnuct)
    if (ifs.eq.0) then
        xnuc = 0.d0
        xnuct = 0.d0
    else if (ifs.ne.1) then
        pause 'Subroutine ''con_e_phon_ion_GYP'': ifs badly defined'
    end if
    ! ******
    n_i = rho / (A * mu)
    n_e = Z * n_i
    kf = (3.d0 * pi**2 * n_e)**(1. / 3.)
    pf = hb * kf
    m_st = sqrt(me**2 + (pf / c)**2)
    Ef = m_st * c**2
    vf = pf / m_st
    x = pf / me / c
    ! ******
    Omega_p = dsqrt(4.d0 * pi * e**2 * Z**2 * n_i / A1 / mu)
    T_p = hb * Omega_p / kb
    tp = T / T_p
    betaZ = pi * a_f * Z * vf / c
    ai = (3.d0 / (4.d0 * pi * n_i))**(1. / 3.)
    Gamma = Z**2 * e**2 / (kb * T * ai)
    ! ******
    r_D = ai / dsqrt(3.d0 * Gamma)
    s_D = 1.d0 / (2.d0 * kf * r_D)**2
    s_i = s_D * (1.d0 + 0.06d0 * Gamma) * dexp(-dsqrt(Gamma))
    s_e = a_f / pi * c / vf
    s = (s_i + s_e) * dexp(-betaZ)
    w = (u_2 / s_D) * (1.0d0 + betaZ / 3.0d0)
    w1 = 14.73d0 * xnuc**2 * (1.d0 + Z * dsqrt(xnuc) / 13.d0)&
            * (1.0d0 + betaZ / 3.0d0)
    ! ******
    G_s = 1.d0 / dsqrt(1.d0 + 0.0361d0 / Z**(1. / 3.) / tp**2) * &
            (1.0d0 + 0.122d0 * betaZ**2)
    G_l = G_s + &
            0.0105d0 * tp / (tp**2 + 0.0081d0)**1.5 * (1.d0 + (vf / c)**3 * betaZ) * &
                    (1.0d0 - 1.0d0 / Z) * &
                    (1.d0 + xnuct**2 * dsqrt(2.d0 * Z))     ! Finite size of nuclei correction
    ! ***************************
    D = dexp(-0.42d0 * dsqrt(x / A / Z) * u_1 * dexp(-9.1d0 * tp))
    ! ***************************
    ! Get Lambda:
    w = w + w1
    call get_lam(s, w, Lam1a, Lam2a)
    Lama = Lam1a - (vf / c)**2 * Lam2a
    w = w1
    call get_lam(s, w, Lam1b, Lam2b)
    Lamb = Lam1b - (vf / c)**2 * Lam2b
    ! ****
    Lam = Lama - Lamb
    ! Coulomb logarithms from umklapp processes, i.e., at high T:
    Lam_s_hT = Lam * G_s * D
    Lam_l_hT = Lam * G_l * D
    ! Coulomb logarithms with umklapp processes frozen, i.e., at low T:
    T_u = T_p * Z**(1. / 3.) * a_f / 3. / vf * c
    Lam_0_lT = 50. * x**0.5 / A1**0.5 / Z
    Lam_s_lT = Lam_0_lT * (4. / 3.) * a_f * c / vf * tp**5
    Lam_l_lT = Lam_0_lT * tp**3
    ! Inerpolation, for all T:
    ww = dexp(-T_u / T)
    Lam_s = Lam_s_hT * ww + Lam_s_lT * (1.d0 - ww)
    Lam_l = Lam_l_hT * ww + Lam_l_lT * (1.d0 - ww)
    ! Collisional frequencies:
    nu0 = 4.d0 * Z * Ef * a_f**2 / 3.d0 / pi / hb
    nu_s = nu0 * Lam_s
    nu_l = nu0 * Lam_l
    sigma = n_e * e**2 / (m_st * nu_s)
    lambda = pi**2 * kb**2 * T * n_e / (3.d0 * m_st * nu_l)
    nu_e_s = nu_s
    nu_e_l = nu_l
    if (debug.eq.1.2d0) print *, 'Exiting con_crust_e_phon_ion:', &
            ' sigma, lambda=', sigma, lambda
    return
end

! *********************************************************************
subroutine get_lam(s, w, Lam1, Lam2)
    implicit real*8(a-h, k-z)
    parameter(eps = 0.05d0)
    if ((s.le.eps).and.(s * w.le.eps)) then
        Lam1 = 0.5d0 * (exp_int(w) + dlog(w) + 0.5772156d0)
        Lam2 = (dexp(-w) - 1.0d0 + w) / (2.0d0 * w)
    else if (w.le.eps) then
        Lam1 = w * ((2.d0 * s + 1.d0) / (2.d0 * s + 2.d0) - s * dlog((s + 1.d0) / s))
        Lam2 = w * ((1.d0 - 3.d0 * s - 6.d0 * s**2) / (4.d0 * s + 4.d0) + &
                1.5d0 * dlog((s + 1.d0) / s))
    else if (w.gt.(1.d0 / eps)) then
        Lam1 = 0.5d0 * (dlog((s + 1.d0) / s) - 1.d0 / (s + 1.d0))
        Lam2 = (2.d0 * s + 1.d0) / (2.d0 * s + 2.d0) - s * dlog((s + 1.d0) / s)
    else
        Lam1 = dlog((s + 1.0d0) / s) + s / (s + 1.0d0) * (1.0d0 - dexp(-w)) - &
                (1.0d0 + s * w) * dexp(s * w) * (exp_int(s * w) - exp_int(s * w + w))
        Lam1 = 0.5d0 * Lam1
        Lam2 = (dexp(-w) - 1.0d0 + w) / w - s**2 / (s + 1.0d0) * (1.0d0 - dexp(-w)) - &
                2.0d0 * s * dlog((s + 1.0d0) / s) + &
                s * (2.0d0 + s * w) * dexp(s * w) * (exp_int(s * w) - exp_int(s * w + w))
        Lam2 = 0.5d0 * Lam2
    end if
    return
end
! *********************************************************************
function exp_int(x)
    !     CHECKED on Dec. 3, 2003 against table in Abramowitz & Stegun:
    !         accurate to 10^-5 for all x
    implicit real*8(a-h, k-z)
    if (x.le.0.d0) pause 'exp_int: x must be > 0 !'
    if (x.ge.1.0) then
        num = x**4 + 8.5733287401d0 * x**3 + 18.0590169730d0 * x**2 + &
                8.6347608925d0 * x + 0.2677737343d0
        den = x**4 + 9.5733223454d0 * x**3 + 25.6329561486d0 * x**2 + &
                21.0996530827d0 * x + 3.9584969228d0
        exp_int = num / den / (x * dexp(x))
    else
        exp_int = - 0.57721566d0 + 0.99999193d0 * x&
                - 0.24991055d0 * x**2 + 0.05519968d0 * x**3&
                - 0.00976004d0 * x**4 + 0.00107857d0 * x**5&
                - dlog(x)
    end if
    return
end

! *********************************************************************
! *********************************************************************
! *********************************************************************
!
!  This subroutine is from Oleg et al code !
!
!   ----------------------------------------------------
! The following block has been copied from "conrt.pas" (D.G.Yakovlev)
! and converted into Fortran. It realizes the SMOOTH COMPOSITION model
!                                                       Version 24.02.99
! Input: BARD (baryon density in fm^{-3}), Index (of phase)
! Output: Z (total number of protons inside the nucleus)
!   Anuc (num.of barions within the nucleus), A (within the cell)
!   tp (smoothness param.of proton core)
!   xnuc (effective proton-core radius divided by the WS cell radius)
!   xnuct (a second proton-core parameter for use in a quantum crystal)
! Internal variables:
! Nin=A-Z - total number of neutrons inside the nucleus
! Nfree - total number of free neutrons (incl.ones penetrating nuclei)
subroutine OYAFORM(BARD, Index, Z, Anuc, A, xnuc, xnuct)
    implicit double precision(A-Z)
    integer Index
    save
    parameter(PI = 3.14159265d0)
    SOyam(t, x) = x**3 - 9. * x**(3. + t) / (3. + t) + &
            9. * x**(3. + 2. * t) / (3. + 2. * t) - x**(3. + 3. * t) / (1. + t)
    ! The above factor corrects the volume for nucl.shape acc.to Oyamatsu:
    ! nn_in=nn_out+dn_n*(1-(r/Rn)^t); x=min(1,Rws/Rn); same for protons.
    if (Index.eq.30) then ! {densities lower than the neutron drip}
        f = dlog(1.d0 + BARD / 5.0d-9)
        Rp = 5.688 + 0.02628 * f + 0.009468 * f * f ! max.proton core radius
        Rn = 5.788 + 0.02077 * f + 0.01489 * f * f ! max.neutron core radius
        np_in = 0.0738 + 1.22e-4 * f - 1.641e-4 * f * f ! centr.num.dens.of protons

        nn_in = 0.0808 + 1.688e-4 * f + 9.439e-5 * f * f ! same for neutrons
        nn_out = 0.0
        tp = 6.d0
        tn = tp
        Nin = PI / .75 * Rn**3 * nn_in * SOyam(tn, 1.d0)
        Z = PI / .75 * Rp**3 * np_in * SOyam(tp, 1.d0)
        Anuc = Z + Nin ! {nucleons within a nucleus}
        A = Anuc
        Rws = (A * .75 / PI / BARD)**.333333
        if (Rws.lt.Rn) stop'OYAFORM: too large Rn for outer envelope!'
        aa = (A / BARD)**.333333 !  {cube size}
    elseif (Index.eq.3) then ! {spheres after drip}
        g = BARD * 100.
        f = dlog(g)
        Rws = 31.68 - 8.400 * f - 0.2380 * f * f + 0.1152 * f**3
        tn = 1. / (0.2027 + 0.004506 * g) ! param.of shape
        Rn = 9.406 + 1.481 * f + 0.4625 * f * f + 0.05738 * f**3
        dn_n = (9.761 - 1.322 * f - 0.5544 * f * f - 0.07624 * f**3) / 100. ! n-height
        Nin = PI / .75 * Rn**3 * dn_n * SOyam(tn, dmin1(1.d0, Rws / Rn))
        tp = 1. / (0.1558 + 2.225e-3 * g + 9.452e-4 * g * g)
        Rp = 8.345 + 0.7767 * f + 0.1333 * f * f + 0.008707 * f**3
        np_in = (4.040 - 1.097 * f - 0.0723 * f * f + 0.0225 * f**3) / 100.
        Z = PI / .75 * Rp**3 * np_in * SOyam(tp, dmin1(1.d0, Rws / Rp))
        Nfree = BARD * PI / .75 * Rws**3 - Z - Nin ! free neutrons outside+under nuc.
        nn_out = Nfree / (PI / .75 * Rws**3) ! free neutron density
        nn_in = nn_out + dn_n ! max.n-density
        A = Z + Nfree + Nin ! total num.of barions in the cell (A')
        Anuc = Z + Nin + Nfree * (Rn / Rws)**3 ! number of barions within Rn
        if(Rn.gt.Rws) Anuc = A
        aa = (A / BARD)**.333333 ! {cube size}
    else
        stop'OYAFORM: invalid Index'
    endif
    Rp0eff = (Z / PI * .75 / np_in)**.333333
    Rp2eff = Rp * dsqrt((1. - 15. / (5. + tp) + 15. / (5. + 2. * tp) - 5. / (5. + 3. * tp)) / &
            SOyam(tp, 1.d0))
    Rp1eff = Rp * (1. - 12. / (4. + tp) + 12. / (4. + 2. * tp) - 4. / (4. + 3. * tp)) / &
            SOyam(tp, 1.d0)
    Rp3eff = Rp * ((1. - 18. / (6. + tp) + 18. / (6. + 2. * tp) - 6. / (6. + 3. * tp)) / &
            SOyam(tp, 1.d0))**.333333
    xnuc = Rp2eff / Rws
    xnuct = xnuc * tp / (.6 + tp)
    return
end

! *********************************************************************
! *********************************************************************
!
! *********************************************************************
! *********************************************************************
! * From:                                                             *
! * - N.Itoh, S.Mitake, H.Iyetomi & S.Ichimaru, ApJ 273 (1983): 774   *
! * - S.Mitake, S.Ichimaru & N.Itoh, ApJ 277 (1984): 375              *
! * - D.Yakovlev, Sov.Astron. 31 (1987): 346                          *
! *********************************************************************
! * Calculate the conductivity due to electron-ion scattering         *
! * in the crust in the liquid phase.                                 *
! *********************************************************************
subroutine con_crust_e_ion_Itoh(T, rho, A, A1, Z, &
        sigma, lambda, debug, &
        nu_e_s, nu_e_l)
    ! *********************************************************************
    ! * Units are cgs-K                A = a_cell    A1= a_ion            *
    ! *********************************************************************
    ! * Checked on OCT. 19 1990 and again on JULY 21, 2000                *
    ! *********************************************************************
    implicit real*8 (a-h, k-z)
    real*8 im1tf, im1, ip1, ip3
    !       INCLUDE 'rho_limits.inc.f'
    parameter (pi = 3.14159265)
    dimension aliq(0:3), bliq(0:2), cliq(0:2), &
            dliq(0:3), eliq(0:2), fliq(0:2)
    data aliq/ 1.4453, -0.1561, 0.0941, -0.0263/
    data bliq/-1.5213, 0.8369, -0.4364/
    data cliq/ 0.6087, -3.1264, 1.8772/
    data dliq/ 0.4764, -0.0024, -0.0003, -0.0014/
    data eliq/-0.6640, 0.0656, -0.0346/
    data fliq/-0.5154, -0.1940, 0.0982/
    save aliq, bliq, cliq, dliq, eliq, fliq
    ! ****
    if (debug.eq.1.2d0) print *, 'Entering con_crust_e_ion_Itoh:', &
            ' T, rho=', T, rho
    ! ****
    rho6 = rho / 1.d6
    T8 = T / 1.d8
    gamma = 0.2275d0 * Z**2 / T8 * (rho6 / A)**(1. / 3.)
    x2 = 1.018d0 * (rho6 * Z / A)**(2. / 3.)
    y = 1.656d-2 / A1 / T8 * (rho6 * Z / A)**(2. / 3.)
    R = x2 / (1. + x2)
    rs = 1.388d-2 * (A / Z / rho6)**(1. / 3.)
    u = 0.45641d0 * dlog(gamma) - 1.31636d0
    ! ****** Classical contribution to the scattering integral s  *********
    sum1 = 1.d0 * aliq(0)
    sum2 = 1.d0 + bliq(0) * rs + cliq(0) * rs**2
    do j = 1, 2
        sum1 = sum1 + aliq(j) * (u**j)
        sum2 = sum2 + bliq(j) * u**j * rs + cliq(j) * u**j * rs**2
    end do
    sum1 = sum1 + aliq(3) * (u**3)
    sm1 = sum1 * sum2
    sm1 = (1.d0 / 3.d0) * dlog(Z / 26.d0) + sm1

    sum1 = 1.d0 * dliq(0)
    sum2 = 1.d0 + eliq(0) * rs + fliq(0) * rs**2
    do j = 1, 2
        sum1 = sum1 + dliq(j) * (u**j)
        sum2 = sum2 + eliq(j) * u**j * rs + fliq(j) * u**j * rs**2
    end do
    sum1 = sum1 + dliq(3) * (u**3)
    sp1 = sum1 * sum2
    sp1 = 0.5d0 - (Z / 26.d0)**(2. / 3.) * (0.5d0 - sp1)

    s_s = sm1 - R * sp1
    s_l = sm1 - R * sp1    ! To this point s_s=s_l
    ! *********************************************************************
    if (y .lt. .01) goto 200
    ! ****** Semiclassical corrections to s *******************************
    sp3 = 0.2493d0 - 0.1081d0 * rs
    rk2 = 9.291d-3 * dsqrt(1.d0 + 1.d0 / x2)
    im1tf = 0.5d0 * (dlog(1.d0 + 4.d0 / rk2) - 1.d0 / (1.d0 + 0.25d0 * rk2))
    im1 = im1tf * (1.0028d0 + 0.0896d0 * rs)
    ip1 = 0.4893d0 - 0.4573d0 * rs + 0.3429d0 * rs**2
    ip3 = 0.2484d0 - 0.0953d0 * rs

    ds_s = -2.d0 / 3.d0 * y * ((ip1 - R * ip3) + (sp1 - R * sp3))
    ds_l = ds_s + 2.d0 * y / (pi**2) * (3.d0 * im1 - (2.d0 + 3.d0 * R) * ip1 + 2. * R * ip3)

    !    WHY IS THIS COMMENTED OUT ? ! ? ! ?
    !        if (y .gt. 0.02d0) then
    !         s_s=s_s+ds_s
    !         s_l=s_l+ds_l
    !        else
    !         ds_s=(y-0.01d0)/0.01d0*ds_s
    !         s_s=s_s+ds_s
    !         ds_l=(y-0.01d0)/0.01d0*ds_l
    !         s_l=s_l+ds_l
    !        end if
    ! *********************************************************************
    200   continue
    ! ************** Correction from 2nd order Coulomb cross section ******
    ! ************** From Yakovlev, Sov.Astron. 31(1987): 346 *************
    mue = A / Z
    beta2 = 1.018d0 * (rho6 / mue)**(2. / 3.)&
            / (1.d0 + 1.018d0 * (rho6 / mue)**(2. / 3.))
    beta = dsqrt(beta2)
    alphab = 1.d0 / 137.d0 / beta * Z
    ds = pi / 2.d0 * alphab * beta2 * (1.d0 + 1.30d0 * alphab)&
            / (1.d0 + alphab**2 * (0.71d0 - 0.54d0 * beta2))
    s_s = s_s + ds
    s_l = s_l + ds
    ! *********************************************************************
    sigma = 8.693d21 * (rho6 / A) * (1.d0 - R) / s_s
    lambda = 2.363d17 * (rho6 * T8 / A) * (1.d0 - R) / s_l
    nu_e_l = (4.11 / 2.363) * 1.d16 * Z * sqrt(1.e0 + x2) * s_l
    nu_e_s = (4.11 / 2.363) * 1.d16 * Z * sqrt(1.e0 + x2) * s_s
    ! ****
    if (debug.eq.1.2d0) print *, 'Exiting con_crust_e_ion_Itoh:', &
            ' sigma,lambda=', sigma, lambda
    ! ****
    return
end

! *********************************************************************
! *********************************************************************
!
! *********************************************************************
! *********************************************************************
!$$$c From: ...
!$$$c *********************************************************************
!$$$c * Calculate the conductivity due to electron-impurity scattering    *
!$$$c * in the crust in the solid phase.                                  *
!$$$c *********************************************************************
!$$$      subroutine con_crust_e_imp_Itoh(T,rho,A,A1,Z,
!$$$     1                                sigma,lambda,debug,
!$$$     2                                nu_e_s,nu_e_l)
!$$$c *********************************************************************
!$$$c * Units are cgs-K                A = a_cell    A1= a_ion            *
!$$$c *********************************************************************
!$$$c * Checked on: ...                                                   *
!$$$c *********************************************************************
!$$$       implicit real*8 (a-h,k-z)
!$$$       real*8 im1tf,im1,ip1,ip3
!$$$       parameter (pi=3.14159265)
!$$$c ****
!$$$       if (debug.eq.1.2d0) print *,'Entering con_crust_e_imp_Itoh:',
!$$$     1     ' T, rho=',T,rho
!$$$c ****
!$$$       pause 'con_crust_e_imp_Itoh is empty !'
!$$$
!$$$       sigma =0.d0
!$$$       lambda=0.d0
!$$$
!$$$c ****
!$$$       if (debug.eq.1.2d0) print *,'Exiting con_crust_e_imp_Itoh:',
!$$$     1     ' sigma,lambda=',sigma,lambda
!$$$c ****
!$$$       return
!$$$      end
! *********************************************************************
! *********************************************************************
! *********************************************************************
! *********************************************************************
!                  Yakovlev & Urpin calculations
! *********************************************************************
! *********************************************************************
! *********************************************************************
! *********************************************************************
!
! *********************************************************************
! *********************************************************************
! * From Yakovlev & Urpin, Sov. Astron. 24 (1980): 303                *
! *********************************************************************
! * Calculates the conductivity due to electron-phonon scattering     *
! * in the crust in the crystalline phase.                            *
! *********************************************************************
subroutine con_crust_e_phonon_YU (T, rho, A, A1, Z, &
        sigma, lambda, debug, &
        nu_e_s, nu_e_l)
    ! *********************************************************************
    ! * Units are cgs-K                A = a_cell    A1= a_ion            *
    ! *********************************************************************
    ! * Checked on Dec. 6 1990                                            *
    ! *********************************************************************
    implicit real*8 (a-h, k-z)
    na = 6.022e23
    kb = 1.38e-16
    hb = 1.054e-27
    e = 4.803e-10
    c = 2.99792e10
    pi = 3.14159265
!    INCLUDE 'Physical_Constants.inc.f'
!    !        na=6.022e23
!    !        kb=1.38e-16
!    !        hb=1.054e-27
!    !        e=4.803e-10
!    !        c=2.99792e10
!    !        pi=3.14159265
!    ! ***************************
    mue = A / Z
    rho6 = rho / 1.d6
    nion = Na * rho / A1
    mion = A / Na

    theta = 2.4d6 * dsqrt(rho6 / mue * 2.d0 / mue)
    omegap = e * Z * dsqrt(4. * pi * nion / mion)
    x = theta / 0.45d0 / T

    phi0 = 13.d0 / dsqrt(1. + (theta / 3.46d0 / T)**2) / x**2
    phi2 = 13.d0 * (theta / 5.1d0 / T)**2 / (1. + (theta / 4.17d0 / T)**2)**1.5&
            * pi**2 / x**2

    beta2 = 1.02d0 * (rho6 / mue)**(2. / 3.)&
            / (1.d0 + 1.02d0 * (rho6 / mue)**(2. / 3.))
    L_Coulomb = 1.d0 / 3.d0 * dlog(4.d0 * Z) - beta2 / 2.d0

    beta = dsqrt(beta2)
    vf = beta * c
    nukap = (e**2 / hb / vf) * omegap * x * &
            ((2.d0 - beta2) * phi0 + &
                    (3.d0 * L_Coulomb - 1.d0 + beta2 / 2.d0) * phi2 / pi**2)

    lambda = 4.11d15 * (rho6 / mue) / dsqrt(1. + (rho6 / mue)**(2. / 3.)) * &
            (T / 1.d6) * (1.d16 / nukap)
    return
end
! *********************************************************************
! *********************************************************************
! * From Yakovlev & Urpin, Sov. Astron. 24 (1980): 303                *
! *********************************************************************
! * Calculates the conductivity due to electron-ion scattering        *
! * in the crust in the liquid phase.                                 *
! *********************************************************************
subroutine con_crust_e_ion_YU (T, rho, a, z, &
        sigma, lbdaei)
    ! *********************************************************************
    ! * Units are cgs-K                                                   *
    ! *********************************************************************
    ! * Checked on DEC. 6, 1990                                           *
    ! *********************************************************************
    implicit real*8 (a-h, k-z)
    !       INCLUDE 'rho_limits.inc.f'
    na = 6.022e23
    kb = 1.38e-16
    hb = 1.054e-27
    e = 4.803e-10
    c = 2.99792e10
    pi = 3.14159265
    ! ***************************
    mue = (1. / z) * a
    rho6 = rho / 1.e6

    beta2 = 1.02 * (rho6 / mue)**(2. / 3.)&
            / (1. + 1.02 * (rho6 / mue)**(2. / 3.))
    beta = dsqrt(beta2)
    gamma = 2.273e5 * z**2 * (rho / a)**(1. / 3.) / t
    lambda = 1. / 6. * dlog(3. * pi**2 / 2. * z**2) + &
            0.5 * dlog(1. + 2. / gamma) - beta2 / 2.
    ! ************* correction from 2nd order cross coulomb section *****
    !        alphab=1./137./beta*z
    !        lambda=lambda+pi/2.*alphab*beta2*(1.+1.30*alphab)
    !     1         /(1.+alphab**2*(0.71-0.54*beta2))
    ! *********************************************************************
    vf = beta * c
    nukap = lambda * z * dsqrt(1. + (rho6 / mue)**(2. / 3.)) / 5.65e-17

    lbdaei = 4.11e15 * (rho6 / mue) / dsqrt(1. + (rho6 / mue)**(2. / 3.)) * &
            (t / 1.e6) * (1.e16 / nukap)
    return
end
! *********************************************************************
! *********************************************************************
! * From Yakovlev & Urpin, Sov. Astron. 24 (1980): 303                *
! *********************************************************************
! * Calculates the conductivity due to electron-impurity scattering   *
! * in the crust in the crystalline phase.                            *
! *********************************************************************
subroutine con_crust_e_imp_YU (T, rho, A, A1, Z, Q, &
        sigma, lambda, debug, &
        nu_e_s, nu_e_l)
    ! *********************************************************************
    ! * Units are cgs-K                A = a_cell    A1= a_ion            *
    ! *********************************************************************
    ! * Checked on September 22, 2009                                     *
    ! *********************************************************************
    implicit real*8 (a-h, k-z)
    parameter(pi = 3.1415926535)
    parameter(hp = 1.0546d-27, c = 2.99792d10, kb = 1.3806d-16)
    parameter(mp = 1.6726d-24, me = 9.1095d-28, e = 4.803d-10)
    ! ****
    if (debug.eq.1.2d0) print *, 'Entering con_crust_e_imp_YU:', &
            ' T, rho, A, A1, Z, Q =', T, rho, A, A1, Z, Q
    ! ****
    x = 1.00884d0 * (Z / A * rho / 1.d6)**(1. / 3.)
    nu = 1.75d16 * dsqrt(1.d0 + x**2) * Q / Z&
            * Coulomb_imp_YU(x)
    lambda = 4.04d17 * x**3 / dsqrt(1.d0 + x**2) * (T / 1.d8) * (1.d16 / nu)
    !ccccccccccccccccccccc
    sigma = 1.d0
    nu_e_s = 1.d0
    nu_e_l = 1.d0
    !ccccccccccccccccccccc
    ! ****
    if (debug.eq.1.2d0) print *, 'Exiting con_crust_e_imp_YU'
    ! ****
    return
end

! *********************************************************************
! *********************************************************************
function Coulomb_imp_YU(x)
    implicit none
    real*8 Coulomb_imp_YU, x, beta, q
    beta = x / dsqrt(1.d0 + x**2)
    q = 0.048196d0 / dsqrt(beta)
    Coulomb_imp_YU = dlog(1.d0 / q) - 0.5d0 * (1.d0 + beta**2)
    return
end
! *********************************************************************
! *********************************************************************