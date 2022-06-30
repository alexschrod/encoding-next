// This is a part of encoding-next.
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

//! Japanese index tables for [encoding-next](https://github.com/alexschrod/encoding-next).

#![cfg_attr(test, feature(test))]

/// JIS X 0208 with common extensions.
///
/// From the Encoding Standard:
///
/// > This is the JIS X 0208 standard including formerly proprietary extensions from IBM and NEC.
#[rustfmt::skip]
pub mod jis0208;

/// JIS X 0212.
///
/// From the Encoding Standard:
///
/// > This is the JIS X 0212 standard.
/// > It is only used by the euc-jp decoder due to lack of widespread support elsewhere.
#[rustfmt::skip]
pub mod jis0212;
