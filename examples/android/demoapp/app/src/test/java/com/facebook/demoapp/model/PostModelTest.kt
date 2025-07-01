/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

package com.facebook.demoapp.model

import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner

@RunWith(RobolectricTestRunner::class)
class PostModelTest {

  @Test
  fun validModel() {
    PostModel("Jane Doe", 123, "This is a post", 20)
  }

  @Test(expected = IllegalArgumentException::class)
  fun invalidModel_EmptyAuthorName() {
    PostModel("", 123, "This is a post", 20)
  }

  @Test(expected = IllegalArgumentException::class)
  fun invalidModel_EmptyDrawable() {
    PostModel("Jane Doe", 0, "This is a post", 20)
  }
}
