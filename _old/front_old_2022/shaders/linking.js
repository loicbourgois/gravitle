function linkings(kinds_count, links, materials) {
  const l = `var<private> linking: array<array<f32, ${kinds_count}>, ${kinds_count}> = array<array<f32, ${kinds_count}>, ${kinds_count}> (
${linkings2(kinds_count, links, materials)}
  );`
  return l
}
function linkings2(kinds_count, links, materials) {
  let strs = []
  for (let material_k in materials ) {
    strs.push(weights(material_k, kinds_count, links, materials))
  }
  return strs.join('\n');
}
function weights(material_k, kinds_count, links, materials) {
  let weights_ = []
  for (let material_k_2 in materials ) {
    weights_.push(weight(material_k, material_k_2, links))
  }
  return `    array<f32, ${kinds_count}> (${weights_.join(', ')}),  // ${material_k}`
}
function weight(material_k, material_k_2, links) {
  for (let link of links) {
    if (  (link[0] == material_k && link[1] == material_k_2)
      ||  (link[0] == material_k_2 && link[1] == material_k)
    ) {
      return link[2].toFixed(1)
    }
  }
  return "0.0"
}
export {
  linkings
}
