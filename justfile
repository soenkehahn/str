test:
  (cd tests ; cargo test -- --test-threads=1)

run-example:
  (cd example ; yarn)
  (cd example ; yarn add link:..)
  (cd example ; ! ../str simple.ts)

ci: test run-example render-readme-check

install prefix="/usr/local/bin":
  cp str {{ prefix }}/

render-readme:
  php README.php > README.md

render-readme-check:
  #!/usr/bin/env bash
  diff <(php README.php) README.md
