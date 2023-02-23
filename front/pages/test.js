import { normalize } from "./math";


const test = () => {
  const functions = {
    normalize: normalize
  }
  const tests = []
  tests.push({
    f: 'normalize',
    i:[{x:0.7,y:0.8}]
  })
  tests.push({
    f: 'normalize',
    i:[{x:0.1,y:0.0}]
  })
  tests.push({
    f: 'normalize',
    i:[{x:0.1,y:0.1}]
  })
  for (let i = 0; i < tests.length; i++) {
      tests[i].o = functions[tests[i].f](...tests[i].i)
  }
  console.log(JSON.stringify(tests, null, 2))
}


export {
  test,
}
