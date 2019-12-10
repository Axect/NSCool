module test_set
    contains
    include '../NSCool/Code/tc_Ioffe.f90'
end module test_set

program test_tc_ioffe
    use test_set
    real*8 kf
    DO k=1,10
        kf = k * 1d0
        write(*,*) kf, Tc_Ioffe_1p(kf)
        write(*,*) kf, Tc_Ioffe_2p(kf)
        write(*,*) kf, Tc_Ioffe_3p(kf)
        write(*,*) kf, Tc_Ioffe_1ns(kf)
        write(*,*) kf, Tc_Ioffe_2ns(kf)
        write(*,*) kf, Tc_Ioffe_3ns(kf)
        write(*,*) kf, Tc_Ioffe_1nt(kf)
        write(*,*) kf, Tc_Ioffe_2nt(kf)
        write(*,*) kf, Tc_Ioffe_3nt(kf)
    END DO
end program test_tc_ioffe