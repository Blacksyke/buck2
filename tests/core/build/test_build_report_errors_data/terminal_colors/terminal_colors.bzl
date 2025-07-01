# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

def _impl(ctx):
    out = ctx.actions.declare_output("out.txt")
    ctx.actions.run(
        cmd_args(
            ["bash", "-c", 'echo -e "\\033[31mHello\\033[0m" && false'],
            hidden = out.as_output(),
        ),
        category = "run",
    )
    return [DefaultInfo(default_output = out)]

terminal_colors = rule(
    impl = _impl,
    attrs = {},
)
