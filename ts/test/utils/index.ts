import test from 'tape'

export * from './amman'
export * from './asserts'

// Due to the web3.js Connection keeping a socket open our process
// can get stuck for a few seconds.
// Here we force the process to exit quickly when tests are done.
export function killStuckProcess() {
  test.onFinish(() => process.exit(0))
}
