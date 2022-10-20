// @ts-check
'use strict'
const path = require('path')
const { LOCALHOST, tmpLedgerDir } = require('@metaplex-foundation/amman')
const PROGRAM_ID = require('./idl/tictactoe.json').metadata.address
const { accountProviders } = require('./dist/src/generated/accounts')

const localDeployPath = path.join(
  __dirname,
  '..',
  'program',
  'target',
  'deploy'
)

module.exports = {
  validator: {
    killRunningValidators: true,
    programs: [
      {
        label: 'TicTacToe Program',
        programId: PROGRAM_ID,
        deployPath: path.join(localDeployPath, 'tictactoe.so'),
      },
    ],
    jsonRpcUrl: LOCALHOST,
    ledgerDir: tmpLedgerDir(),
    resetLedger: true,
  },
  relay: {
    accountProviders,
  },
  storage: {
    enabled: false,
  },
}
