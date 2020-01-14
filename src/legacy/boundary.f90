function fteff_ZARho(Tb, Z, A, Rho, gs14)
    ! This is the Hernquist & Applegate: ApJ 287, 244 (1984), analytical
    ! boundary condition for an arbitrary density (their Equ (3.12).
    ! WARNING : Tb must be the local value, not the redshifted one !
    implicit real*8 (a-h, k-z)
    save print

    if (print==1.) goto 10
    print *, '-----------------------------------------------------'
    print *, 'Using Hernquist & Applegate boundary condition at  '
    print *, '  Rho =', Rho
    print *, '-----------------------------------------------------'
    print = 1.
    10   continue

    fteff_ZARho = gs14**0.25 * &
            (A**3 / Z**4 / 4.25d30 * Tb**6.5 / Rho**2)**0.25

end

! *********************************************************************
! *********************************************************************

function fteff_NT(Tb, gs14)

    ! this is the Nomoto & Tsuruta boundary condition at rho=1.e10 g/cm3

    ! WARNING : Tb must be the local value, not the redshifted one !

    implicit real*8 (a-h, k-z)
    dimension ltb(11), lts(11)

    data lts/5.698, 5.764, 5.943, 6.103, 6.228, &
            6.363, 6.493, 6.633, 6.773, 6.918, 7.068/
    data ltb/7.67, 7.75, 8.00, 8.25, 8.50, &
            8.75, 9.00, 9.25, 9.50, 9.75, 10.00/

    save lts, ltb
    save print
    !*********************
    if (print==1.) goto 10
    print *, '-------------------------------------------------'
    print *, 'Using Nomoto & Tsuruta boundary condition'
    print *, '-------------------------------------------------'
    print = 1.
    10   continue
    !*********************

    logtb = dlog10(Tb)

    if (logtb<=ltb(1)) then
        i1 = 1
    else if (logtb>=ltb(11)) then
        i1 = 10
    else
        do i = 1, 10
            if ((logtb>=ltb(i)).and.(logtb<=ltb(i + 1))) then
                i1 = i
                goto 100
            end if
        end do
        100    continue
    end if

    i2 = i1 + 1
    deltb = ltb(i2) - ltb(i1)
    w1 = (ltb(i2) - logtb) / deltb
    w2 = 1. - w1
    logts = lts(i1) * w1 + lts(i2) * w2
    logts = logts + 0.25 * dlog10(gs14)

    fteff_NT = 10.**logts
end

! *********************************************************************
! *********************************************************************

function fteff_GPE(Tb, gs14)

    ! this is the Gudmundsson, Pethick & Epstein, ApJ 259, L19 (1982)
    ! boundary condition at rho=1.e10 g/cm3

    ! WARNING : Tb must be the local value, not the redshifted one !

    implicit real*8 (a-h, k-z)
    save print

    if (print==1.) goto 10
    print *, '-------------------------------------------'
    print *, 'Using Gundmundsson et al boundary condition'
    print *, '-------------------------------------------'
    print = 1.
    10   continue

    Tb8 = Tb / 1.e8
    fteff_GPE = 0.87e6 * gs14**.25 * (Tb8)**0.55

end

! *********************************************************************
! *********************************************************************

function fteff_acc(Tb, eta, gs14)

    ! this is the boundary condition for accreted envelope
    ! From Potekhin, Chabrier & Yakovlev, A&A 323, 415 (1999)
    ! CHECKED on MARCH 13, 2001
    ! WARNING : Tb must be the local value, not the redshifted one !

    implicit real*8 (a-h, k-z)
    save print
    !*****
    if (print==1.) goto 10
    print *, '------------------------------------------'
    print *, 'Using accreted envelope boundary condition'
    print '(a15,1p1e10.2)', 'with   eta =', eta
    print *, '------------------------------------------'
    print = 1.
    10   continue
    !*****

    Tb9 = Tb / 1.d9
    Ts = sqrt(7.0d0 * Tb9 * sqrt(gs14))
    z = Tb9 - Ts / 1.d3
    t4_iron = gs14 * ((7.0d0 * z)**2.25 + (z / 3.0d0)**1.25)
    t4_wacc = gs14 * (18.1d0 * Tb9)**2.42
    if (eta>1.d-30) then
        a = (1.2d0 + (5.3d-6 / eta)**0.38) * Tb9**(5. / 3.)
        t4_acc = (a * t4_iron + t4_wacc) / (a + 1.0d0)
    else
        t4_acc = t4_iron
    end if

    fteff_acc = t4_acc**0.25 * 1.d6

end

! *********************************************************************
! *********************************************************************

function fteff_field_iron(Tb, bfield, gs14)

    ! this is the boundary condition for iron envelope with B field
    ! From Potekhin & Yakovlev, A&A 374 (2001), p. 213
    ! CHECKED on June 5, 2002
    ! WARNING : Tb must be the local value, not the redshifted one !
    ! WARNING: bfield is B at the magnetic pole
    ! WARNING: good only for Tb > 1e7 K

    implicit real*8 (a-h, k-z)
    save print
    !*****
    if (print==1.) goto 10
    print *, '------------------------------------------'
    print *, 'Using iron envelope with magnetic field:'
    print *, '(a15,1p1e10.2)', 'B_pole =', bfield
    print *, '------------------------------------------'
    print = 1.
    10   continue
    !*****
    eta = 0.d0
    f_zero = fteff_acc(Tb, eta, gs14)

    B12 = bfield / 1.d12
    T9 = Tb / 1.d9
    beta = 0.074d0 * dsqrt(B12) / T9**0.45
    a1 = 5059.d0 * T9**0.75 / &
            (1.d0 + 20.4d0 * T9**0.5 + 138.d0 * T9**1.5 + 1102.d0 * T9**2)**0.5
    a2 = 1484.d0 * T9**0.75 / &
            (1.d0 + 90.d0 * T9**1.5 + 125.d0 * T9**2)**0.5
    a3 = 5530.d0 / dsqrt(1.d0 - 0.4d0 * compactness) * T9**0.75 / &
            (1.d0 + 8.16d0 * T9**0.5 + 107.8d0 * T9**1.5 + 560.d0 * T9**2)**0.5

    ratio = (1.d0 + a1 * beta**2 + a2 * beta**3 + 0.007d0 * a3 * beta**4) / &
            (1.d0 + a3 * beta**2)

    fteff_field_iron = f_zero * ratio**0.25

end

!************************************************************************
!************************************************************************
!************************************************************************

SUBROUTINE SPLINT1(XA, YA, Y2A, IN, X, Y)
    use, intrinsic:: iso_fortran_env, only: stdin=>input_unit
    !************************************************************************
    ! Given the arrays XA and YA of length IN, which tabulate a function    *
    ! (with the XA(i)'s in order), and given the array Y2A, which is the    *
    ! output from SPLINE, calculate the cubic-spline interpolated value     *
    ! Y for a given value X.                                                *
    !                                                                       *
    ! From NUMERICAL RECIPES, p.89.                                         *
    !************************************************************************

    IMPLICIT REAL*8 (A-H, L-Z)
    DIMENSION XA(IN), YA(IN), Y2A(IN)

    KLO = 1
    KHI = IN
    1     IF (KHI - KLO>1) THEN
        K = (KHI + KLO) / 2
        IF(XA(K)>X) THEN
            KHI = K
        ELSE
            KLO = K
        END IF
        GOTO 1
    END IF

    H = XA(KHI) - XA(KLO)
    IF (H==0.) THEN
        print *, 'Bad XA input, push Enter to continue'
        read(stdin,*)
        RETURN
    END IF
    A = (XA(KHI) - X) / H
    B = (X - XA(KLO)) / H
    Y = A * YA(KLO) + B * YA(KHI)&
            + ((A**3 - A) * Y2A(KLO) + (B**3 - B) * Y2A(KHI)) * (H**2) / 6.

    RETURN

END

!************************************************************************
!************************************************************************
!************************************************************************

SUBROUTINE SPLINE1(X, Y, IN, YP1, YPN, Y2)

    !************************************************************************
    ! Given the arrays X and Y of length IN, which tabulate a function      *
    ! (with the XA(i)'s in order), and given values YP1 and YPN of the      *
    ! first derivative of the interpolating function at points 1 and N,     *
    ! this subroutine returns an array Y2 of length IN which contains the   *
    ! second derivatives of the interpolating function at the tabulated     *
    ! points X(i)'s.                                                        *
    !                                                                       *
    ! From NUMERICAL RECIPES, p.88                                          *
    !************************************************************************

    IMPLICIT REAL*8 (A-H, L-Z)
    PARAMETER (JMAX = 100)
    DIMENSION X(IN), Y(IN), Y2(IN), U(JMAX)

    IF (YP1>1.E30) THEN
        Y2(1) = 0.
        U(1) = 0.
    ELSE
        Y2(1) = -0.5
        U(1) = (3. / (X(2) - X(1))) * ((Y(2) - Y(1)) / (X(2) - X(1)) - YP1)
    ENDIF
    DO I = 2, IN - 1
        SIG = (X(I) - X(I - 1)) / (X(I + 1) - X(I - 1))
        P = SIG * Y2(I - 1) + 2.
        Y2(I) = (SIG - 1.) / P
        U(I) = (6. * ((Y(I + 1) - Y(I)) / (X(I + 1) - X(I)) - (Y(I) - Y(I - 1))&
                / (X(I) - X(I - 1))) / (X(I + 1) - X(I - 1)) - SIG * U(I - 1)) / P
    END DO

    IF (YPN>1.E30) THEN
        QN = 0.
        UN = 0.
    ELSE
        QN = 0.5
        UN = (3. / (X(IN) - X(IN - 1))) * (YPN - (Y(IN) - Y(IN - 1)) / (X(IN) - X(IN - 1)))
    END IF
    Y2(IN) = (UN - QN * U(IN - 1)) / (QN * Y2(IN - 1) + 1.)

    DO K = IN - 1, 1, -1
        Y2(K) = Y2(K) * Y2(K + 1) + U(K)
    END DO

    RETURN
END
