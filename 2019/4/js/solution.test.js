const solver = require("./solution.js");
//const file = require("./read-input.js");
const file = "284639-748759";

describe('Verify examples, A', () => {
	it('1', () => {
		const input = 111111;
		const result = solver.verify(input);
		expect(result).toBe(true);
	})
	it('2', () => {
		const input = 112350;
		const result = solver.verify(input);
		expect(result).toBe(false);
	})
	it('3', () => {
		const input = 123789;
		const result = solver.verify(input);
		expect(result).toBe(false);
	})
})

describe('Test repetition', () =>{
	const f = solver.hasRepetition;

	it('112345 has repetition', () =>{
		const input = 112345;
		const expected = true;
		const result = f(input);
		expect(result).toBe(expected);
	})
	it('123789 has no repetition', () =>{
		const input = 123789;
		const expected = false;
		const result = f(input);
		expect(result).toBe(expected);
	})
})
describe('Test sequenc', () =>{
	const f = solver.decresingSequence;

	it('112345 is increasing sequence', () =>{
		const input = 112345;
		const expected = false;
		const result = f(input);
		expect(result).toBe(expected);
	})
	it('112350 has decreasing sequence', () =>{
		const input = 112350;
		const expected = true;
		const result = f(input);
		expect(result).toBe(expected);
	})
})
describe('Test verify', () =>{
	const f = solver.verify;

	it('112345 is correct', () =>{
		const input = 112345;
		const expected = true;
		const result = f(input);
		expect(result).toBe(expected);
	})
	it('112350 is incorrect', () =>{
		const input = 112350;
		const expected = false;
		const result = f(input);
		expect(result).toBe(expected);
	})
})

it('Test basic range', () => {
	const input = "100000-111111";
	const result = solver.anyGroup(input);
	const expected = [111111];
	expect(result).toStrictEqual(expected);
})

it('Solve A', () => {
	const input = file;
	const result = solver.anyGroup(input);
	//console.log("Result:", result);
	console.log("Answer A:",result.length);
	expect(result.length).toBe(895);
})

describe('Verify examples, B', () => {
	const f = solver.verifyWithLimits;

	it('1', () => {
		const input = 112233;
		const result = f(input);
		expect(result).toBe(true);
	})
	it('2', () => {
		const input = 123444;
		const result = f(input);
		expect(result).toBe(false);
	})
	it('3', () => {
		const input = 111122;
		const result = f(input);
		expect(result).toBe(true);
	})
})

it('Solve B', () => {
	const input = file;
	const result = solver.groupLimits(input);
	//console.log("Result:", result);
	console.log("Answer B:",result.length);
	expect(result.length).toBe(591);
})
