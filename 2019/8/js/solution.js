const divideArr = (arr, n) => arr.map(() => arr.splice(0,n)).filter(b => b)
const mapD = data => data.trim().split('').map(s => Number.parseInt(s))
const mapData = (data,cols,rows) => divideArr(mapD(data), rows*cols)
const numberOf = (arr, find) => find.map(f => arr.filter(a => a == f).length)

function arrWithFewest(arr, n){
  let index = 0
  let lowest = undefined
  for(let i =0; i<arr.length;++i){
    const num = arr[i].filter(k => k == n).length
    if(lowest == undefined || num < lowest) {
      index = i
      lowest = num
    }
  }
  return arr[index]
}

function solveA(data, rows, cols, find, operation){
  const arr = mapData(data, rows, cols)
  const awf = arrWithFewest(arr, 0)
  const no = numberOf(awf, find)
  return no.reduce((a,c) => operation(a,c))
}

function solveB(data, rows, cols, operation){
  const arr = mapData(data, rows, cols).map(a => divideArr(a, cols))

  let result = [];
  for(let i=0;i<rows;++i) {
    result.push([])
    for(let j=0;j<cols;j++) result[i].push(2)
  }

  for(let layer = 0; layer<arr.length; ++layer){
    for(let row = 0; row<rows; ++row){
      for(let col = 0; col<cols; ++col){
        const a = arr[layer][row][col]
        if(a !== 2 && result[row][col]==2) {
          result[row][col] = a
        }
      }
    }
  }
  return result
}


    // const data = "0222112222120000"
    // const rows = 2
    // const cols = 2
    // const operation = (a,b) => a*b
    // const p = solveB(data, rows, cols, operation)

module.exports = {
  solveA: solveA
  , solveB: solveB
}