#!/usr/bin/env node
const { execSync } = require("child_process");
const path = require("path");

// Construct the path to the binary
const binaryPath = path.join(__dirname, "..", "target", "release", "npm-auto-git");

// Run the binary
execSync(binaryPath, { stdio: "inherit" });
