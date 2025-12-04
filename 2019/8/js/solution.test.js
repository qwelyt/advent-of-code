const app = require('./solution.js')
const file = require('./read-input.js')

describe('Solve given exampels, A', () => {
  it('1', () => {
    const data = "123456789012"
    const rows = 2
    const cols = 3
    const find1 = 1
    const find2 = 2
    const operation = (a,b) => a*b
    const expected = 1
    const result = app.solveA(data, rows, cols, [find1, find2], operation)
    expect(result).toBe(expected)
  })
})

describe('Solve basic examples, A', () => {
  it('1', () => {
    const data = "121212" + "012012"
    const rows = 2
    const cols = 3
    const operation = (a,b) => a*b
    const expected = 9
    const result = app.solveA(data, rows, cols, [1, 2], operation)
    expect(result).toBe(expected)
  })
  it('2', () => {
    const data = "000000"+"012345"
    const rows = 2
    const cols = 3
    const operation = (a,b) => a*b
    const expected = 1
    const result = app.solveA(data, rows, cols, [1, 2], operation)
    expect(result).toBe(expected)
  })
  it('3', () => {
    const data = "014702580369"+"976965463213"+"015903572486"
    const rows = 4
    const cols = 3
    const operation = (a,b) => a*b
    const expected = 1
    const result = app.solveA(data, rows, cols, [1, 2], operation)
    expect(result).toBe(expected)
  })
  it('4', () => {
    const data = "011202580369"+"976965463213"+"011113572286"
    const rows = 4
    const cols = 3
    const operation = (a,b) => a*b
    const expected = 1
    const result = app.solveA(data, rows, cols, [1, 2], operation)
    expect(result).toBe(expected)
  })
})

it('solve A', () => {
  const data = file
  const rows = 25
  const cols = 6
  const operation = (a,b) => a*b
  const expected = 1905
  const result = app.solveA(data, rows, cols, [1, 2], operation)
  expect(result).toBe(expected)
})

describe('Solve given examples, B', () => {
  it('1', () => {
    const data = "0222112222120000"
    const rows = 2
    const cols = 2
    const operation = (a,b) => a*b
    const expected = [[0,1], [1,0]]
    const result = app.solveB(data, rows, cols, operation)
    expect(result).toStrictEqual(expected)
  })
})
describe('Solve basic examples, B', () => {
  it('1', () => {
    const data = "121" +
                 "212" + 
                 ""+
                 "012"+
                 "012"
    const rows = 2
    const cols = 3
    const operation = (a,b) => a*b
    const expected = [[1,1,1],[0,1,2]]
    const result = app.solveB(data, rows, cols, operation)
    expect(result).toStrictEqual(expected)
  })
  it('2', () => {
    const data = "22220" +
                 "12222" + 
                 "22222"+
                 ""+
                 "22222"+
                 "01222"+
                 "00221" +
                 "" +
                 "00122" +
                 "12120" +
                 "22001" +
                 "" +
                 "01201" +
                 "21001" +
                 "02102"
    const rows = 3
    const cols = 5
    const operation = (a,b) => a*b
    const expected = [[0,0,1,0,0],[1,1,1,0,0], [0,0,0,0,1]]
    const result = app.solveB(data, rows, cols, operation)
    expect(result).toStrictEqual(expected)
  })
})

it('solve B', () => {
  const data = file
  const rows = 6
  const cols = 25
  const operation = (a,b) => a*b
  const expected = [
    [0,1,1,0,0,0,1,1,0,0,1,0,0,1,0,1,1,1,0,0,1,1,1,1,0]
    , [1,0,0,1,0,1,0,0,1,0,1,0,1,0,0,1,0,0,1,0,0,0,0,1,0]
    , [1,0,0,1,0,1,0,0,0,0,1,1,0,0,0,1,0,0,1,0,0,0,1,0,0]
    , [1,1,1,1,0,1,0,0,0,0,1,0,1,0,0,1,1,1,0,0,0,1,0,0,0]
    , [1,0,0,1,0,1,0,0,1,0,1,0,1,0,0,1,0,0,0,0,1,0,0,0,0]
    , [1,0,0,1,0,0,1,1,0,0,1,0,0,1,0,1,0,0,0,0,1,1,1,1,0]
  ]
  const result = app.solveB(data, rows, cols, operation)

  const strings = []
  for(let row = 0; row<rows; ++row){
    let str = ""
    for(let col = 0; col<cols; ++col){
      const a = result[row][col]
      const s = a == 0 ? " " : "#"
      str += s
    }
    strings.push(str)
  }
  const concated = strings.reduce((a,c) => a+"\n"+c)
  console.log(concated)
  expect(result).toStrictEqual(expected)
})