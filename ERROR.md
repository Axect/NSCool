# Error in original NSCool

## Errata

* NSCool/Code/tc.f90 : Extra `&` at 882 line
    ```fortran
    data k0/+0.00E+00, +2.50E-01, +4.00E-01, +6.00E-01, &
                +7.00E-01, +8.20E-01, +9.00E-01, +1.00E+00, &
                +1.10E+00, +1.20E+00, +1.30E+00, +1.50E+00/&
    ```

* `I_Files/I_Neutrino_1.dat` : `INU_N1S0_BPF` -> `INU_N1S0_PBF`
    ```fortran
    1	INU_N1S0_BPF
    ```
* `NSCool/Code/conductivity_crust.f90` : No blank near `stop` at 625 line
    ```fortran
    if (Rws.lt.Rn) stop'OYAFORM: too large Rn for outer envelope!'
    ```
* `NSCool/Code/conductivity_crust.f90` : No blank near `stop` at 647 line
    ```fortran
    stop'OYAFORM: invalid Index'
    ```

## Path

* NSCool/Code/tc.f90 : homepage at 819 line
    ```fortran
    open(unit = 35, file = '/home/page/nstar/pierre/tc.dat', status = 'old')
    ```