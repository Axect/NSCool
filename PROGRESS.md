# PROGRESS

## 2020-03-03

* Finish `conductivity_crust`
    * Write `con_crust`
    * Should test!

## 2020-02-14

* Convert `conductivity_crust` (in process)
    * `legacy/conductivity_crust.f90` : Fixed original code
    * `legacy/conductivity_crust.c` : Wrapper
    * `src/backend/conductivity_crust.rs` : Wrapper & Hand written
## 2020-02-12

* Convert `accretion`
    - [ ] Tested
* Convert `conductivity` (Not yet)
* Convert `.inc` files
    * `pairing.inc/superfluid`
    * `profile_comp.inc/profile_comp`
    * `fermi.inc/fermi, effmass`

## 2020-01-15

* Convert whole `I_Files/*.dat` to `data/I_Files/*.toml`
    - [ ] Checked

## 2020-01-14 ~ 2020-01-15

### Main Process

* Convert `boundary`
    - [ ] Tested

### Sub Process

* Extract pure functions & subroutines of `boundary.f90` : `src/legacy/boundary.f90`
* C-wrapper for `src/legacy/boundary.f90` : `src/legacy/boundary.c`
* In `boundary.rs`, there are two kinds of functions
    * Use `extern`
        - [x] `fteff_ZARho`
        - [x] `fteff_NT`
        - [x] `fteff_GPE`
        - [x] `fteff_acc`
        - [x] `fteff_field_iron`
        - [x] `splint1`
        - [x] `spline1`
    * Hand written
        - [x] `fteff`
        - [ ] `fteff_table` (deprecated)
* New dependency
    * `toml` : for `I_files`

## 2019-12-14

* Convert `opacity`
    - [ ] Tested

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