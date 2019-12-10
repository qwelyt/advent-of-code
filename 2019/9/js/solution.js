const computer = require('../../IntCodeComputer/js/IntCodeComputer.js')

function solveA(codes) {
     computer.compute(codes, [1])
}


module.exports = {
    solveA: solveA
}
