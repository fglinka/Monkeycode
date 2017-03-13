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

static MONKEYCODE_CODE_TABLE : [char; 4] = ['\u{1f435}', '\u{1f648}', '\u{1f649}', '\u{1f64a}'];

pub fn char_to_monkeycode(c: &u8) -> [char; 4] {
    let mut ret : [char; 4] = ['\0'; 4];
    for i in 0..4 {
        ret[i] = MONKEYCODE_CODE_TABLE[((c >> (i * 2)) % (4 as u8)) as usize];
    }
    ret
}

pub fn string_to_monkeycode(s: &str) -> String {
    iter_to_monkeycode(s.as_bytes().iter())
}

pub fn iter_to_monkeycode<'a, I>(iter: I) -> String 
where I: Iterator<Item=&'a u8> {
    iter.fold(String::new(), |acc, &x| {
        char_to_monkeycode(&x).iter().fold(acc, |mut acc2, &c| {
            acc2.push(c);
            acc2
        })
    })
}

