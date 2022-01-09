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
        "#,
    );
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
        "#,
    );
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
        "#,
    );
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
        "#,
    );
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
        "#,
    );
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
        "#,
    );
    Ok(())
}

#[test]
#[ignore]
fn before_each_can_be_declared_multiple_times() -> Result<()> {
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
        "#,
    );
    Ok(())
}

#[test]
#[ignore]
fn before_each_can_be_stacked() -> Result<()> {
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
        "#,
    );
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
        "#,
    );
    Ok(())
}

#[test]
#[ignore]
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
                counter = "set";
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
        "#,
    );
    Ok(())
}

#[test]
#[ignore]
fn before_all_is_run_only_for_tests_in_its_scope() -> Result<()> {
    let context = Context::new()?;
    context.write(
        "index.test.ts",
        r#"
            import { assertEq, it, beforeEach } from "str";

            let test_variable;
            it("works", () => {
                console.error(test_variable);
                test_variable = "dirty";
            });

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
            index.test.ts -> works ...
            set
            index.test.ts -> works PASSED
        "#,
    );
    Ok(())
}
