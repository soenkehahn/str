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
  cd ../../example
  ../if-newer package.json node_modules/.touch yarn install
  touch node_modules/.touch

build:
  go build cmd/str.go

test *args="": typescript-library-bundle build
  cargo ltest --test basic -- {{ args }}
  cargo ltest --test assertions -- {{ args }}

integration: typescript-library-bundle build
  (cargo ltest --test integration -- --test-threads=1)

run-example: setup typescript-library-bundle
  (cd example && go run ../cmd/str.go simple.ts || true)

clippy:
  cargo lclippy --tests

render-readme:
  php README.php > README.md

render-readme-check:
  #!/usr/bin/env bash
  set -eux
  diff <(php README.php) README.md

install prefix="/usr/local": build
  cp str {{ prefix }}/bin/

typescript-library-bundle: setup
  #!/usr/bin/env bash
  set -eu
  cd typescript-library
  (find src -type f -name '*.ts' ; echo tsconfig.json) | \
    ../if-newer - dist/index.js yarn tsc
  ../if-newer dist/index.js str.tgz yarn pack --filename str.tgz
