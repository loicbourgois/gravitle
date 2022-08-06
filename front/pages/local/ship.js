const ship = {
  p1: 'core',
  p2: 'core',
  parts: [
    [0,1, 'armor'],
    [0,2, 'gun'],
    [0,3, 'armor'],
    [0,4, 'armor'],
    [0,5, 'armor'],
    [2,1, 'gun'],
    [7,1, 'armor'],
    [8,1, 'armor'],
    [6,5, 'armor'],
    [9,6, 'armor'],
    [8,9, 'armor'],
    [5,4, 'armor'],
    [5,13, 'booster'],
    [12,9, 'booster'],
    [4,3, 'armor'],
    [7,8, 'armor'],
  ],
  links: [
    [1,6],
    [6,9],
    [10,11],
  ],
  key_bindings: {
    'f': [14],
    'j': [15],
  },
}
const ship_2 = {
  p1: 'core',
  p2: 'armor',
  parts: [
    [0,1, 'armor'],
    [0,2, 'armor'],
    [0,3, 'armor'],
    [0,4, 'armor'],
    [0,5, 'armor'],
  ],
  links: [
    [1,6]
  ],
  key_bindings: {},
}

export {
  ship,
  ship_2,
}
