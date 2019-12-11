module test_set
contains
include '../NSCool/Code/tc.f90'
end module test_set

program test_tc
    use test_set
    real*8 kf
    kf = 1d0
    write(*,*) kf, tcn1_sfb(kf)
end program test_tc