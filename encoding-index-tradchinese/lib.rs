// This is a part of encoding-next.
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

//! Traditional Chinese index tables for
//! [encoding-next](https://github.com/alexschrod/encoding-next).

#![cfg_attr(test, feature(test))]

/// Big5 and HKSCS.
///
/// From the Encoding Standard:
///
/// > This matches the Big5 standard
/// > in combination with the Hong Kong Supplementary Character Set and other common extensions.
#[rustfmt::skip]
pub mod big5;
