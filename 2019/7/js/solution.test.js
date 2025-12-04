const app = require('./solution.js')
const file = require('./read-input.js')

describe('solve given examples, A', () => {
  it('1', () => {
    const codes = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
    const phaseSettings = [4,3,2,1,0]
    const expected = 43210
    const result = app.run(codes, phaseSettings)
    expect(result).toBe(expected)
  })
  it('2', () => {
    const codes = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
    const phaseSettings = [0,1,2,3,4]
    const expected = 54321
    const result = app.run(codes, phaseSettings)
    expect(result).toBe(expected)
  })
  it('3', () => {
    const codes = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
    const phaseSettings = [1,0,4,3,2]
    const expected = 65210
    const result = app.run(codes, phaseSettings)
    expect(result).toBe(expected)
  })
})

// it.only('Oddly high', () => {
//     const codes = file
//     const phaseSettings = [3,0,0,0,0]
//     const result = app.run(codes, phaseSettings)
//     const expected = 0
//     expect(result).toBe(expected)
// })

it.only('solve A', () => {
    const codes = file
    const result = app.solveA(codes)
    const expected = 0
    expect(result).toBe(expected)
})