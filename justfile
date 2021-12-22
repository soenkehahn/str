test:
  (cd tests ; cargo test -- --test-threads=1)

run-example:
  (cd example ; yarn)
  (cd example ; yarn add link:..)
  (cd example ; ! ../str simple.ts)

ci: test run-example
