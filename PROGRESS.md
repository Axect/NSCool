# PROGRESS

## 2019-12-11

### Main

* Wrap `tc`
    - [x] Tested (Weak)

### Tools

* C-Wrapper for `tc.f90` : `tc.c`
* New build file with `cc` : `build.rs`
* New dependency : `libc`
* New build-dependency : `cc`
* Rust wrapper for `tc.c` : `tc.rs`
* Should careful to wrap fortran code
    ```shell script
    gfortran -c tc.f90
    ```

## 2019-12-10

* Convert `tc_ioffe`
    - [x] Tested (Strong)

## 2019-12-02

* Convert `density`
    - [x] Tested (Weak)

## 2019-12-01

* Fix error of `P_electron`
    * `Row` based -> `Col` based
* Test`P_electron`
    - [x] Tested (Strong)
* Convert `pressure`
    - [x] Tested (Weak)

## 2019-11-30

* Convert `P_electron`
    - [x] Tested (Weak)

## 2019-11-29

* Convert several EOS `.dat` files to `.nc` (netcdf) file formats.