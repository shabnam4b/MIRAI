// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

// A test that visits the ProjectionElem::Downcast case of Visitor::visit_projection_elem

pub enum Foo {
    Bar1(i32),
    Bar2(i32),
}

pub fn g() -> Foo {
    Foo::Bar1(2)
}
