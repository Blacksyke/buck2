/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

// This comment is to silence the naive linter (https://fburl.com/code/6hotojag) which simply checks for
// the existence of "docusaurus-plugin-internaldocs-fb/internal" string which is defined in ./config_impl.ts


// Our internal doc builder requires a `.js` file to exist, so have this and keep the actual
// implementation in `.ts`

const { config } = require('./config_impl.ts');

module.exports = config
