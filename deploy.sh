#!/bin/bash

deploy() {
  echo "Deploying... to ${TARGET_SERVER} as ${SERVER_USER}"
  set -e

  cargo build --target x86_64-unknown-linux-musl --release
  cd dockio-front && trunk build --release && cd ..
  wscat -n -L --connect "ws://${TARGET_SERVER}:${DOCKIO_WS_PORT}" -x 'Terminate'
  scp target/x86_64-unknown-linux-musl/release/dockio "${SERVER_USER}@${TARGET_SERVER}:/opt/dockio/dockio"
  scp -r dockio-front/dist/* "${SERVER_USER}@${TARGET_SERVER}:/opt/dockio/"

  echo "done"
}

main() {
  set -o allexport
  # shellcheck source=/dev/null
  source .env
  set +o allexport

  case "$1" in
    "deploy")
      deploy
      ;;
    *)
      echo "Usage: $0 {deploy}"
      exit 1
      ;;
  esac

}

main "$@"
