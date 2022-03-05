// #[test]
// use crate::plan::ab_plan_to_link_plan;
// #[test]
// use crate::plan::dna_to_link_plan;
// #[test]
// use crate::plan::dna_to_str;
// // #[test]
// // use crate::plan::get_plan_3;
// // #[test]
// // use crate::plan::get_plan_4;
// // #[test]
// // use crate::plan::get_plan_6;
// #[test]
// use crate::plan::get_plan_planner;
// #[test]
// use crate::plan::get_plan_pusher;
// #[test]
// use crate::plan::link_plan_to_ab_plan;
// #[test]
// use crate::plan::link_plan_to_dna;
// #[test]
// use crate::plan::mutate_dna_inplace;
// #[test]
// use crate::plan::get_plan_pyra;
//
// // #[test]
// // fn test_get_plan_pusher() {
// //     let plan = get_plan_pusher();
// //     let dna = plan_to_dna(&plan);
// //     let new_plan = dna_to_plan(&dna);
// //     let new_dna = plan_to_dna(&new_plan);
// //     assert_eq!(format!("{:?}",plan), format!("{:?}",new_plan));
// //     assert_eq!(dna, new_dna);
// // }
//
// // #[test]
// // fn test_get_plan_planner() {
// //     let plan = get_plan_planner();
// //     let dna = plan_to_dna(&plan);
// //     let new_plan = dna_to_plan(&dna);
// //     let new_dna = plan_to_dna(&new_plan);
// //     assert_eq!(format!("{:?}",plan), format!("{:?}", new_plan));
// //     assert_eq!(dna, new_dna);
// // }
//
// #[test]
// fn test_ab_plan_to_link_plan() {
//     for ab_plan in [
//         // get_plan_3(),
//         // get_plan_4(),
//         // get_plan_6(),
//         get_plan_pusher(),
//         get_plan_planner(),
//         get_plan_pyra(),
//     ] {
//         let link_plan = ab_plan_to_link_plan(&ab_plan);
//         let ab_plan_new = link_plan_to_ab_plan(&link_plan);
//         assert_eq!(format!("{:?}", ab_plan), format!("{:?}", ab_plan_new));
//         let dna = link_plan_to_dna(&link_plan);
//         let link_plan_new = dna_to_link_plan(&dna);
//         let dna_new = link_plan_to_dna(&link_plan_new);
//         assert_eq!(dna, dna_new);
//     }
// }
//
// #[test]
// fn test_mutation() {
//     let link_plan = ab_plan_to_link_plan(&get_plan_pusher());
//     let mut dna = link_plan_to_dna(&link_plan);
//     println!("{}", dna_to_str(&dna));
//     for _ in 0..1000 {
//         mutate_dna_inplace(&mut dna);
//         println!("{}", dna_to_str(&dna));
//         let link_plan_tmp = dna_to_link_plan(&dna);
//     }
// }
