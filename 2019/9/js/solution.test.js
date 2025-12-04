const computer = require('../../IntCodeComputer/js/IntCodeComputer.js')
const file = require("./read-input.js")


it('solve A', () => {
    const codes = file
    const result = computer.compute(codes,[1])
    const expected = [3063082071]
    // console.log(result)
    expect(result.output).toStrictEqual(expected)
})
it('solve B', () => {
    const codes = file
    const result = computer.compute(codes,[2])
    const expected = [81348]
    // console.log(result)
    expect(result.output).toStrictEqual(expected)
})


