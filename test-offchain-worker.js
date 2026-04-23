#!/usr/bin/env node
/**
 * Test script for Confidential Offchain Worker
 * 
 * This script:
 * 1. Connects to the blockchain node
 * 2. Calls the do_something extrinsic to trigger the offchain worker
 * 3. Monitors the logs to see the offchain worker making authenticated HTTP requests
 */

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

async function main() {
  console.log('\n╔═══════════════════════════════════════════════════════════╗');
  console.log('║   Test Confidential Offchain Worker                      ║');
  console.log('╚═══════════════════════════════════════════════════════════╝\n');

  // Wait for crypto libraries to be ready
  await cryptoWaitReady();

  // Connect to the blockchain
  console.log('🔗 Connecting to blockchain at ws://localhost:9944...');
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });
  
  console.log('✅ Connected to blockchain!');
  console.log(`   Chain: ${await api.rpc.system.chain()}`);
  console.log(`   Version: ${await api.rpc.system.version()}`);
  
  // Create keyring and add Alice
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  console.log(`\n👤 Using account: ${alice.address}`);

  // Get current block number
  const header = await api.rpc.chain.getHeader();
  const blockNumber = header.number.toNumber();
  console.log(`📦 Current block: #${blockNumber}`);

  // Call the do_something extrinsic to trigger offchain worker
  console.log('\n🚀 Calling ocwPallet.doSomething extrinsic...');
  console.log('   This will trigger the offchain worker on the next block.\n');

  const unsub = await api.tx.ocwPallet
    .doSomething(blockNumber + 1)
    .signAndSend(alice, ({ status, events }) => {
      console.log(`📡 Transaction status: ${status.type}`);

      if (status.isInBlock) {
        console.log(`✅ Transaction included in block: ${status.asInBlock.toHex()}`);
        
        events.forEach(({ event }) => {
          const { section, method, data } = event;
          console.log(`   Event: ${section}.${method}`, data.toString());
        });
      }

      if (status.isFinalized) {
        console.log(`✅ Transaction finalized in block: ${status.asFinalized.toHex()}`);
        console.log('\n' + '═'.repeat(60));
        console.log('🎉 SUCCESS!');
        console.log('═'.repeat(60));
        console.log('\nThe offchain worker should now be processing...');
        console.log('Check the node logs to see:');
        console.log('  1. API key retrieval from offchain storage');
        console.log('  2. HTTP request to the API server with authentication');
        console.log('  3. Response from the API server\n');
        console.log('💡 Tip: Run the node with -lruntime::offchain=trace to see detailed logs\n');
        
        unsub();
        process.exit(0);
      }
    });

  // Timeout after 60 seconds
  setTimeout(() => {
    console.log('\n⏱️  Timeout waiting for transaction...');
    unsub();
    process.exit(1);
  }, 60000);
}

main().catch((error) => {
  console.error('❌ Error:', error);
  process.exit(1);
});

