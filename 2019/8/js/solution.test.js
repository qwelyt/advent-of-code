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