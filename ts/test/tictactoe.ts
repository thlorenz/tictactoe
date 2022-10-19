import { LOCALHOST } from '@metaplex-foundation/amman-client'
import { Connection, Transaction } from '@solana/web3.js'
import { createInitializeGameInstruction } from 'src/generated'
import { pdaForGame } from 'src/tictactoe'
import test from 'tape'
import { amman, killStuckProcess } from './utils'

killStuckProcess()

async function setupPlayerX() {
  const connection = new Connection(LOCALHOST, 'confirmed')

  const [playerX, playerXPair] = await amman.addr.genLabeledKeypair('player x')
  await amman.airdrop(connection, playerX, 2)

  const transactionHandler = amman.payerTransactionHandler(
    connection,
    playerXPair
  )

  return {
    playerXHandler: transactionHandler,
    connection,
    playerX,
    playerXPair,
  }
}

test('tx: init game and add player x', async (t) => {
  const { playerXHandler, playerX, playerXPair } = await setupPlayerX()

  const [game] = amman.addr.genKeypair()
  const gamePda = await pdaForGame(game)
  await amman.addr.addLabels({ gamePda })

  const ix = createInitializeGameInstruction({
    playerX,
    gamePda,
  })

  const tx = new Transaction().add(ix)
  await playerXHandler
    .sendAndConfirmTransaction(tx, [playerXPair], 'tx: init game')
    .assertSuccess(t, [/IX: initialize_game/])
})
