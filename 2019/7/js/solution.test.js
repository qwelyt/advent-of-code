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

it('solve A', () => {
    const codes = file
    const result = app.solveA(codes)
    const expected = 67023
    expect(result).toBe(expected)
})

describe('solve given examples, B', () => {
  it('1', () => {
    const codes = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
    const phaseSettings = [9,8,7,6,5]
    const expected = 139629729
    const result = app.runContinious(codes, phaseSettings)
    expect(result).toBe(expected)
  })
  it('2', () => {
    const codes = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
    const phaseSettings = [9,7,8,5,6]
    const expected = 18216
    const result = app.runContinious(codes, phaseSettings)
    expect(result).toBe(expected)
  })
})

it('solve B', () => {
    const codes = file
    const result = app.solveB(codes)
    const expected = 7818398
    expect(result).toBe(expected)
})