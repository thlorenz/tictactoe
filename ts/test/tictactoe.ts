import { LOCALHOST } from '@metaplex-foundation/amman-client'
import { Connection, PublicKey, Transaction } from '@solana/web3.js'
import {
  createInitializeGameInstruction,
  createPlayerJoinInstruction,
  Game,
  GameState,
  InitializeGameInstructionAccounts,
  InitializeGameInstructionArgs,
  PlayerJoinInstructionAccounts,
} from '../src/generated'
import { pdaForGame } from '../src/tictactoe'
import test from 'tape'
import { amman, killStuckProcess, spokSamePubkey } from './utils'
import spok from 'spok'

killStuckProcess()

async function setupPlayer(label: string) {
  // 1. Create a connection to the local test validator
  const connection = new Connection(LOCALHOST, 'confirmed')

  // 2. Generate a keypair for the first player and have amman label it for us
  const [player, playerPair] = await amman.addr.genLabeledKeypair(label)
  await amman.airdrop(connection, player, 2)

  // 3. Create a transaction handler that will use the player as signer
  const transactionHandler = amman.payerTransactionHandler(
    connection,
    playerPair
  )

  return {
    transactionHandler,
    connection,
    player,
    playerPair,
  }
}

test('tx: init game and add player x', async (t) => {
  // 1. Setup the player and its transaction handler
  const {
    transactionHandler: playerXHandler,
    player: playerX,
    playerPair: playerXPair,
    connection,
  } = await setupPlayer('player x')

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

test.only('tx: join game to add player o', async (t) => {
  // NOTE: the repetition of the init game code here. We will address that in one of the next steps.

  const [game] = amman.addr.genKeypair()
  const gamePda = await pdaForGame(game)
  await amman.addr.addLabels({ gamePda })

  // 1. setup the players
  const {
    transactionHandler: playerXHandler,
    player: playerX,
    playerPair: playerXPair,
    connection,
  } = await setupPlayer('player x')

  const {
    transactionHandler: playerOHandler,
    player: playerO,
    playerPair: playerOPair,
  } = await setupPlayer('player o')

  {
    // 2. Initialize the game
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
  }

  {
    // 3. Join the game with player o
    const accounts: PlayerJoinInstructionAccounts = {
      playerO,
      gamePda,
    }
    const ix = createPlayerJoinInstruction(accounts)

    const tx = new Transaction().add(ix)
    await playerOHandler
      .sendAndConfirmTransaction(tx, [playerOPair], 'tx: join game')
      .assertSuccess(t, [/IX: player_join/])

    const gameAccount = await Game.fromAccountAddress(connection, gamePda)
    spok(t, gameAccount, {
      $topic: 'game',
      state: GameState.Full,
      playerX: spokSamePubkey(playerX),
      playerO: spokSamePubkey(playerO),
      board: spok.arrayElements(9),
    })
  }
  {
    // 4. Try to join another player
    const {
      transactionHandler: playerOHandler,
      player: playerOTwo,
      playerPair: playerOPairTwo,
    } = await setupPlayer('player o two')

    const accounts: PlayerJoinInstructionAccounts = {
      playerO: playerOTwo,
      gamePda,
    }
    const ix = createPlayerJoinInstruction(accounts)

    const tx = new Transaction().add(ix)
    await playerOHandler
      .sendAndConfirmTransaction(
        tx,
        [playerOPairTwo],
        'fail: joining game again'
      )
      .assertError(t)
  }
})
