# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

# pyre-strict


from buck2.tests.e2e_util.api.buck import Buck
from buck2.tests.e2e_util.buck_workspace import buck_test, env


@buck_test()
async def test_run_test_with_content_based_path(buck: Buck) -> None:
    await buck.test("root//:run_test_with_content_based_path")


@buck_test()
@env("BUCK2_ALLOW_INTERNAL_TEST_RUNNER_DO_NOT_USE", "1")
async def test_platform_resolution(buck: Buck) -> None:
    await buck.test(
        ":local_resources_test",
        test_executor="",
    )
    res = await buck.log("what-ran")
    assert "MY_RESOURCE_ID=42" in res.stdout
