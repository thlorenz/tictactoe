import { AmmanAccountRendererMap } from '@metaplex-foundation/amman-client'
import { Game } from './generated'

export * as accountProviders from './generated/accounts'

const BOARD_ITEM_FREE = 0
const BOARD_ITEM_X = 1
const BOARD_ITEM_O = 2

export const accountRenderers: AmmanAccountRendererMap = new Map([
  [Game, renderGame],
])

export function renderGame(game: Game) {
  const { board } = game
  const i = board.map(renderBoardItem)
  return `+---+---+---+
|${i[0]}|${i[1]}|${i[2]}|
+---+---+---+
|${i[3]}|${i[4]}|${i[5]}|
+---+---+---+
|${i[6]}|${i[7]}|${i[8]}|
+---+---+---+`
}

function renderBoardItem(item: number) {
  switch (item) {
    case BOARD_ITEM_FREE:
      return '   '
    case BOARD_ITEM_X:
      return ' X '
    case BOARD_ITEM_O:
      return ' O '
    default:
      throw new Error(`Unknown board item: ${item}`)
  }
}
