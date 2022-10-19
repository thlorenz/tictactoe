import test from 'tape'

export * from './amman'

export function killStuckProcess() {
  test.onFinish(() => process.exit(0))
}
