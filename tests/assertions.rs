mod common;

use anyhow::Result;
use common::Context;

#[test]
fn describe_simple() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, describe } from "str";
            describe("description", () => {
                it("works", () => {
                    assertEq(true, true);
                });
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> description -> works ...
            index.test.ts -> description -> works PASSED
            Ran 1 test, 1 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn describe_bigger() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, describe } from "str";
            describe("description", () => {
                it("works", () => {
                    assertEq(true, true);
                });
                describe("second description", () => {
                    it("also works", () => {
                        assertEq(true, true);
                    });
                });
                it("works, too", () => {
                    assertEq(true, true);
                });
            });
            it("fails", () => {
                assertEq(true, false);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        r#"
            index.test.ts -> description -> works ...
            index.test.ts -> description -> works PASSED
            index.test.ts -> description -> second description -> also works ...
            index.test.ts -> description -> second description -> also works PASSED
            index.test.ts -> description -> works, too ...
            index.test.ts -> description -> works, too PASSED
            index.test.ts -> fails ...
            true
                !==
            false
            index.test.ts -> fails FAILED
            Ran 4 tests, 3 passed, 1 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn describe_pops_description_stack_correctly_after_failures() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, describe } from "str";
            describe("description", () => {
                it("fails", () => {
                    assertEq(true, false);
                });
            });
            it("works", () => {
                assertEq(true, true);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        r#"
            index.test.ts -> description -> fails ...
            true
                !==
            false
            index.test.ts -> description -> fails FAILED
            index.test.ts -> works ...
            index.test.ts -> works PASSED
            Ran 2 tests, 1 passed, 1 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn before_each_is_run_before_every_test() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, beforeEach } from "str";
            let test_variable;
            beforeEach(() => {
                test_variable = "set";
            });
            it("works", () => {
                console.error(test_variable);
                test_variable = "dirty";
            });
            it("works", () => {
                console.error(test_variable);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> works ...
            set
            index.test.ts -> works PASSED
            index.test.ts -> works ...
            set
            index.test.ts -> works PASSED
            Ran 2 tests, 2 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn before_each_works_when_declared_later() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, beforeEach } from "str";
            let test_variable;
            it("works", () => {
                console.error(test_variable);
            });
            beforeEach(() => {
                test_variable = "set";
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> works ...
            set
            index.test.ts -> works PASSED
            Ran 1 test, 1 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn before_each_is_run_only_for_tests_in_its_scope() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, beforeEach, describe } from "str";
            it("outer", () => {});
            describe("scope", () => {
                beforeEach(() => {
                    console.error("beforeEach");
                });
                it("inner", () => {});
            });
            it("outer", () => {});
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> outer ...
            index.test.ts -> outer PASSED
            index.test.ts -> scope -> inner ...
            beforeEach
            index.test.ts -> scope -> inner PASSED
            index.test.ts -> outer ...
            index.test.ts -> outer PASSED
            Ran 3 tests, 3 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn before_each_can_be_declared_multiple_times() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, beforeEach, describe } from "str";
            let variable;
            beforeEach(() => {
                variable = [];
                variable.push(1);
            });
            beforeEach(() => {
                variable.push(2);
            });
            it("works", () => {
                console.error(variable);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> works ...
            [ 1, 2 ]
            index.test.ts -> works PASSED
            Ran 1 test, 1 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn before_each_can_be_stacked() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, beforeEach, describe } from "str";
            let variable;
            beforeEach(() => {
                variable = []
                variable.push("outer beforeEach");
            });
            it("outer", () => {
                console.error(variable);
                variable.push("dirty");
            });
            describe("scope", () => {
                beforeEach(() => {
                    variable.push("inner beforeEach");
                });
                it("inner", () => {
                    console.error(variable);
                });
            });
            it("outer", () => {
                console.error(variable);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> outer ...
            [ 'outer beforeEach' ]
            index.test.ts -> outer PASSED
            index.test.ts -> scope -> inner ...
            [ 'outer beforeEach', 'inner beforeEach' ]
            index.test.ts -> scope -> inner PASSED
            index.test.ts -> outer ...
            [ 'outer beforeEach' ]
            index.test.ts -> outer PASSED
            Ran 3 tests, 3 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn before_each_on_the_top_level_will_be_run_for_every_nested_test() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, beforeEach, describe } from "str";
            let variable;
            beforeEach(() => {
                variable = [];
                variable.push("outer beforeEach");
            });
            describe("nested", () => {
                beforeEach(() => variable.push("inner beforeEach"));
                it("inner", () => {
                    console.error(variable);
                    variable.push("dirty");
                });
                it("inner", () => console.error(variable));
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> nested -> inner ...
            [ 'outer beforeEach', 'inner beforeEach' ]
            index.test.ts -> nested -> inner PASSED
            index.test.ts -> nested -> inner ...
            [ 'outer beforeEach', 'inner beforeEach' ]
            index.test.ts -> nested -> inner PASSED
            Ran 2 tests, 2 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn after_each_simple() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, beforeEach, afterEach } from "str";
            let counter = 0;
            beforeEach(() => {
                counter++;
            });
            afterEach(() => {
                counter--;
            });
            it("a", () => {
                console.error(counter);
            });
            it("b", () => {
                console.error(counter);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> a ...
            1
            index.test.ts -> a PASSED
            index.test.ts -> b ...
            1
            index.test.ts -> b PASSED
            Ran 2 tests, 2 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn after_each_can_be_declared_multiple_times() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, beforeEach, afterEach } from "str";
            let counter = 0;
            beforeEach(() => {
                counter += 2;
            });
            afterEach(() => {
                counter--;
            });
            afterEach(() => {
                counter--;
            });
            it("a", () => {
                console.error(counter);
            });
            it("b", () => {
                console.error(counter);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> a ...
            2
            index.test.ts -> a PASSED
            index.test.ts -> b ...
            2
            index.test.ts -> b PASSED
            Ran 2 tests, 2 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn after_each_when_nested_is_executed_bottom_up() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, beforeEach, afterEach, describe } from "str";
            let outer;
            beforeEach(() => {
                outer = "outer set";
            });
            afterEach(() => {
                console.error(`outer afterEach: ${JSON.stringify({outer})}, typeof inner: ${typeof inner}`);
                outer = null;
            });
            describe("nested", () => {
                let inner;
                beforeEach(() => {
                    inner = "inner set";
                });
                afterEach(() => {
                    console.error(`inner afterEach: ${JSON.stringify({outer, inner})}`);
                    inner = "empty and dirty";
                });
                it("test", () => {});
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> nested -> test ...
            inner afterEach: {"outer":"outer set","inner":"inner set"}
            outer afterEach: {"outer":"outer set"}, typeof inner: undefined
            index.test.ts -> nested -> test PASSED
            Ran 1 test, 1 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn after_each_can_be_declared_later() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, beforeEach, afterEach, describe } from "str";
            it("test", () => {});
            afterEach(() => console.error("afterEach"));
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> test ...
            afterEach
            index.test.ts -> test PASSED
            Ran 1 test, 1 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn before_all_runs_before_all_tests_once() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, beforeAll } from "str";
            let counter = 0;
            beforeAll(() => {
                counter += 1;
            });
            it("a", () => {
                console.error(counter);
            });
            it("b", () => {
                console.error(counter);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> a ...
            1
            index.test.ts -> a PASSED
            index.test.ts -> b ...
            1
            index.test.ts -> b PASSED
            Ran 2 tests, 2 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn before_all_runs_before_all_tests_when_declared_later() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, beforeAll } from "str";
            let variable;
            it("a", () => {
                console.error(variable);
            });
            beforeAll(() => {
                variable = "set";
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> a ...
            set
            index.test.ts -> a PASSED
            Ran 1 test, 1 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn before_all_is_run_only_for_tests_in_its_scope() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, describe, beforeAll } from "str";
            let variable = "outer value";
            it("outer", () => {
                console.error(variable);
                variable = "dirty";
            });
            describe("description", () => {
                beforeAll(() => {
                    variable = "inner value";
                });
                it("inner", () => {
                    console.error(variable);
                });
            });
            it("outer", () => {
                console.error(variable);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> outer ...
            outer value
            index.test.ts -> outer PASSED
            index.test.ts -> description -> inner ...
            inner value
            index.test.ts -> description -> inner PASSED
            index.test.ts -> outer ...
            inner value
            index.test.ts -> outer PASSED
            Ran 3 tests, 3 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn before_all_can_be_declared_multiple_times() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { it, beforeAll } from "str";
            let variable = [];
            beforeAll(() => {
                variable.push(1);
            });
            beforeAll(() => {
                variable.push(2);
            });
            it("works", () => {
                console.error(variable);
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> works ...
            [ 1, 2 ]
            index.test.ts -> works PASSED
            Ran 1 test, 1 passed, 0 failed.
        "#,
    )?;
    Ok(())
}

#[test]
fn after_all_runs_after_all_tests() -> Result<()> {
    let context = Context::new()?;
    context.write("file", "foo")?;
    context.write(
        "index.test.ts",
        r#"
            import { it, afterAll } from "str";
            import { readFileSync, writeFileSync } from "fs";
            it("works", () => {
                const content = readFileSync("file").toString();
                console.error(content);
            });
            afterAll(() => {
                writeFileSync("file", "bar");
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        0,
        r#"
            index.test.ts -> works ...
            foo
            index.test.ts -> works PASSED
            Ran 1 test, 1 passed, 0 failed.
        "#,
    )?;
    assert_eq!(context.read("file")?, "bar");
    Ok(())
}

#[test]
#[ignore]
fn after_all_runs_after_all_tests_once() -> Result<()> {
    Ok(())
}

#[test]
#[ignore]
fn after_all_runs_after_all_tests_if_declared_first() -> Result<()> {
    Ok(())
}

#[test]
#[ignore]
fn after_all_can_be_declared_multiple_times() -> Result<()> {
    Ok(())
}

#[test]
#[ignore]
fn after_all_is_only_run_for_tests_in_its_scope() -> Result<()> {
    Ok(())
}

#[test]
fn test_alias() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { test } from "str";
            test("works", () => {});
            test("fails", () => {
                throw "foo";
            });
        "#,
    )?;
    context.run_assert(
        "index.test.ts",
        1,
        r#"
            index.test.ts -> works ...
            index.test.ts -> works PASSED
            index.test.ts -> fails ...
            EXCEPTION: foo
            index.test.ts -> fails FAILED
            Ran 2 tests, 1 passed, 1 failed.
        "#,
    )?;
    Ok(())
}
