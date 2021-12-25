ci: test integration run-example render-readme-check

test *args="":
  (cd tests ; cargo test --lib -- --test-threads=1 {{ args }})

integration:
  (cd tests ; cargo test integration -- --test-threads=1)

run-example:
  #!/usr/bin/env bash
  set -eux
  cd example
  yarn
  yarn add link:..
  ! ../str simple.ts

render-readme:
  php README.php > README.md

render-readme-check:
  #!/usr/bin/env bash
  set -eux
  diff <(php README.php) README.md

install prefix="/usr/local/bin":
  cp str {{ prefix }}/
