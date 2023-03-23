#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi
echo ">> Deploying contractokok"

# near create-account sub1.multiactionhup.testnet --masterAccount multiactionhup.testnet --initial-balance 50

near deploy sub1.multiactionhup.testnet --wasmFile ./target/wasm32-unknown-unknown/release/hello_near.wasm
