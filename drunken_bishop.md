```js
let abcs = ""
abcs += '\u{2500}' // horizontal light
//abcs += '\u{25AA}' // solid square
//abcs += '\u{2501}' // horizontal heavy
abcs += " " // space
//abcs += '\u{2550}' // horizontal double
abcs += '\u{25AC}' // fat box horizontal


abcs += '\u{25CB}' // empty circle
//abcs += '\u{25C7}' // empty diamond

//abcs += '\u{2542}' // big plus vertical heavy
//abcs += '\u{253C}' // big plus
abcs += '\u{2503}' // vertical bar

abcs += '\u{25CE}' // double circle
abcs += '\u{2592}' // half shade
abcs += '\u{25EB}' // split box
abcs += '\u{25AD}' // wide rec empty
abcs += '\u{25E0}' // upper curve
abcs += '\u{25E1}' // lower curve





abcs += '\u{2573}' // x

abcs += '\u{25CD}' // shaded circle
//abcs += '\u{2585}' // right bar

//abcs += '\u{2726}' // drawin 4
//abcs += '\u{2534}' // drawin 3
//abcs += '\u{251C}' // drawin 3
//abcs += '\u{2524}' // drawin 3
//abcs += '\u{252C}' // drawin 3
//abcs += '\u{2571}' // right slash
//abcs += '\u{2572}' // left slash
//abcs += '\u{2501}' // drawin
//abcs += '\u{2550}' // drawin
//abcs += '\u{2591}' // light shade
//abcs += '\u{25AB}' // empty square tiny

//abcs += '\u{25B5}' // empty triangle tiny
//



//abcs += '\u{271A}' // cross

//

abcs += '\u{25EC}' // dot triangle

abcs += '\u{2591}' // light shade

abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle
abcs += '\u{25CD}' // shaded circle


const directions = {
  "00": [-1, -1],
  "01": [1, -1],
  "10": [-1, 1],
  "11": [1, 1]
}

const h = 9
const w = 17

function hex2bin(hex) {
  return (parseInt(hex, 16).toString(2)).padStart(8, '0');
}

function generateGrid(fingerprint) {
  const cleanHex = fingerprint.replace(/:/g, '')
  const hexes = cleanHex.match(/.{2}/g) || []

  // Build forward move sequence
  const moves = []
  hexes.forEach(hex => {
    const binary = hex2bin(hex)
    const pairs = binary.match(/.{2}/g)
    pairs.forEach(pair => moves.push(pair))
  })

  console.log(moves)

  // Now alternate: 1 from forward, 1 from reverse, repeat
  let grid = Array(h).fill().map(() => Array(w).fill(0))
  let bishop = [Math.floor(w/2), Math.floor(h/2)]
  let totalMoves = 0

  for (let i = 0; i < moves.length; i++) {
    let direction = directions[moves[i]]
    bishop[0] = Math.max(0, Math.min(w-1, bishop[0] + direction[0]))
    bishop[1] = Math.max(0, Math.min(h-1, bishop[1] + direction[1]))

    grid[bishop[1]][bishop[0]] = Math.max(grid[bishop[1]][bishop[0]],0) + 1
    totalMoves++

    direction = directions[moves[moves.length-1 - i]]
    bishop[0] = Math.max(0, Math.min(w-1, bishop[0] + direction[1])) // flipped on purpose
    bishop[1] = Math.max(0, Math.min(h-1, bishop[1] + direction[0]))

    grid[bishop[1]][bishop[0]] = Math.max(grid[bishop[1]][bishop[0]],0) + 1
    totalMoves++
  }

  grid[bishop[1]][bishop[0]] = '\u2588'
  console.log(`Total moves: ${totalMoves}`)
  return grid
}



function renderGrid(grid, selector) {
  const gridText = "+-----------------+\n" +
    grid.map((row, index) =>
      "|" + row.map(el => {
        if (el == "S" || el == '\u{2588}') return el
        return abcs[Math.min(el, abcs.length - 1)] || ' '
      }).join('') + "|"
    ).join("\n") +
    "\n+-----------------+"

  document.querySelector(selector).innerHTML = gridText
}

function generateBoth() {
  const input1 = document.getElementById('input1').value
  const input2 = document.getElementById('input2').value

  const grid1 = generateGrid(input1)
  const grid2 = generateGrid(input2)

  renderGrid(grid1, '.grid1')
  renderGrid(grid2, '.grid2')
}

// Generate on load
generateBoth()

// Auto-generate on input change
document.getElementById('input1').addEventListener('input', generateBoth)
document.getElementById('input2').addEventListener('input', generateBoth)
  ```
