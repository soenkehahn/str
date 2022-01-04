ci: setup test run-example clippy render-readme-check integration

setup:
  #!/usr/bin/env bash
  set -eu
  cd typescript-library
  ../if-newer package.json node_modules/.touch yarn install
  touch node_modules/.touch
  cd ../tests/test-project
  ../../if-newer package.json node_modules/.touch yarn install
  touch node_modules/.touch

test *args="": typescript-library-bundle
  (cargo test --test unit -- {{ args }})

integration: typescript-library-bundle
  (cargo test --test integration -- --test-threads=1)

run-example: setup typescript-library-bundle
  #!/usr/bin/env bash
  set -eux
  cd example
  yarn install
  cargo run -- simple.ts || true

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
  set -eu
  cd typescript-library
  ls *.ts tsconfig.json | ../if-newer - dist/index.js yarn tsc
  ../if-newer dist/index.js str.tgz yarn pack --filename str.tgz
