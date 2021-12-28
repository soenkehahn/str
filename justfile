ci: test run-example clippy render-readme-check integration

test *args="": bundle-typescript-library
  (cargo test --bin str -- --test-threads=1 {{ args }})

integration:
  (cargo test integration -- --test-threads=1)

run-example: bundle-typescript-library
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

bundle-typescript-library:
  #!/usr/bin/env bash
  set -eux
  cd typescript-library
  rm -rf dist
  yarn install
  yarn tsc
  yarn pack
