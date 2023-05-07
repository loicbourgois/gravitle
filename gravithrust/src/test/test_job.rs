use crate::test::helpers::set_job_by_name;
use crate::test::helpers::setup_simulation;
#[test]
fn test_job() {
    let mut g = setup_simulation();
    for name in ["plasma_collector", "plasma_transporter"] {
        set_job_by_name(&mut g, name).unwrap();
    }
}
