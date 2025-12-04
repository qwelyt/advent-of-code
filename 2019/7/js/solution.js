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

function runContinious(codes, phaseSettings){
  let outputs = []
  let previousOutput = 0
  let ampMem = {
    0: {arr: codes, pointer:0, output: [], inputs: [phaseSettings[0], 0], inputIndex:0}
    , 1: {arr: codes, pointer:0, output: [], inputs: [phaseSettings[1]], inputIndex:0} 
    , 2: {arr: codes, pointer:0, output: [], inputs: [phaseSettings[2]], inputIndex:0}
    , 3: {arr: codes, pointer:0, output: [], inputs: [phaseSettings[3]], inputIndex:0}
    , 4: {arr: codes, pointer:0, output: [], inputs: [phaseSettings[4]], inputIndex:0}
  }
  const amps = Object.keys(ampMem).length
  for(let i =0; i<amps;){
    const out = computer.runPause(ampMem[i].arr, ampMem[i].inputs, true, ampMem[i].pointer, ampMem[i].inputIndex)
    ampMem[i].arr = out.arr
    ampMem[i].pointer = out.pointer
    ampMem[i].output.concat[out.output]
    ampMem[i].inputIndex = out.inputIndex

    if(out.output[0] != undefined){
      previousOutput = out.output[0]
      outputs.push(previousOutput)
    }
    
    if(i == amps-1 && out.pointer !== undefined) i = 0
    else ++i
    if(i < amps) ampMem[i].inputs.push(previousOutput)
  }
  return previousOutput
}

function generateValues(codes, settings, f){
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
            const value = f(codes, phaseSettings)
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
  const g = generateValues(codes, [0,1,2,3,4], run)
  return g.max
}

function solveB(codes){
  const g = generateValues(codes, [5,6,7,8,9], runContinious)
  return g.max
}

module.exports = {
  run: run
  , runContinious, runContinious
  , solveA: solveA
  , solveB: solveB
}