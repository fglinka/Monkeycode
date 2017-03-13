/*
 *monkeycode - the reference implementation for the monkeycode encoding
 *Copyright © 2017 xeeF7
 *
 *Permission is hereby granted, free of charge, to any person obtaining
 *a copy of this software and associated documentation files (the "Software"),
 *to deal in the Software without restriction, including without limitation
 *the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *and/or sell copies of the Software, and to permit persons to whom the
 *Software is furnished to do so, subject to the following conditions:
 *
 *The above copyright notice and this permission notice shall be included
 *in all copies or substantial portions of the Software.
 *
 *THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 *EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 *OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 *IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 *DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 *TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
 *OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use std::collections::BTreeMap;

lazy_static! {
    static ref MONKEYCODE_DECODE_MAP : BTreeMap<char, u8> = {
        let mut map = BTreeMap::new();
        map.insert('\u{1f435}', 0);
        map.insert('\u{1f648}', 1);
        map.insert('\u{1f649}', 2);
        map.insert('\u{1f64a}', 3);
        map
    };
}

pub fn monkeycode_to_byte(chars : &[char]) -> Option<u8> {
    let mut ret: u8 = 0;
    for (i, &j) in chars.iter().take(4).enumerate() {
        ret += match MONKEYCODE_DECODE_MAP.get(&j) {
            None => return None,
            Some(val) => val << ((i as u8) * 2)
        };
    }
    Some(ret)
}

pub fn monkeycode_iter_to_bytes<'a, I>(mut iter: I) -> Option<Vec<u8>>
where I: Iterator<Item=&'a char> {
    let mut ret: Vec<u8> = Vec::new();
    loop {
        let part: Vec<char> = iter.by_ref().take(4).cloned().collect();
        if part.len() == 0 {
            break;
        }
        let byte = match monkeycode_to_byte(part.as_slice()) {
            None => return None,
            Some(val) => val
        };
        ret.push(byte);
        if part.len() < 4 {
            break
        }
    }
    Some(ret)
}

pub fn monkeycode_string_to_bytes(s : &str) -> Option<Vec<u8>> {
    let chars: Vec<char> = s.chars().collect();
    monkeycode_iter_to_bytes((&chars).into_iter())
}
