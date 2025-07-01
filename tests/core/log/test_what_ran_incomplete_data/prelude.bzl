# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

def _action(ctx):
    out = ctx.actions.declare_output("out")
    ctx.actions.run(cmd_args(["sh", "-c", 'echo > "$1"', "--", out.as_output()]), category = "run")
    return [DefaultInfo(default_outputs = [out])]

my_rule = rule(
    impl = _action,
    attrs = {},
)
