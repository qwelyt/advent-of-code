const app = require("./solution.js")

describe('Solve given examples, A', () => {
    it('1', () => {
        const codes ="109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
        const expected = {
            arr: [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]
            , output: []
        }
        const result = app.solveA(codes)
        expect(result).toStrictEqual(expected)
    })
    it('2', () => {
        const codes = "1102,34915192,34915192,7,4,7,99,0"
        const expected = {
            arr: [1102,34915192,34915192,7,4,7,99,1219070632396864]
            , output: [1219070632396864]
        }
        const result = app.solveA(codes)
        expect(result).toStrictEqual(expected)
    })
    it.only('3', () => {
        const codes = "104,1125899906842624,99"
        const expected = {
            arr: [104,1125899906842624,99]
            , output: [1125899906842624]
        }
        const result = app.solveA(codes)
        expect(result).toStrictEqual(expected)
    })
})

describe('Solve basic things, A', () => {
    it('1', () => {
        const codes = "1002,4,3,4,33"
        const expected = {
            arr: [1002, 4, 3, 4 ,99]
            , output: []
        }
        const result = app.solveA(codes)
        expect(result).toStrictEqual(expected)
    })
})
