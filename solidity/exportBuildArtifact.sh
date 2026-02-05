#!/bin/sh

# set script location as working directory
cd "$(dirname "$0")"

# Define the artifacts directory
artifactsDir="./artifacts/build-info"
# Define the output file
outputFileJson="./dist/buildArtifact.json"
outputFileJs="./dist/buildArtifact.js"
outputFileTsd="./dist/buildArtifact.d.ts"

# log that we're in the script
echo 'Finding and processing hardhat build artifact...'

# Find most recently modified JSON build artifact.
# Avoid platform-specific `stat` flags (BSD vs GNU) by using Node.
jsonFiles=$(node --input-type=module -e '
  import fs from "fs";
  import path from "path";

  function walk(dir) {
    const entries = fs.readdirSync(dir, { withFileTypes: true });
    const files = [];
    for (const entry of entries) {
      const p = path.join(dir, entry.name);
      if (entry.isDirectory()) files.push(...walk(p));
      else if (entry.isFile() && p.endsWith(".json")) files.push(p);
    }
    return files;
  }

  const dir = process.argv[1];
  const files = walk(dir);
  files.sort((a, b) => fs.statSync(b).mtimeMs - fs.statSync(a).mtimeMs);
  if (files.length) process.stdout.write(files[0]);
' "$artifactsDir")

if [ ! -f "$jsonFiles" ]; then
  echo 'Failed to find build artifact'
  exit 1
fi

# Extract required keys and write to outputFile
if jq -c '{input, solcLongVersion}' "$jsonFiles" > "$outputFileJson"; then
  echo "export const buildArtifact = " > "$outputFileJs"
  cat "$outputFileJson" >> "$outputFileJs"
  echo "export const buildArtifact: any" > "$outputFileTsd"
  echo 'Finished processing build artifact.'
else
  echo 'Failed to process build artifact with jq'
  exit 1
fi

# ZKSYNC

if [ "$ZKSYNC" = "true" ]; then
  # Define the artifacts directory
  artifactsDir="./artifacts-zk/build-info"
  # Define the output file
  outputFileJson="./dist/zksync/buildArtifact.json"
  outputFileJs="./dist/zksync/buildArtifact.js"
  outputFileTsd="./dist/zksync/buildArtifact.d.ts"

  # log that we're in the script
  echo 'Finding and processing ZKSync hardhat build artifact...'

  # Find most recently modified JSON build artifact.
  # Avoid platform-specific `stat` flags (BSD vs GNU) by using Node.
  jsonFiles=$(node --input-type=module -e '
    import fs from "fs";
    import path from "path";

    function walk(dir) {
      const entries = fs.readdirSync(dir, { withFileTypes: true });
      const files = [];
      for (const entry of entries) {
        const p = path.join(dir, entry.name);
        if (entry.isDirectory()) files.push(...walk(p));
        else if (entry.isFile() && p.endsWith(".json")) files.push(p);
      }
      return files;
    }

    const dir = process.argv[1];
    const files = walk(dir);
    files.sort((a, b) => fs.statSync(b).mtimeMs - fs.statSync(a).mtimeMs);
    if (files.length) process.stdout.write(files[0]);
  ' "$artifactsDir")

  if [ ! -f "$jsonFiles" ]; then
    echo 'Failed to find ZKSync build artifact'
    exit 1
  fi

  # Extract required keys and write to outputFile
  if jq -c '{input, solcLongVersion, zk_version: .output.zk_version}' "$jsonFiles" >"$outputFileJson"; then
    echo "export const buildArtifact = " >"$outputFileJs"
    cat "$outputFileJson" >>"$outputFileJs"
    echo "export const buildArtifact: any" >"$outputFileTsd"
    echo 'Finished processing ZKSync build artifact.'
  else
    echo 'Failed to process ZKSync build artifact with jq'
    exit 1
  fi
fi
