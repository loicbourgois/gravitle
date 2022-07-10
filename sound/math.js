const cos_ = (x) => {
  return Math.cos(x*2*Math.PI) * 0.5 + 0.5
}


const sin_ = (x) => {
  return Math.sin(x) * 0.5 + 0.5
}


const fract = (x) => {
  return x - Math.trunc(x)
}


export {
  cos_,
  sin_,
  fract,
}
