const a = require("./a.js");
const file = require('./read-input.js');

it('works', () => {
  expect(1).toBe(1);
})

it('should find operands', () =>{
  const input = [1,2,0,4];
  let o1 = a.operand(input,0,1);
  let o2 = a.operand(input,0,2);
  expect(o1).toBe(0);
  expect(o2).toBe(1);
})

it('should find operands 2', () =>{
  const input = [1,0,0,0,99];
  let o1 = a.operand(input,0,1);
  let o2 = a.operand(input,0,2);
  expect(o1).toBe(1);
  expect(o2).toBe(1);
})



describe('should solve given examples', () => {
  it('1', () => {
    const input = [1,0,0,0,99];
    const expected = [2,0,0,0,99];
    expect(a.process(input)).toStrictEqual(expected);
  })
  it('2', () => {
    const input = [2,3,0,3,99];
    const expected = [2,3,0,6,99];
    expect(a.process(input)).toStrictEqual(expected);
  })
  it('3', () => {
    const input = [2,4,4,5,99,0];
    const expected = [2,4,4,5,99,9801];
    expect(a.process(input)).toStrictEqual(expected);
  })
  it('4', () =>{
    const input = [1,1,1,4,99,5,6,0,99];
    const expected = [30,1,1,4,2,5,6,0,99];
    expect(a.process(input)).toStrictEqual(expected);
  })
})


it('Should calculate based on input file', () => {
  const input = file.split(',').map(s => Number.parseInt(s));
  const result = a.process(input);
  console.log("Answer: ", ""+result[0]);
  expect(1).toBe(1);
})

it('solve b', () => {

  const input = file.split(',').map(s => Number.parseInt(s));
  let solved = false;
  let result = [];
  for(let noun = 0; noun < 100; noun++){
    input[1] = noun;
    for(let verb = 0; verb < 100; verb++){
      input[2] = verb;
      result = a.process(input);
      if(result[0] == 19690720){
        solved = true;
        break;
      }
    }
    if(solved){
      break;
    }
  }
  const obj = {noun:input[1], verb:input[2]};
  console.log(obj)
  console.log("Answer:", 100 * obj.noun + obj.verb);
  expect(result[0]).toBe(19690720);

})