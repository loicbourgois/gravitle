
function random_int(min, max) {
  return Math.floor(Math.random() * (max-min+1)) + min;
}

export {
  random_int
}
