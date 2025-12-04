const computer = require('./IntCodeComputer.js')

describe('Basic functions', () => {
  describe('Parse instructions', () => {
    it('1', () => {
      const instruction = 1
      const expected = {
        operation: computer.operations.add
        , opCode: "0" + instruction
        , paramModes: [computer.readMode.positionMode, computer.readMode.positionMode]
        , storeMode: computer.writeMode.positionMode
        , length: 4
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('2', () => {
      const instruction = 2
      const expected = {
        operation: computer.operations.multiply
        , opCode: "0" + instruction
        , paramModes: [computer.readMode.positionMode, computer.readMode.positionMode]
        , storeMode: computer.writeMode.positionMode
        , length: 4
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('3', () => {
      const instruction = 3
      const expected = {
        operation: computer.operations.write
        , opCode: "0" + instruction
        , paramModes: null
        , storeMode: computer.writeMode.positionMode
        , length: 2
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('4', () => {
      const instruction = 4
      const expected = {
        operation: computer.operations.read
        , opCode: "0" + instruction
        , paramModes: [computer.readMode.positionMode]
        , storeMode: null
        , length: 2
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('5', () => {
      const instruction = 5
      const expected = {
        operation: computer.operations.jumpIfTrue
        , opCode: "0" + instruction
        , paramModes: [computer.readMode.positionMode, computer.readMode.positionMode]
        , storeMode: null
        , length: 3
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('6', () => {
      const instruction = 6
      const expected = {
        operation: computer.operations.jumpIfFalse
        , opCode: "0" + instruction
        , paramModes: [computer.readMode.positionMode, computer.readMode.positionMode]
        , storeMode: null
        , length: 3
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('7', () => {
      const instruction = 7
      const expected = {
        operation: computer.operations.lessThan
        , opCode: "0" + instruction
        , paramModes: [computer.readMode.positionMode, computer.readMode.positionMode]
        , storeMode: computer.writeMode.positionMode
        , length: 4
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('8', () => {
      const instruction = 8
      const expected = {
        operation: computer.operations.equalTo
        , opCode: "0" + instruction
        , paramModes: [computer.readMode.positionMode, computer.readMode.positionMode]
        , storeMode: computer.writeMode.positionMode
        , length: 4
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('9', () => {
      const instruction = 9
      const expected = {
        operation: computer.operations.adjustPointer
        , opCode: "0" + instruction
        , paramModes: [computer.readMode.positionMode]
        , storeMode: null
        , length: 2
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('101', () => {
      const instruction = 101
      const expected = {
        operation: computer.operations.add
        , opCode: "01"
        , paramModes: [computer.readMode.immediateMode, computer.readMode.positionMode]
        , storeMode: computer.writeMode.positionMode
        , length: 4
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('1101', () => {
      const instruction = 1101
      const expected = {
        operation: computer.operations.add
        , opCode: "01"
        , paramModes: [computer.readMode.immediateMode, computer.readMode.immediateMode]
        , storeMode: computer.writeMode.positionMode
        , length: 4
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
    it('21101', () => {
      const instruction = 21101
      const expected = {
        operation: computer.operations.add
        , opCode: "01"
        , paramModes: [computer.readMode.immediateMode, computer.readMode.immediateMode]
        , storeMode: computer.writeMode.relativeMode
        , length: 4
      }
      const result = computer.parseInstruction(instruction)
      expect(result).toStrictEqual(expected)
    })
  })
})

describe('Day 2 instructions', () => {
  it('1', () => {
    const input = [1, 0, 0, 0, 99];
    const expected = [2, 0, 0, 0, 99];
    expect(computer.compute(input).arr).toStrictEqual(expected);
  })
  it('2', () => {
    const input = [2, 3, 0, 3, 99];
    const expected = [2, 3, 0, 6, 99];
    expect(computer.compute(input).arr).toStrictEqual(expected);
  })
  it('3', () => {
    const input = [2, 4, 4, 5, 99, 0];
    const expected = [2, 4, 4, 5, 99, 9801];
    expect(computer.compute(input).arr).toStrictEqual(expected);
  })
  it('4', () => {
    const input = [1, 1, 1, 4, 99, 5, 6, 0, 99];
    const expected = [30, 1, 1, 4, 2, 5, 6, 0, 99];
    expect(computer.compute(input).arr).toStrictEqual(expected);
  })
})
describe('Day 5 instructions', () => {
  it('1', () => {
    const input = "1002,4,3,4,33"
    const expected = {
      arr: [1002, 4, 3, 4, 99]
      , output: []
    }
    const result = computer.compute(input)
    expect(result).toStrictEqual(expected)
  })
  it('2', () => {
    const input = "1101,100,-1,4,0"
    const expected = {
      arr: [1101, 100, -1, 4, 99]
      , output: []
    }
    const result = computer.compute(input)
    expect(result).toStrictEqual(expected)
  })
  it('3', () => {
    const input = "3,2,23"
    const expected = {
      arr: [3, 2, 99]
      , output: []
    }
    const result = computer.compute(input, [99])
    expect(result).toStrictEqual(expected)
  })
  it('4', () => {
    const input = "4,1"
    const expected = {
      arr: [4, 1]
      , output: [1]
    }
    const result = computer.compute(input)
    expect(result).toStrictEqual(expected)
  })

  describe('Equality operators 7 and 8', () => {
    describe('3,9,8,9,10,9,4,9,99,-1,8, equal to 8, positional mode', () => {
      const codes = "3,9,8,9,10,9,4,9,99,-1,8"
      it('Equal to 8', () => {
        const input = 8
        const expected = {
          arr: [3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8]
          , output: [1]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
      it('Less than 8', () => {
        const input = 7
        const expected = {
          arr: [3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8]
          , output: [0]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
      it('greater than 8', () => {
        const input = 9
        const expected = {
          arr: [3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8]
          , output: [0]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
    })
    describe('3,9,7,9,10,9,4,9,99,-1,8, less than 8, positional mode', () => {
      const codes = "3,9,7,9,10,9,4,9,99,-1,8"
      it('Equal to 8', () => {
        const input = 8
        const expected = {
          arr: [3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8]
          , output: [0]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
      it('Less than 8', () => {
        const input = 7
        const expected = {
          arr: [3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8]
          , output: [1]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
      it('greater than 8', () => {
        const input = 9
        const expected = {
          arr: [3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8]
          , output: [0]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
    })
    describe('3,3,1108,-1,8,3,4,3,99, equal to 8, immediate mode', () => {
      const codes = "3,3,1108,-1,8,3,4,3,99"
      it('Equal to 8', () => {
        const input = 8
        const expected = {
          arr: [3, 3, 1108, 1, 8, 3, 4, 3, 99]
          , output: [1]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
      it('Less than 8', () => {
        const input = 7
        const expected = {
          arr: [3, 3, 1108, 0, 8, 3, 4, 3, 99]
          , output: [0]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
      it('greater than 8', () => {
        const input = 9
        const expected = {
          arr: [3, 3, 1108, 0, 8, 3, 4, 3, 99]
          , output: [0]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
    })
    describe('3,3,1107,-1,8,3,4,3,99, less than 8, immediare mode', () => {
      const codes = "3,3,1107,-1,8,3,4,3,99"
      it('Equal to 8', () => {
        const input = 8
        const expected = {
          arr: [3, 3, 1107, 0, 8, 3, 4, 3, 99]
          , output: [0]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
      it('Less than 8', () => {
        const input = 7
        const expected = {
          arr: [3, 3, 1107, 1, 8, 3, 4, 3, 99]
          , output: [1]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
      it('greater than 8', () => {
        const input = 9
        const expected = {
          arr: [3, 3, 1107, 0, 8, 3, 4, 3, 99]
          , output: [0]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
    })
  })

  describe('Jumpinstructions 5 and 6', () => {
    describe('Position mode', () => {
      const codes = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"
      it('Input 0', () => {
        const input = 0
        const expected = {
          arr: [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9]
          , output: [0]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
      it('Input 1', () => {
        const input = 1
        const expected = {
          arr: [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 1, 1, 1, 9]
          , output: [1]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
    })
    describe('Immediate mode', () => {
      const codes = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1"
      it('Input 0', () => {
        const input = 0
        const expected = {
          arr: [3, 3, 1105, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 0]
          , output: [0]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
      it('Input 1', () => {
        const input = 1
        const expected = {
          arr: [3, 3, 1105, 1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
          , output: [1]
        }
        const result = computer.compute(codes, input)
        expect(result).toStrictEqual(expected)
      })
    })
  })
})

describe('Day 9 instructions', () => {
  xit('1', () => {
    const codes = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
    const expected = {
      arr: [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
      , output: [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    }
    const result = computer.compute(codes)
    expect(result).toStrictEqual(expected)
  })
  it('2', () => {
    const codes = "1102,34915192,34915192,7,4,7,99,0"
    const expected = {
      arr: [1102, 34915192, 34915192, 7, 4, 7, 99, 1219070632396864]
      , output: [1219070632396864]
    }
    const result = computer.compute(codes)
    expect(result).toStrictEqual(expected)
  })
  it('3', () => {
    const codes = "104,1125899906842624,99"
    const expected = {
      arr: [104, 1125899906842624, 99]
      , output: [1125899906842624]
    }
    const result = computer.compute(codes)
    expect(result).toStrictEqual(expected)
  })
})