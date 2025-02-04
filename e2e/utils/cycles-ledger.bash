CYCLES_LEDGER_VERSION="0.2.1"

build_artifact_url() {
    echo "https://raw.githubusercontent.com/dfinity/sdk/cycles-ledger-prerelease/cycles-ledger-v$CYCLES_LEDGER_VERSION/${1}"
}

downloaded_cycles_ledger_canisters_dir() {
    echo "$DFX_CACHE_ROOT/canisters/cycles-ledger/$CYCLES_LEDGER_VERSION"
}

download_cycles_ledger_canisters() {
    DOWNLOAD_DIR="$DFX_CACHE_ROOT/.download"
    DEST_DIR="$(downloaded_cycles_ledger_canisters_dir)"

    if test -d "$DEST_DIR"; then
        return
    fi

    rm -rf "$DOWNLOAD_DIR"
    mkdir -p "$DOWNLOAD_DIR" "$(dirname "$DEST_DIR")"

    for name in cycles-ledger cycles-depositor; do
        for ext in wasm.gz wasm.gz.sha256 did; do
            URL=$(build_artifact_url "${name}.${ext}")
            curl -v --fail -o "$DOWNLOAD_DIR/${name}.${ext}" "$URL"
        done
    done

    ( cd "$DOWNLOAD_DIR" && shasum -c cycles-ledger.wasm.gz.sha256 && shasum -c cycles-depositor.wasm.gz.sha256 )
    mv "$DOWNLOAD_DIR" "$DEST_DIR"
}

install_cycles_ledger_canisters() {
    download_cycles_ledger_canisters
    cp "$(downloaded_cycles_ledger_canisters_dir)"/* .
}
