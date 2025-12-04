const a = require("./a.js");
const file = require('./read-input.js');

it('works', () => {
  expect(1).toBe(1);
})

describe('Solve given examples', () =>{
  it('1', () => {
    const input = "R8,U5,L5,D3\nU7,R6,D4,L4";
    const expected = 6;
    const result = a.solve(input);
    expect(result).toBe(expected);
  })
  it('2', () => {
    const input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    const expected = 159;
    const result = a.solve(input);
    expect(result).toBe(expected);
  })
  it('3', () => {
    const input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
    const expected = 135;
    const result = a.solve(input);
    expect(result).toBe(expected);
  })
})

it('solve a', () => {
  console.log(file);
  const input = file;
  const result = a.solve(input);
  console.log(result);
  expect(1).toBe(1);
})