const app = require("./solution.js")
const file = require('./read-input.js');

describe('Solve example input', () => {
	it('1', () => {
		const input = "1002,4,3,4,33"
		const expected = {
			arr: [1002, 4, 3, 4, 99]
			, output: []
		}
		const result = app.solveA(input)
		expect(result).toStrictEqual(expected)
	})
	it('2', () => {
		const input = "1101,100,-1,4,0"
		const expected = {
			arr: [1101, 100, -1, 4, 99]
			, output: []
		}
		const result = app.solveA(input)
		expect(result).toStrictEqual(expected)
	})
	it('3', () => {
		const input = "3,2,23"
		const expected = {
			arr: [3,2,1]
			, output: []
		}
		const result = app.solveA(input)
		expect(result).toStrictEqual(expected)
	})
	it('4', () => {
		const input = "4,1"
		const expected = {
			arr: [4, 1]
			, output: [1]
		}
		const result = app.solveA(input)
		expect(result).toStrictEqual(expected)
	})
})

describe('Parse instructions', () => {
	it('1002', () => {
		const input = "1002"
		const expected = {
			opCode: "02"
			, operation: app.multiply
			, param1: app.positionMode
			, param2: app.immediateMode
			, storePosition: app.immediateMode
			, length: 4
			, modifies: true
		}
		const result = app.parseInstruction(input)
		expect(result).toStrictEqual(expected)
	})
	it('11002', () => {
		const input = "11002"
		const expected = {
			opCode: "02"
			, operation: app.multiply
			, param1: app.positionMode
			, param2: app.immediateMode
			, storePosition: app.positionMode
			, length: 4
			, modifies: true
		}
		const result = app.parseInstruction(input)
		expect(result).toStrictEqual(expected)
	})
})

it.only('SolveA', () => {
	const input = file
	const result = app.solveA(input)
	console.log(result)
})
