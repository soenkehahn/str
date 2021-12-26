ci: test run-example render-readme-check integration

test *args="":
  (cargo test --bin str -- --test-threads=1 {{ args }})

integration:
  (cargo test integration -- --test-threads=1)

run-example:
  #!/usr/bin/env bash
  set -eux
  cd example
  yarn install
  cargo run simple.ts || true

render-readme:
  php README.php > README.md

render-readme-check:
  #!/usr/bin/env bash
  set -eux
  diff <(php README.php) README.md

install prefix="/usr/local":
  cargo install --path . --root {{ prefix }}
