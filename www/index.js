import { Engine, Color } from "../pkg"
import { memory } from "../pkg/tetris_wasm_bg"

const CELL_SIZE = 30
const GRID_COLOR = "#CCCCCC"

const WHITE = "#FFFFFF"
const RED = "#FF0000"
const BLUE = "#0000FF"

// Construct the universe, and get its width and height.
const engine = Engine.new()
const width = engine.width()
const height = engine.height()

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas")
canvas.height = (CELL_SIZE + 1) * height + 1
canvas.width = (CELL_SIZE + 1) * width + 1

const ctx = canvas.getContext("2d")

const drawGrid = () => {
  ctx.beginPath()
  ctx.strokeStyle = GRID_COLOR

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0)
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1)
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1)
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1)
  }

  ctx.stroke()
}

const getIndex = (row, column) => {
  return row * width + column
}

const drawCells = () => {
  const cellsPtr = engine.board()
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height)

  ctx.beginPath()

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col)

      const color = colorString(cells[idx])
      ctx.fillStyle = color

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      )
    }
  }

  ctx.stroke()
}

const colorString = (color) => {
  switch (color) {
    case Color.Red:
      return RED

    case Color.Blue:
      return BLUE

    default:
      return WHITE
  }
}

const pauseButton = document.getElementById("play-pause")
pauseButton.addEventListener("click", (event) => {
  let isNowPaused = engine.toggle_pause()
  pauseButton.textContent = isNowPaused ? "Play" : "Pause"
})
const resetButton = document.getElementById("reset")
resetButton.addEventListener("click", (event) => {
  engine.reset()
})

// Input
function keys(event) {
  switch (event.keyCode) {
    case 37:
      console.log("Left key is pressed.")
      engine.left()
      break
    case 38:
      console.log("Up key is pressed.")
      engine.up()
      break
    case 39:
      console.log("Right key is pressed.")
      engine.right()
      break
    case 40:
      console.log("Down key is pressed.")
      engine.down()
      break
  }
}

const container = document.getElementById("canvas-container")
container.addEventListener("keydown", keys, false)

// Prevent arrow keys scrolling
window.addEventListener(
  "keydown",
  (e) => {
    if (
      ["Space", "ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(
        e.code
      )
    ) {
      e.preventDefault()
    }
  },
  false
)

// Loop
const renderLoop = () => {
  engine.tick()
  const c = engine.tetrimino()

  drawGrid()
  drawCells()
  requestAnimationFrame(renderLoop)
}

// Main
drawGrid()
drawCells()
renderLoop()
