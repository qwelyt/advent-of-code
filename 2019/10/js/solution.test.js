const app = require('./solution.js')
const file = require('./read-input.js')

describe('Solve given examples', () => {
it('1', () => {
  const map = ".#..#" +
            "\n....."+
            "\n#####"+
            "\n....#"+
            "\n...##"
  const expected = {
    coord: {
      x: 3
      , y: 4
    }
    , sees: 8
  }
  const result = app.solveA(map)
  expect(result).toStrictEqual(expected)
})

it('2', () => {
  const map = "......#.#.\n"+
  "#..#.#....\n"+
  "..#######.\n"+
  ".#.#.###..\n"+
  ".#..#.....\n"+
  "..#....#.#\n"+
  "#..#....#.\n"+
  ".##.#..###\n"+
  "##...#..#.\n"+
  ".#....####"
  const expected = {
    coord: {
      x: 5
      , y: 8
    }
    , sees: 33
  }
  const result = app.solveA(map)
  expect(result).toStrictEqual(expected)
})

it('3', () => {
  const map = "#.#...#.#."+
  "\n.###....#."+
  "\n.#....#..."+
  "\n##.#.#.#.#"+
  "\n....#.#.#."+
  "\n.##..###.#"+
  "\n..#...##.."+
  "\n..##....##"+
  "\n......#..."+
  "\n.####.###."
  const expected = {
    coord: {
      x: 1
      , y: 2
    }
    , sees: 35
  }
  const result = app.solveA(map)
  expect(result).toStrictEqual(expected)
})
it('4', () => {
  const map = ".#..#..###"+
"\n####.###.#"+
"\n....###.#."+
"\n..###.##.#"+
"\n##.##.#.#."+
"\n....###..#"+
"\n..#.#..#.#"+
"\n#..#.#.###"+
"\n.##...##.#"+
"\n.....#.#.."
  const expected = {
    coord: {
      x: 6
      , y: 3
    }
    , sees: 41
  }
  const result = app.solveA(map)
  expect(result).toStrictEqual(expected)
})
it('5', () => {
  const map = ".#..##.###...#######" +
  "\n##.############..##." +
  "\n.#.######.########.#" +
  "\n.###.#######.####.#." +
  "\n#####.##.#.##.###.##" +
  "\n..#####..#.#########" +
  "\n####################" +
  "\n#.####....###.#.#.##" +
  "\n##.#################" +
  "\n#####.##.###..####.." +
  "\n..######..##.#######" +
  "\n####.##.####...##..#" +
  "\n.#####..#.######.###" +
  "\n##...#.##########..." +
  "\n#.##########.#######" +
  "\n.####.#.###.###.#.##" +
  "\n....##.##.###..#####" +
  "\n.#.#.###########.###" +
  "\n#.#.#.#####.####.###" +
  "\n###.##.####.##.#..##" 
  const expected = {
    coord: {
      x: 11
      , y: 13
    }
    , sees: 210
  }
  const result = app.solveA(map)
  expect(result).toStrictEqual(expected)
})
})

it('Solve A', () => {
  const map = file
  const expected = {
    coord: {
      x: 20
      , y: 18
    }
    , sees: 280
  }
  const result = app.solveA(map)
  expect(result).toStrictEqual(expected)
})
it.only('Solve B', () => {
  const map = file
  const expected = {
    coord: {
      x: 20
      , y: 18
    }
    , sees: 280
  }
  const result = app.solveB(map, {x:20, y:18})
  console.log(result)
  const r = 100*result.x + result.y
  console.log(r)
  // expect(result).toStrictEqual(expected)
})