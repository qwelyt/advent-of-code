const convertStringToArr = (string,char) => string.split(char);
const splitLines = string => convertStringToArr(string, "\n");
const splitComma = string => convertStringToArr(string, ",");
const convertInput = string => splitLines(string).map(s => splitComma(s));
const instruction = string => { return {"direction": string.substring(0,1), "amount": Number.parseInt(string.substring(1))}}

const arrInter = (a,b) => {
  let ret = [];
  const _A = a.map(_a => JSON.stringify(_a));
  const _B = b.map(_a => JSON.stringify(_a));
  let intersecting = _A.filter(n => _B.indexOf(n) > -1)
  return intersecting.map(i => JSON.parse(i))
}

function intersection(a,b){
  const A = Array.from(a.values());
  const B = Array.from(b.values());
  if(A.length > b.length) return arrInter(A,B);
  return arrInter(B,A);
}

function mapToThreads(arr){
  DX = {'L':-1, 'R':1, 'U':0, 'D':0};
  DY = {'L':0, 'R':0, 'U':1, 'D':-1};
  let _x = 0;
  let _y = 0;
  let length = 0;
  let ans = new Map();
  for(const command of arr){
    let cmd = instruction(command);
    for(let i=0; i<cmd.amount;++i){
      _x += DX[cmd.direction];
      _y += DY[cmd.direction];
      length++;
      const key = {x:_x,y:_y}
      if(!(key in ans)){
        ans.set(key, length);
      }
    }
  }
  return ans;
}

function solve(input){
  const inp = convertInput(input);
  const threads = inp.map(arr => mapToThreads(arr));
  const sets = threads.map(map => new Set(map.keys()))
  const intersect = intersection(sets[0], sets[1])
  const sum = intersect.map(o => Math.abs(o.x) + Math.abs(o.y))
  return Math.min(...sum);
}

// const input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
// const expected = 159;
// const result = solve(input);

// console.log("Result: ", result);
// console.log("Success: ", expected == result);

module.exports = {solve: solve};