// This is a part of encoding-next.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! Macros and utilities for testing.

use crate::types::{RawDecoder, RawEncoder};
use std::borrow::ToOwned;

pub struct TestResult<'a, Output: 'a + ?Sized + ToOwned> {
    pub expected_return: (usize, Option<isize>),
    pub expected_push: &'a Output,
    pub actual_return: (usize, Option<isize>),
    pub actual_push: Output::Owned,
}

pub trait Testable {
    type Input: ?Sized;
    type Output: ?Sized + ToOwned;

    fn process_feed_ok<'a>(
        &mut self,
        processed: &Self::Input,
        unprocessed: &Self::Input,
        output: &'a Self::Output,
    ) -> TestResult<'a, Self::Output>;
    fn process_feed_err<'a>(
        &mut self,
        backup: isize,
        processed: &Self::Input,
        problem: &Self::Input,
        remaining: &Self::Input,
        output: &'a Self::Output,
    ) -> TestResult<'a, Self::Output>;
    fn process_finish_ok<'a>(&mut self, output: &'a Self::Output) -> TestResult<'a, Self::Output>;
    fn process_finish_err<'a>(
        &mut self,
        backup: isize,
        output: &'a Self::Output,
    ) -> TestResult<'a, Self::Output>;
}

impl Testable for dyn RawDecoder {
    type Input = [u8];
    type Output = str;

    fn process_feed_ok<'a>(
        &mut self,
        processed: &[u8],
        unprocessed: &[u8],
        output: &'a str,
    ) -> TestResult<'a, str> {
        let mut input = Vec::with_capacity(processed.len() + unprocessed.len());
        input.extend(processed.iter().cloned());
        input.extend(unprocessed.iter().cloned());

        let mut buf = String::new();
        let (nprocessed, err) = self.raw_feed(&input, &mut buf);
        TestResult {
            expected_return: (processed.len(), None),
            expected_push: output,
            actual_return: (nprocessed, err.map(|e| e.upto)),
            actual_push: buf,
        }
    }

    fn process_feed_err<'a>(
        &mut self,
        backup: isize,
        processed: &[u8],
        problem: &[u8],
        remaining: &[u8],
        output: &'a str,
    ) -> TestResult<'a, str> {
        let mut input = Vec::with_capacity(processed.len() + problem.len() + remaining.len());
        input.extend(processed.iter().cloned());
        input.extend(problem.iter().cloned());
        input.extend(remaining.iter().cloned());

        let mut buf = String::new();
        let (nprocessed, err) = self.raw_feed(&input[-backup as usize..], &mut buf);
        TestResult {
            expected_return: (
                processed.len(),
                Some(processed.len() as isize + problem.len() as isize + backup),
            ),
            expected_push: output,
            actual_return: (nprocessed, err.map(|e| e.upto)),
            actual_push: buf,
        }
    }

    fn process_finish_ok<'a>(&mut self, output: &'a str) -> TestResult<'a, str> {
        let mut buf = String::new();
        let err = self.raw_finish(&mut buf);
        TestResult {
            expected_return: (0, None),
            expected_push: output,
            actual_return: (0, err.map(|e| e.upto)),
            actual_push: buf,
        }
    }

    fn process_finish_err<'a>(&mut self, backup: isize, output: &'a str) -> TestResult<'a, str> {
        let mut buf = String::new();
        let err = self.raw_finish(&mut buf);
        TestResult {
            expected_return: (0, Some(backup)),
            expected_push: output,
            actual_return: (0, err.map(|e| e.upto)),
            actual_push: buf,
        }
    }
}

impl Testable for dyn RawEncoder {
    type Input = str;
    type Output = [u8];

    fn process_feed_ok<'a>(
        &mut self,
        processed: &str,
        unprocessed: &str,
        output: &'a [u8],
    ) -> TestResult<'a, [u8]> {
        let mut input = String::with_capacity(processed.len() + unprocessed.len());
        input.push_str(processed);
        input.push_str(unprocessed);

        let mut buf = Vec::new();
        let (nprocessed, err) = self.raw_feed(&input, &mut buf);
        TestResult {
            expected_return: (processed.len(), None),
            expected_push: output,
            actual_return: (nprocessed, err.map(|e| e.upto)),
            actual_push: buf,
        }
    }

    fn process_feed_err<'a>(
        &mut self,
        backup: isize,
        processed: &str,
        problem: &str,
        remaining: &str,
        output: &'a [u8],
    ) -> TestResult<'a, [u8]> {
        let mut input = String::with_capacity(processed.len() + problem.len() + remaining.len());
        input.push_str(processed);
        input.push_str(problem);
        input.push_str(remaining);

        let mut buf = Vec::new();
        let (nprocessed, err) = self.raw_feed(&input[-backup as usize..], &mut buf);
        TestResult {
            expected_return: (
                processed.len(),
                Some(processed.len() as isize + problem.len() as isize + backup),
            ),
            expected_push: output,
            actual_return: (nprocessed, err.map(|e| e.upto)),
            actual_push: buf,
        }
    }

    fn process_finish_ok<'a>(&mut self, output: &'a [u8]) -> TestResult<'a, [u8]> {
        let mut buf = Vec::new();
        let err = self.raw_finish(&mut buf);
        TestResult {
            expected_return: (0, None),
            expected_push: output,
            actual_return: (0, err.map(|e| e.upto)),
            actual_push: buf,
        }
    }

    fn process_finish_err<'a>(&mut self, backup: isize, output: &'a [u8]) -> TestResult<'a, [u8]> {
        let mut buf = Vec::new();
        let err = self.raw_finish(&mut buf);
        TestResult {
            expected_return: (0, Some(backup)),
            expected_push: output,
            actual_return: (0, err.map(|e| e.upto)),
            actual_push: buf,
        }
    }
}

macro_rules! assert_expected {
    ($result:expr, $func:expr, $filter:expr) => {{
        use crate::testutils::Testable;
        match $result {
            result => {
                assert!(
                    result.expected_return == result.actual_return,
                    "{} should return {:?}, but instead returned {:?}",
                    $func,
                    $filter(result.expected_return),
                    $filter(result.actual_return)
                );
                assert!(
                    &result.expected_push[..] == &result.actual_push[..],
                    "{} should push {:?}, but instead pushed {:?}",
                    $func,
                    result.expected_push,
                    result.actual_push
                );
            }
        }
    }};
}

macro_rules! assert_feed_ok {
    ($this:expr, $processed:expr, $unprocessed:expr, $output:expr) => {
        assert_expected!(
            $this.process_feed_ok(&$processed, &$unprocessed, &$output),
            "raw_feed",
            |r| r
        )
    };
}

macro_rules! assert_feed_err {
    ($this:expr, $backup:expr, $processed:expr, $problem:expr, $remaining:expr, $output:expr) => {
        assert_expected!(
            $this.process_feed_err($backup, &$processed, &$problem, &$remaining, &$output),
            "raw_feed",
            |r| r
        )
    };
    ($this:expr, $processed:expr, $problem:expr, $remaining:expr, $output:expr) => {
        assert_feed_err!($this, 0, $processed, $problem, $remaining, $output)
    };
}

macro_rules! assert_finish_ok {
    ($this:expr, $output:expr) => {
        assert_expected!($this.process_finish_ok(&$output), "raw_finish", |r: (
            usize,
            Option<isize>
        )| r.0)
    };
}

macro_rules! assert_finish_err {
    ($this:expr, $backup:expr, $output:expr) => {
        assert_expected!(
            $this.process_finish_err($backup, &$output),
            "raw_finish",
            |r: (usize, Option<isize>)| r.0
        )
    };
    ($this:expr, $output:expr) => {
        assert_finish_err!($this, 0, $output)
    };
}

/// Some ASCII-only text to test.
//
// the first paragraphs of the article "English Language" from English Wikipedia.
// https://en.wikipedia.org/w/index.php?title=English_language&oldid=608500518
pub static ASCII_TEXT: &str =
    "English is a West Germanic language that was first spoken in early medieval England \
     and is now a global lingua franca. It is spoken as a first language by \
     the majority populations of several sovereign states, including the United Kingdom, \
     the United States, Canada, Australia, Ireland, New Zealand and a number of Caribbean nations; \
     and it is an official language of almost 60 sovereign states. It is the third-most-common \
     native language in the world, after Mandarin Chinese and Spanish. It is widely learned as \
     a second language and is an official language of the European Union, many Commonwealth \
     countries and the United Nations, as well as in many world organisations.";

/// Some Korean text to test.
//
// the first paragraphs of the article "Korean Language" from Korean Wikipedia.
// https://ko.wikipedia.org/w/index.php?title=%ED%95%9C%EA%B5%AD%EC%96%B4&oldid=12331875
pub static KOREAN_TEXT: &str =
    "?????????(?????????)??? ?????? ?????????(?????????)??? ?????????(?????????) ?????? ???????????? ????????? ?????????, \
     ????????????????????? ?????????, ?????????????????? ?????????, ???????????????????????????????????? ??????, ??????????????? \
     ?????????(?????????), ?????????????????? ?????????. ??????????????????, ????????? ??? ??? ????????? ???????????? ??????????????? \
     ?????????(?????????)??? ?????????. 19?????? ?????? ?????? ???????????? ?????? ????????? ??????, 20?????? ?????? \
     ?????? ??????????????? ??????, 20?????? ?????? ??????????????? ?????? ????????? ?????? ?????? ?????? ??????, ??????, \
     ????????? ???????????? ?????????, ??????????????????, ??????, ?????????, ?????????????????????, ?????????, ?????????, ????????? ??? \
     ?????? ????????? ???????????? ??????????????? ???????????? ????????? ??????. ????????? ?????? ????????? ??? ????????? ????????? \
     ??? 8???250??? ????????? ????????????.";

/// Some Japanese text to test.
//
// the first paragraphs of the article "Japanese Language" from Japanese Wikipedia.
// https://ja.wikipedia.org/w/index.php?title=%E6%97%A5%E6%9C%AC%E8%AA%9E&oldid=51443986
pub static JAPANESE_TEXT: &str =
    "???????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ?????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ??????????????????????????????74???????????????????????????57??????????????????????????????2????????????????????????\
     ??????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ??????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ??????????????????????????????????????????????????????????????????????????????????????????1???3????????????????????????????????????\
     ???????????????????????????????????????????????????????????????????????????????????????????????????10????????????????????????????????????";

/// Some simplified Chinese text to test.
//
// the first paragraphs of the article "Chinese Language" from Chinese Wikipedia.
// https://zh.wikipedia.org/w/index.php?title=%E6%B1%89%E8%AF%AD&variant=zh-cn&oldid=31224104
pub static SIMPLIFIED_CHINESE_TEXT: &str =
    "?????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ??????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ??????????????????????????????????????????????????????????????????????????????????????????????????????\
     ?????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ??????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ???????????????????????????????????????????????????????????????????????????????????????????????????\
     ?????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ?????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ????????????????????????????????????????????????";

/// Some traditional Chinese text to test.
//
// the first paragraphs of the article "Chinese Language" from Chinese Wikipedia.
// https://zh.wikipedia.org/w/index.php?title=%E6%B1%89%E8%AF%AD&variant=zh-tw&oldid=31224104
pub static TRADITIONAL_CHINESE_TEXT: &str =
    "?????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ??????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ??????????????????????????????????????????????????????????????????????????????????????????????????????\
     ?????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ??????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ???????????????????????????????????????????????????????????????????????????????????????????????????\
     ?????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ????????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ?????????????????????????????????????????????????????????????????????????????????????????????????????????\
     ????????????????????????????????????????????????";

/// Some text with various invalid UTF-8 sequences.
//
// Markus Kuhn's UTF-8 decoder capability and stress test.
// http://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt
pub static INVALID_UTF8_TEXT: &[u8] = include_bytes!("examples/UTF-8-test.txt");

/// Returns a longer text used for external data benchmarks.
/// This can be overriden with an environment variable `EXTERNAL_BENCH_DATA`,
/// or it will use a built-in sample data (of about 100KB).
pub fn get_external_bench_data() -> Vec<u8> {
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    // An HTML file derived from the Outer Space Treaty of 1967, in six available languages.
    // http://www.unoosa.org/oosa/SpaceLaw/outerspt.html
    static LONGER_TEXT: &[u8] = include_bytes!("examples/outer-space-treaty.html");

    match env::var("EXTERNAL_BENCH_DATA") {
        Ok(path) => {
            let path = Path::new(&path);
            let mut file = File::open(&path)
                .ok()
                .expect("cannot read an external bench data");
            let mut ret = Vec::new();
            file.read_to_end(&mut ret)
                .ok()
                .expect("cannot read an external bench data");
            ret
        }
        Err(..) => LONGER_TEXT.to_vec(),
    }
}
