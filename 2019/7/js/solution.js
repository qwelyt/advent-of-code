// input: [codes] [phaseSettings]
// Run the codes for each phase
// The first input in each phase is the phaseSetting
// The secon input in each phase is the previouse phases output, 0 for the first phase
// Answer is the output from the last phase

const computer = require("./intcode-computer.js")

function run(codes, phaseSettings){
  let previousOutput = 0
  let outputs = []
  for(let i=0; i<phaseSettings.length; ++i){
    const out = computer.run(codes, [phaseSettings[i], previousOutput]).output[0]
    previousOutput = out
    outputs.push(out)
  }
  return previousOutput
}

function generateValues(codes){
  const settings = [0,1,2,3,4]
  const values = []
  const valueObject = {}
  for(const a of settings){
    for(const b of settings){
      if(b === a) continue
      for(const c of settings){
        if(c === a || c === b) continue
        for(const d of settings){
          if(d === a || d === b || d === c) continue
          for(const e of settings){
            if(e === a || e === b || e === c || e === d) continue
            const phaseSettings = [a,b,c,d,e]
            const value = run(codes, phaseSettings)
            values.push(value)
            valueObject[JSON.stringify(phaseSettings)] = value
          }
        }
      }
    }
  }
  const max = Math.max.apply(null, values)
  return {
    values: valueObject
    , max: max
  }

}

function solveA(codes){
  const g = generateValues(codes)
  return g.max
}

module.exports = {
  run: run
  , solveA: solveA
}