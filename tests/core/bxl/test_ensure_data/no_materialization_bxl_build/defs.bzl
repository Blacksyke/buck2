# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

def _remote_text_impl(ctx):
    text = ctx.attrs.text

    out = ctx.actions.declare_output("action_output")
    ctx.actions.run(
        cmd_args(["cp", text, out.as_output()]),
        category = "touch",
    )

    return [DefaultInfo(default_output = out)]

remote_text = rule(
    impl = _remote_text_impl,
    attrs = {
        "text": attrs.source(),
    },
)
