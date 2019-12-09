const computer = require('./intcode-computer.js')

function solveA(codes){
    return computer.run(codes, [])
}


module.exports = {
    solveA: solveA
}
