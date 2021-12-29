ci: setup test run-example clippy render-readme-check integration

setup:
  (cd typescript-library && yarn install)

test *args="": typescript-library-bundle
  (cargo test --bin str -- --test-threads=1 {{ args }})

integration: typescript-library-bundle
  (cargo test integration -- --test-threads=1)

run-example: setup typescript-library-bundle
  #!/usr/bin/env bash
  set -eux
  cd example
  yarn install
  cargo run simple.ts || true

clippy:
  cargo clippy --tests

render-readme:
  php README.php > README.md

render-readme-check:
  #!/usr/bin/env bash
  set -eux
  diff <(php README.php) README.md

install prefix="/usr/local":
  cargo install --path . --root {{ prefix }}

typescript-library-bundle: setup
  #!/usr/bin/env bash
  set -eux
  cd typescript-library
  rm -rf dist
  yarn install
  yarn tsc
  yarn pack
