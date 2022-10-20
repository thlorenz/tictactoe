import { PublicKey } from '@solana/web3.js'
import { PROGRAM_ID } from '../tictactoe'

export async function pdaForGame(game: PublicKey): Promise<PublicKey> {
  const [pda] = await PublicKey.findProgramAddress(
    [Buffer.from('tictactoe'), game.toBuffer()],
    PROGRAM_ID
  )
  return pda
}
