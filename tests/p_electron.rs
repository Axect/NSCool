extern crate peroxide;
extern crate nscool;

use peroxide::*;
use nscool::backend::density::P_electron;

#[test]
fn check_p_electron() {
    let mut empty_set = vec![];
    assert_eq!(P_electron(1f64, 1f64, &mut empty_set), 1.3801253325935195e-16)
}
