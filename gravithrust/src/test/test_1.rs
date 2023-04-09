use crate::models::model_2::MODEL_2;
use crate::parse_model;

#[test]
fn test_parse_model() {
    let _model = parse_model(MODEL_2, 0.005);
}
