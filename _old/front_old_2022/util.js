import * as uuid from 'uuid';
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
function player_id() {
  return uuid.v4()
}
export {
  assert,
  last,
  len,
  player_id,
}
