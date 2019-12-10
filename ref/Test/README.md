# Test for NSCool (Fortran)

1. Include Module
    ```fortran
    module test_set
       contains
        include '../NSCool/Code/tc_Ioffe.f90'
    end module test_set
    ```
2. Compile with Include
    ```shell script
    gfortran -c test.f90 -I../NSCool/Code
    ```
3. Compile
    ```shell script
    gfortran test.f90
    ```  