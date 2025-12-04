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
	it('7', () => {
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
	it('8', () => {
		const input = "" +
			"A)B" +
			"\nB)C" +
			"\nC)D" +
			"\nD)E" +
			"\nE)F" +
			"\nB)G" +
			"\nG)H" +
			"\nD)I" +
			"\nE)J" +
			"\nJ)K" +
			"\nK)L" +
			"\nD)M" +
			"\nD)N" +
			"\nN)O" +
			"\nJ)P" +
			"\nJ)Q"
		const expected = 67
		const result = app.solveA(input)
		expect(result).toBe(expected)
	})
	it('9', () => {
		const input = "" +
			"A)B" +
			"\nAA)BB" +
			"\nB)C" +
			"\nB)D" +
			"\nB)E" +
			"\nE)F" +
			"\nF)G" +
			"\nC)H" +
			"\nC)J" +
			"\nAA)CC" +
			"\nCC)DD" +
			"\nCC)EE" +
			"\nDD)FF" +
			"\nFF)GG" +
			"\nEE)HH" +
			"\nEE)LL"
		const expected = 39
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
	expect(result).toBe(162439)
})

describe('Solve given examples, B', () => {
	it('1', () => {
		const input = ""+
			"COM)B"+
			"\nB)C"+
			"\nC)D"+
			"\nD)E"+
			"\nE)F"+
			"\nB)G"+
			"\nG)H"+
			"\nD)I"+
			"\nE)J"+
			"\nJ)K"+
			"\nK)L"+
			"\nK)YOU"+
			"\nI)SAN"
		const expected = 4
		const result = app.solveB(input, "YOU", "SAN")
		expect(result).toBe(expected)
	})
})

it('Solve B', () => {
	const input = file
	const result = app.solveB(input, "YOU", "SAN")
	console.log(result)
	expect(result).toBe(367)
})
