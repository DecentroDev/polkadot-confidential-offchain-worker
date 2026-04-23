#!/usr/bin/env node
/**
 * Simple HTTP API Server for Confidential Offchain Worker Demo
 * 
 * This server provides a test endpoint that requires API key authentication.
 * The offchain worker will fetch data from this server using the API key
 * stored in the blockchain's offchain local storage.
 */

const http = require('http');
const url = require('url');

const PORT = process.env.PORT || 8080;
const VALID_API_KEY = process.env.API_KEY || 'my_secret_api_key_12345';

const server = http.createServer((req, res) => {
  const timestamp = new Date().toISOString();
  const parsedUrl = url.parse(req.url, true);
  
  console.log(`[${timestamp}] ${req.method} ${req.url}`);
  
  // CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Authorization, Content-Type');
  
  // Handle CORS preflight
  if (req.method === 'OPTIONS') {
    res.writeHead(200);
    res.end();
    return;
  }
  
  // Extract API key from Authorization header
  const authHeader = req.headers['authorization'] || '';
  const apiKey = authHeader.replace('Bearer ', '').trim();
  
  console.log(`  Authorization: ${authHeader ? authHeader.substring(0, 20) + '...' : '(none)'}`);
  
  // Check API key
  if (apiKey !== VALID_API_KEY) {
    console.log(`  ❌ Invalid API key`);
    res.writeHead(401, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ 
      error: 'Unauthorized',
      message: 'Invalid or missing API key'
    }));
    return;
  }
  
  console.log(`  ✅ Valid API key`);
  
  // Route handling
  if (parsedUrl.pathname === '/api/data' || parsedUrl.pathname === '/basic-auth') {
    // Return some confidential data
    const responseData = {
      authenticated: true,
      message: 'Success! This is confidential data from the API.',
      timestamp: timestamp,
      data: {
        temperature: 23.5,
        humidity: 65,
        status: 'operational',
        secret_value: 'This data is protected by API key authentication'
      }
    };
    
    console.log(`  📤 Returning confidential data`);
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify(responseData, null, 2));
    
  } else if (parsedUrl.pathname === '/api/health') {
    // Health check endpoint (no auth required)
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ 
      status: 'healthy',
      timestamp: timestamp
    }));
    
  } else {
    // 404 for unknown routes
    res.writeHead(404, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ 
      error: 'Not Found',
      message: 'Available endpoints: /api/data, /api/health'
    }));
  }
});

server.listen(PORT, '0.0.0.0', () => {
  console.log('');
  console.log('╔═══════════════════════════════════════════════════════════╗');
  console.log('║   Confidential Offchain Worker - HTTP API Server         ║');
  console.log('╚═══════════════════════════════════════════════════════════╝');
  console.log('');
  console.log(`🌐 Server running on: http://0.0.0.0:${PORT}`);
  console.log(`🔑 API Key: ${VALID_API_KEY}`);
  console.log('');
  console.log('Available endpoints:');
  console.log(`  GET  /api/data        - Protected endpoint (requires API key)`);
  console.log(`  GET  /api/health      - Health check (no auth required)`);
  console.log('');
  console.log('Test with curl:');
  console.log(`  curl -H "Authorization: Bearer ${VALID_API_KEY}" http://localhost:${PORT}/api/data`);
  console.log('');
  console.log('Waiting for requests...');
  console.log('');
});

