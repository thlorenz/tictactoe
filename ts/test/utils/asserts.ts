import { PublicKey } from '@solana/web3.js'
import { Specifications } from 'spok'

export function spokSamePubkey(
  a: PublicKey | string | null | undefined
): Specifications<PublicKey> {
  const keyStr = typeof a === 'string' ? a : a?.toString()
  const key = typeof a === 'string' ? new PublicKey(a) : a
  const same = (b: PublicKey | null | undefined) =>
    b != null && !!key?.equals(b)

  same.$spec = `spokSamePubkey(${keyStr})`
  same.$description = `${keyStr} equal`
  return same
}
