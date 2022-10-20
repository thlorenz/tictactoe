import { LOCALHOST } from '@metaplex-foundation/amman-client'
import { Connection, PublicKey, Transaction } from '@solana/web3.js'
import {
  createInitializeGameInstruction,
  Game,
  GameState,
  InitializeGameInstructionAccounts,
  InitializeGameInstructionArgs,
} from '../src/generated'
import { pdaForGame } from '../src/tictactoe'
import test from 'tape'
import { amman, killStuckProcess, spokSamePubkey } from './utils'
import spok from 'spok'

killStuckProcess()

async function setupPlayerX() {
  // 1. Create a connection to the local test validator
  const connection = new Connection(LOCALHOST, 'confirmed')

  // 2. Generate a keypair for the first player and have amman label it for us
  const [playerX, playerXPair] = await amman.addr.genLabeledKeypair('player x')
  await amman.airdrop(connection, playerX, 2)

  // 3. Create a transaction handler that will use the first player as signer
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
  // 1. Setup the player and its transaction handler
  const { playerXHandler, playerX, playerXPair, connection } =
    await setupPlayerX()

  // 2. Generate public key for our game and derive the game PDA
  const [game] = amman.addr.genKeypair()
  const gamePda = await pdaForGame(game)
  // 3. Label the game PDA so we can identify it in the Amman Explorer
  await amman.addr.addLabels({ gamePda })

  // 4. Prepare and execute init game transaction
  const accounts: InitializeGameInstructionAccounts = {
    playerX,
    gamePda,
  }
  const args: InitializeGameInstructionArgs = {
    initializeGameArgs: { game },
  }

  const ix = createInitializeGameInstruction(accounts, args)

  const tx = new Transaction().add(ix)
  await playerXHandler
    .sendAndConfirmTransaction(tx, [playerXPair], 'tx: init game')
    .assertSuccess(t, [/IX: initialize_game/])

  // 5. Fetch the game state from the chain and assert each property is set to what we expect
  const gameAccount = await Game.fromAccountAddress(connection, gamePda)
  spok(t, gameAccount, {
    $topic: 'game',
    state: GameState.WaitingForOpponent,
    playerX: spokSamePubkey(playerX),
    playerO: spokSamePubkey(PublicKey.default),
    board: spok.arrayElements(9),
  })
})
