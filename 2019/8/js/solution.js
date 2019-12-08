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

module.exports = {
  solveA: solveA
}