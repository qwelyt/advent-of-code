const app = require('./solution.js')
const file = require('./read-input.js');

describe('Solve basics', () => {
	it('1', () => {
		const input = "A)B"
		const expected = 1
		const result = app.solveA(input)
		expect(result).toBe(expected)
	})
	it('2', () => {
		const input = "A)B\nB)C"
		const expected = 3
		const result = app.solveA(input)
		expect(result).toBe(expected)
	})
	it('3.1', () => {
		const input = "A)B\nB)C\nC)D"
		const expected = 6
		const result = app.solveA(input)
		expect(result).toBe(expected)
	})
	it('3.2', () => {
		const input = "A)B)\nB)C\nB)D"
		const expected = 5
		const result = app.solveA(input)
		expect(result).toBe(expected)
	})
	it('4', () => {
		const input = "A)B)\nB)C\nB)D\nC)F"
		const expected = 8
		const result = app.solveA(input)
		expect(result).toBe(expected)
	})
	it('5', () => {
		const input = "A)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nD)M\nD)N\nN)O"
		const expected = 55
		const result = app.solveA(input)
		expect(result).toBe(expected)
	})
	it('6', () => {
		const input = "" +
			"WGB)S14" +
			"\nWN4)27C" +
			"\n18L)M18" +
			"\n1HY)6ZP" +
			"\nTQ9)KQ6" +
			"\nHQ3)HH1" +
			"\nFLC)F1Z" +
			"\nD6R)ZPC" +
			"\n2VD)GK3"
		const expected = 9
		const result = app.solveA(input)
		expect(result).toBe(expected)
	})
	it.only('7', () => {
		const input = "" +
			"A)S14" +
			"\nA)27C" +
			"\nA)M18" +
			"\nA)6ZP" +
			"\nA)KQ6" +
			"\nA)HH1" +
			"\nA)F1Z" +
			"\nA)ZPC" +
			"\nA)GK3"
		const expected = 9
		const result = app.solveA(input)
		expect(result).toBe(expected)
	})
})

describe('Solve given examples, A', () => {
	it('1', () => {
		const input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"
		const expected = 42
		const result = app.solveA(input)
		expect(result).toBe(expected)
	})
})

it('Solve A', () => {
	const input = file
	const result = app.solveA(input)
	console.log(result)
})
