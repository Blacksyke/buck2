# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

def _action_fail(ctx):
    out1 = ctx.actions.declare_output("failed_action.json")
    out2 = ctx.actions.declare_output("failed_action.txt")

    run = ctx.actions.write(
        "run.py",
        [
            "import sys",
            "with open(sys.argv[1], 'w') as f:",
            "  f.write('json')",
            "with open(sys.argv[2], 'w') as f:",
            "  f.write('txt')",
            "sys.exit(1)",
        ],
    )

    ctx.actions.run(
        cmd_args(
            "python3",
            run,
            out1.as_output(),
            out2.as_output(),
        ),
        category = "test",
        outputs_for_error_handler = [out1.as_output()],
    )
    return [DefaultInfo(default_outputs = [out1, out2])]

action_fail = rule(
    impl = _action_fail,
    attrs = {
    },
)

def _undeclared_output(ctx):
    declared = ctx.actions.declare_output("failed_action.json")
    undeclared = ctx.actions.declare_output("failed_action.txt")
    ctx.actions.run(
        cmd_args(
            "python3",
            "-c",
            "import sys; sys.exit(1)",
            declared.as_output(),
        ),
        category = "test",
        outputs_for_error_handler = [undeclared.as_output()],
    )
    return [DefaultInfo(default_outputs = [declared])]

undeclared_output = rule(
    impl = _undeclared_output,
    attrs = {
    },
)
