use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ValueOrReference {
    Value(f32),
    Reference(String),
}

// example:
// {
//     "density": 1.750,
//     "color": "#888"
// }
#[derive(Debug, Deserialize)]
pub struct MaterialDefinition {
    pub density: ValueOrReference,
    pub color: String,
}

// impl MaterialDefinition {
//     pub fn as_material(&self, materials: ) -> Material {
//         Material {
//             density: match self.density {
//                 ValueOrReference::value(v) => {
//                     v
//                 }
//                 ValueOrReference::reference => {

//                 }
//             },
//         }
//     }
// }
