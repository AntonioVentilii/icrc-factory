#!/usr/bin/env bash
set -euo pipefail

function install_did_files() {
  jq -r '.canisters | to_entries | .[] | select(.value.candid != null) | "\(.key) \(.value.candid)"' dfx.json |
    while read -r line; do
      IFS=', ' read -r -a array <<<"$line"
      canister_name="${array[0]}"
      source="${array[1]}"
      filename="${source##*/}"
      #      filename="${filename//-/_}" # dfx uses underscores rather than hyphens
      destination=".dfx/local/canisters/${array[0]}/${filename}"
      mkdir -p "$(dirname "$destination")"
      case "$source" in
      http*) scripts/download-immutable.sh "$source" "$destination" ;;
      *) if test -e "$source"; then cp "$source" "$destination"; else echo "WARNING: $canister_name did file not found at $source"; fi ;;
      esac
    done
}

generate_declarations() {
  local canister="$1"

  echo "Generating bindings for $canister"

  local didfile=".dfx/local/canisters/${canister}/${canister}.did"
  local didfolder="src/declarations/${canister}"

  local generatedFolder="${didfolder}/declarations"
  local generatedTsfile="${generatedFolder}/${canister}.did.d.ts"
  local generatedJsfile="${generatedFolder}/${canister}.did.js"

  if [ -f "$didfile" ]; then
    mkdir -p "$didfolder"

    # --actor-disabled: skip generating actor files, since we handle those ourselves
    # --force: overwrite files. Required; otherwise, icp-bindgen would delete files at preprocess,
    # which causes issues when multiple .did files are located in the same folder.
    npx icp-bindgen --did-file "${didfile}" --out-dir "${didfolder}" --actor-disabled --force

    # icp-bindgen generates the files in a `declarations` subfolder
    # using a different suffix for JavaScript as the one we used to use.
    # That's why we have to post-process the results.
    mv "${generatedTsfile}" "${didfolder}"
    mv "${generatedJsfile}" "${didfolder}"
    rm -r "${generatedFolder}"
  else
    echo "DID file skipped: $didfile"
  fi
}

#scripts/setup cargo-binstall candid-extractor didc

cargo build

WASM="target/wasm32-unknown-unknown/release/icrc_factory.wasm"

test -e "$WASM" || cargo build --manifest-path="src/icrc-factory/Cargo.toml" \
  --target wasm32-unknown-unknown \
  --release --package "icrc-factory"

candid-extractor "$WASM" >"src/icrc-factory/icrc-factory.did"

install_did_files

generate_declarations "icrc-factory"
