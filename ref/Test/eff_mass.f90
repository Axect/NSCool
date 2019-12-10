module test_set
    contains
    include './NSCool/Code/eff_mass.f90'
end module test_set


program test_eff_mass
    use test_set
    DO k=1,3
    write(*,*) mstn_awp_3(k * 1.d0)
    END DO

end program test_eff_mass
