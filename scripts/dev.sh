#!/bin/sh
cargo run -p orchestrator &
cd ui && npm run dev
