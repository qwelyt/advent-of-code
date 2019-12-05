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

xit('SolveA', () => {
	const input = file
	const result = app.solveA(input)
	console.log(result)
})

describe('Solve example input for B', () => {
	describe('Equality operators 7 and 8', () => {
		describe('3,9,8,9,10,9,4,9,99,-1,8, equal to 8, positional mode', () => {
			const codes = "3,9,8,9,10,9,4,9,99,-1,8"
			it('Equal to 8', () => {
				const input = 8
				const expected = {
					arr: [3,9,8,9,10,9,4,9,99,1,8]
					, output: [1]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
			it('Less than 8', () => {
				const input = 7
				const expected = {
					arr: [3,9,8,9,10,9,4,9,99,0,8]
					, output: [0]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
			it('greater than 8', () => {
				const input = 9
				const expected = {
					arr: [3,9,8,9,10,9,4,9,99,0,8]
					, output: [0]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
		})
		describe('3,9,7,9,10,9,4,9,99,-1,8, less than 8, positional mode', () => {
			const codes = "3,9,7,9,10,9,4,9,99,-1,8"
			it('Equal to 8', () => {
				const input = 8
				const expected = {
					arr: [3,9,7,9,10,9,4,9,99,0,8]
					, output: [0]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
			it('Less than 8', () => {
				const input = 7
				const expected = {
					arr: [3,9,7,9,10,9,4,9,99,1,8]
					, output: [1]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
			it('greater than 8', () => {
				const input = 9
				const expected = {
					arr: [3,9,7,9,10,9,4,9,99,0,8]
					, output: [0]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
		})
		describe('3,3,1108,-1,8,3,4,3,99, equal to 8, immediate mode', () => {
			const codes = "3,3,1108,-1,8,3,4,3,99"
			it('Equal to 8', () => {
				const input = 8
				const expected = {
					arr: [3,3,1108,1,8,3,4,3,99]
					, output: [1]
				}
				const result = app.solveB(codes, input)
				console.log(result)
				expect(result).toStrictEqual(expected)
			})
			it('Less than 8', () => {
				const input = 7
				const expected = {
					arr: [3,3,1108,0,8,3,4,3,99]
					, output: [0]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
			it('greater than 8', () => {
				const input = 9
				const expected = {
					arr: [3,3,1108,0,8,3,4,3,99]
					, output: [0]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
		})
		describe('3,3,1107,-1,8,3,4,3,99, less than 8, immediare mode', () => {
			const codes = "3,3,1107,-1,8,3,4,3,99"
			it('Equal to 8', () => {
				const input = 8
				const expected = {
					arr: [3,3,1107,0,8,3,4,3,99]
					, output: [0]
				}
				const result = app.solveB(codes, input)
				console.log(result)
				expect(result).toStrictEqual(expected)
			})
			it('Less than 8', () => {
				const input = 7
				const expected = {
					arr: [3,3,1107,1,8,3,4,3,99]
					, output: [1]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
			it('greater than 8', () => {
				const input = 9
				const expected = {
					arr: [3,3,1107,0,8,3,4,3,99]
					, output: [0]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
		})
	})

	describe('Jumpinstructions 5 and 6', () =>{
		describe('Position mode', () => {
			const codes = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"
			it('Input 0', () => {
				const input = 0
				const expected = {
					arr: [3,12,6,12,15,1,13,14,13,4,13,99,0,0,1,9]
					, output: [0]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
			it('Input 1', () => {
				const input = 1
				const expected = {
					arr: [3,12,6,12,15,1,13,14,13,4,13,99,1,1,1,9]
					, output: [1]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
		})
		describe('Immediate mode', () => {
			const codes = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1"
			it('Input 0', () => {
				const input = 0
				const expected = {
					arr: [3,3,1105,0,9,1101,0,0,12,4,12,99,0]
					, output: [0]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
			it('Input 1', () => {
				const input = 1
				const expected = {
					arr: [3,3,1105,1,9,1101,0,0,12,4,12,99,1]
					, output: [1]
				}
				const result = app.solveB(codes, input)
				expect(result).toStrictEqual(expected)
			})
		})
	})
})

it.only('solve B', () =>{
	const codes = file
	const input = 5
	const result = app.solveB(codes, input)
	console.log(result)
})
