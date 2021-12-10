function inter_linkings(kinds_count, links, materials) {
  const l = `var<private> inter_linking: array<array<f32, ${kinds_count}>, ${kinds_count}> = array<array<f32, ${kinds_count}>, ${kinds_count}> (
${inter_linkings2(kinds_count, links, materials)}
  );`
  // console.log(l)
  return l
}
function inter_linkings2(kinds_count, links, materials) {
  let strs = []
  for (let material_k in materials ) {
    strs.push(inter_linkings_values(material_k, kinds_count, links, materials))
  }
  return strs.join('\n');
}
function inter_linkings_values(material_k, kinds_count, links, materials) {
  let weights_ = []
  for (let material_k_2 in materials ) {
    weights_.push(inter_linking_value(material_k, material_k_2, links))
  }
  return `    array<f32, ${kinds_count}> (${weights_.join(', ')}),  // ${material_k}`
}
function inter_linking_value(material_k, material_k_2, links) {
  for (let link of links) {
    if (  (link[0] == material_k && link[1] == material_k_2)
      ||  (link[0] == material_k_2 && link[1] == material_k)
    ) {
      return link[3].toFixed(1)
    }
  }
  return "0.0"
}
export {
  inter_linkings
}
