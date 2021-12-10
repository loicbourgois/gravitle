function assert(x) {
  if (!x) {
    console.error("failed condition");
  }
}
function len(x) {
  return x.length
}
function last(x) {
  if (len(x) > 0) {
    return x[len(x)-1]
  } else {
    return undefined
  }
}
export {
assert,
  last,
  len,
}
