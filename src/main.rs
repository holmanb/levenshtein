use std::cmp;
use std::env;

fn usage() -> Result<usize, &'static str> {
    Err("usage: dist [ham|lev|dam] [string1] [string2]")
}

fn damerau(s1: String, s2: String) -> usize {
    let s1_length = s1.len();
    let s2_length = s2.len();
    if s1_length == 0 {
        s2_length
    } else if s2_length == 0 {
        s1_length
    } else {
        let tail_l = s1[1..].to_string();
        let tail_r = s2[1..].to_string();
        let s1_chars_0 = s1.chars().nth(0);
        let s1_chars_1 = s1.chars().nth(1);
        let s2_chars_0 = s2.chars().nth(0);
        let s2_chars_1 = s2.chars().nth(1);
        if s1_chars_0 == s2_chars_0 {
            damerau(tail_l.clone(), tail_r.clone())
        } else {
            let edit = damerau(tail_l.clone(), tail_r.clone());
            let transpose = if s1_length > 1
                && s2_length > 1
                && s1_chars_0 == s2_chars_1
                && s1_chars_1 == s2_chars_0
            {
                let trans_l = s1[2..].to_string();
                let trans_r = s2[2..].to_string();
                let transpose = damerau(trans_l, trans_r);
                cmp::min(edit, transpose)
            } else {
                edit
            };
            let insert = damerau(s1, tail_r);
            let delete = damerau(tail_l, s2);
            1 + cmp::min(insert, cmp::min(delete, cmp::min(edit, transpose)))
        }
    }
}

/* levinshtein distance definition:
 * ================================
 * |a|                      if |b| = 0
 * |b|                      if |a| = 0
 * lev(tail(a), tail(b))    if a[0] = b[0]
 * 1 + min                  lev(tail(a), b)
 * 1 + min                  lev(a, tail(b))
 * 1 + min                  lev(tail(a), tail(b))
 *
 * where tail(a) == a[1..]
 * */
fn levinshtein(s1: String, s2: String) -> usize {
    if s1.len() == 0 {
        s2.len()
    } else if s2.len() == 0 {
        s1.len()
    } else {
        let tail_l = s1[1..].to_string();
        let tail_r = s2[1..].to_string();
        if s1.chars().nth(0) == s2.chars().nth(0) {
            levinshtein(tail_l, tail_r)
        } else {
            let lev_tail_r = levinshtein(s1, tail_r.clone());
            let lev_tail_l = levinshtein(tail_l.clone(), s2);
            let lev_tail_b = levinshtein(tail_l, tail_r);
            1 + cmp::min(lev_tail_r, cmp::min(lev_tail_l, lev_tail_b))
        }
    }
}

fn hamming(s1: String, s2: String) -> usize {
    let same = s1
        .chars()
        .zip(s2.chars())
        .filter(|(a, b)| a == b)
        .count();
    s1.len() - same
}

fn main () {

    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let code = if command == "ham" {
        "Hamming"
    } else if command == "dam"{
        "Damerau"
    } else {
        "Levinshtein"
    };
    let out: Result<usize, &'static str> = if args.len() != 4 {
        println!("incorrect number of arguments: {}", args.len());
        usage()
    } else {
        let str1 = &args[2];
        let str2 = &args[3];
        if command == "lev" {
            Ok(levinshtein(str1.to_string(), str2.to_string()))
        } else if command == "dam" {
            Ok(damerau(str1.to_string(), str2.to_string()))
        } else if command == "ham" {
            if str1.len() != str2.len() {
                Err("invalid arguments, must be same length for hamming")
            } else {
                Ok(hamming(str1.to_string(), str2.to_string()))
            }
        } else {
            println!("invalid command: {}", command);
            usage()
        }
    };
    let f = match out {
        Ok(out) => format!("{} Distance: {}", code, out),
        Err(out) => format!("Error:\n{}", out),
    };
    println!("{}", f);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ham_0() {
        assert_eq!(0, hamming("1".to_string(), "1".to_string()));
    }
    #[test]
    fn ham_1() {
        assert_eq!(1, hamming("0".to_string(), "1".to_string()));
    }
    #[test]
    fn ham_4() {
        assert_eq!(4, hamming("0000".to_string(), "1111".to_string()));
    }
    #[test]
    fn lev_0() {
        assert_eq!(0, levinshtein("1".to_string(), "1".to_string()));
    }
    #[test]
    fn lev_1_same_len() {
        assert_eq!(1, levinshtein("0".to_string(), "1".to_string()));
    }
    #[test]
    fn lev_1_diff_len() {
        assert_eq!(1, levinshtein("0".to_string(), "01".to_string()));
    }
    #[test]
    fn lev_4_diff_len() {
        assert_eq!(4, levinshtein("".to_string(), "1234".to_string()));
    }
    #[test]
    fn dam_0() {
        assert_eq!(0, damerau("".to_string(), "".to_string()));
    }
    #[test]
    fn dam_0_len_1() {
        assert_eq!(0, damerau("1".to_string(), "1".to_string()));
    }
    #[test]
    fn dam_delete() {
        assert_eq!(2, damerau("192".to_string(), "1".to_string()));
    }
    #[test]
    fn dam_insert() {
        assert_eq!(2, damerau("1".to_string(), "192".to_string()));
    }
    #[test]
    fn dam_edit() {
        assert_eq!(1, damerau("122".to_string(), "192".to_string()));
    }
    #[test]
    fn dam_transpose() {
        assert_eq!(1, damerau("129".to_string(), "192".to_string()));
    }
}
