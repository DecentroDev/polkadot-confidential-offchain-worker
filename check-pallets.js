#!/usr/bin/env node
const { ApiPromise, WsProvider } = require('@polkadot/api');

(async () => {
  const api = await ApiPromise.create({ provider: new WsProvider('ws://localhost:9944') });
  
  console.log('\n📦 Available Pallets:\n');
  console.log('Transactions (api.tx.*):');
  Object.keys(api.tx).sort().forEach(pallet => {
    console.log(`  - ${pallet}`);
    if (pallet.toLowerCase().includes('ocw') || pallet.toLowerCase().includes('offchain') || pallet.toLowerCase().includes('template')) {
      console.log(`    Methods: ${Object.keys(api.tx[pallet]).join(', ')}`);
    }
  });
  
  process.exit(0);
})().catch(console.error);

