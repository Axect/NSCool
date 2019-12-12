module test_set
contains
include '../NSCool/Code/tc.f90'
end module test_set

program test_tc
    use test_set
    real*8 kf
    kf = 1d0
        write(*,*) kf, tcn1_sfb(kf)
        write(*,*) kf, tcn1_ccdk(kf)
        write(*,*) kf, tcn1_wap(kf)
        write(*,*) kf, tcn1_GC(kf)
        write(*,*) kf, tcn1_gipsf(kf)
        write(*,*) kf, tcn1_t72(kf)
        write(*,*) kf, tcn1_ns(kf)
        write(*,*) kf, tcn1_t84(kf)
        write(*,*) kf, tcn1_ao(kf)
        write(*,*) kf, tcn1_ccks_var(kf)
        write(*,*) kf, tcn1_ccks_cbf(kf)
        write(*,*) kf, tcn1_awp_2(kf)
        write(*,*) kf, tcn1_awp_3(kf)
!        write(*,*) kf, tcn1_bbllp(kf)
        write(*,*) kf, tcn1_sclbl96(kf)
        write(*,*) kf, tcn1_sclbl96_pol(kf)
        write(*,*) kf, tcn3_hgrr(kf)
        write(*,*) kf, tcn3_ao(kf)
        write(*,*) kf, tcn3_t72(kf)
        write(*,*) kf, tcn3_t72_m1(kf)
        write(*,*) kf, tcn3_ao_m1(kf)
        write(*,*) kf, tcn3_bcll92(kf)
        write(*,*) kf, tcn3_eehjo96_nr(kf)
        write(*,*) kf, tcn3_eehjo96_r(kf)
        write(*,*) kf, tcp1_ccy_ms(kf)
        write(*,*) kf, tcp1_ccy_ps(kf)
        write(*,*) kf, tcp1_ccdk(kf)
        write(*,*) kf, tcp1_t73(kf)
        write(*,*) kf, tcp1_ns(kf)
        write(*,*) kf, tcp1_ao(kf)
        write(*,*) kf, tcp1_bcll92(kf)
        write(*,*) kf, tcp1_eeho(kf)
end program test_tc