
// This is an auto-generated test file
// Generated by src/test/test_generator.py
//
// Last Modified: April 04 2014 09:24AM

macro_rules! run_tests(
  ($re: expr, $input: expr, $matched: expr, $ident: expr,
   $expect: pat, $groups: expr) => (
    {
      let re = match UncompiledRegexp::new($re) {
        Ok(regex) => regex,
        Err(e) => fail!(e)
      };
      let res = re.search($input);
      let expect_test = match res {
        $expect => true,
        _ => {
          println!("Failed with test {:s}: <Re: '{:s}'> | <Input: '{:s}'> | <Actual Output: '{:s}'>",
                  $ident, $re, $input, res.to_str());
          false
        }
      };
      if (!expect_test) {
        assert!(expect_test);
        return
      }
      if (res.is_some()) {
        match res {
          Some(ma) => {
            assert_eq!(ma.matched(), $matched)

            let groups: &'static[&'static str] = $groups;
            let mut i = 0;

            for g in groups.iter() {
              match ma.group(i) {
                Some(match_group) => {
                  assert_eq!(match_group, g.to_str());
                }
                None => (assert!(false))
              }

              i += 1;
            }
          }
          _ => ()
        }
      }
    }
  )
)

#[cfg(test)]
mod python_tests {
  use regexp::UncompiledRegexp;

  // Tests start here

  #[test]
  fn test_case_ident_000() {
    run_tests!("[^^]+", "abc", ~"abc", "000", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_001() {
    run_tests!("[^^]+", "^", ~"", "001", None, &'static [])
  }

  #[test]
  fn test_case_ident_002() {
    run_tests!("[^al-obc]+", "kpd", ~"kpd", "002", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_003() {
    run_tests!("[^al-obc]+", "abc", ~"", "003", None, &'static [])
  }

  #[test]
  fn test_case_ident_004() {
    run_tests!("[al-obc]+", "almocb", ~"almocb", "004", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_005() {
    run_tests!("[al-obc]+", "defzx", ~"", "005", None, &'static [])
  }

  #[test]
  fn test_case_ident_006() {
    run_tests!("a(?:b|c|d)(.)", "ace", ~"ace", "006", Some(_), &'static ["e"])
  }

  #[test]
  fn test_case_ident_007() {
    run_tests!("a(?:b|c|d)*(.)", "ace", ~"ace", "007", Some(_), &'static ["e"])
  }

  #[test]
  fn test_case_ident_008() {
    run_tests!("a(?:b|c|d)+?(.)", "ace", ~"ace", "008", Some(_), &'static ["e"])
  }

  #[test]
  fn test_case_ident_009() {
    run_tests!("[-+]?[0-9]*\\.?[0-9]+", "3.14", ~"3.14", "009", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_010() {
    run_tests!("<TAG\\b[^>]*>(.*?)</TAG>", "one<TAG>two</TAG>three", ~"<TAG>two</TAG>", "010", Some(_), &'static ["two"])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_011() {
    run_tests!(")", "", ~"", "011", None, &'static [])
  }

  #[test]
  fn test_case_ident_012() {
    run_tests!("", "", ~"", "012", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_013() {
    run_tests!("abc", "abc", ~"abc", "013", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_014() {
    run_tests!("abc", "xbc", ~"", "014", None, &'static [])
  }

  #[test]
  fn test_case_ident_015() {
    run_tests!("abc", "axc", ~"", "015", None, &'static [])
  }

  #[test]
  fn test_case_ident_016() {
    run_tests!("abc", "xabcy", ~"abc", "016", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_017() {
    run_tests!("abc", "ababc", ~"abc", "017", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_018() {
    run_tests!("ab*c", "abc", ~"abc", "018", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_019() {
    run_tests!("ab*bc", "abbc", ~"abbc", "019", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_020() {
    run_tests!("ab*bc", "abbbbc", ~"abbbbc", "020", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_021() {
    run_tests!("ab{0,}bc", "abbbbc", ~"abbbbc", "021", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_022() {
    run_tests!("ab+bc", "abbc", ~"abbc", "022", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_023() {
    run_tests!("ab+bc", "abc", ~"", "023", None, &'static [])
  }

  #[test]
  fn test_case_ident_024() {
    run_tests!("ab+bc", "abq", ~"", "024", None, &'static [])
  }

  #[test]
  fn test_case_ident_025() {
    run_tests!("ab{1,}bc", "abq", ~"", "025", None, &'static [])
  }

  #[test]
  fn test_case_ident_026() {
    run_tests!("ab+bc", "abbbbc", ~"abbbbc", "026", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_027() {
    run_tests!("ab{1,}bc", "abbbbc", ~"abbbbc", "027", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_028() {
    run_tests!("ab{1,3}bc", "abbbbc", ~"abbbbc", "028", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_029() {
    run_tests!("ab{3,4}bc", "abbbbc", ~"abbbbc", "029", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_030() {
    run_tests!("ab{4,5}bc", "abbbbc", ~"abbbbc", "030", None, &'static [])
  }

  #[test]
  fn test_case_ident_031() {
    run_tests!("ab?bc", "abbc", ~"abbc", "031", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_032() {
    run_tests!("ab?bc", "abc", ~"abc", "032", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_033() {
    run_tests!("ab{0,1}bc", "abc", ~"abc", "033", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_034() {
    run_tests!("ab?bc", "abbbbc", ~"", "034", None, &'static [])
  }

  #[test]
  fn test_case_ident_035() {
    run_tests!("ab?c", "abc", ~"abc", "035", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_036() {
    run_tests!("ab{0,1}c", "abc", ~"abc", "036", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_037() {
    run_tests!("^abc$", "abc", ~"abc", "037", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_038() {
    run_tests!("^abc$", "abcc", ~"", "038", None, &'static [])
  }

  #[test]
  fn test_case_ident_039() {
    run_tests!("^abc", "abcc", ~"abc", "039", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_040() {
    run_tests!("^abc$", "aabc", ~"", "040", None, &'static [])
  }

  #[test]
  fn test_case_ident_041() {
    run_tests!("abc$", "abcc", ~"", "041", None, &'static [])
  }

  #[test]
  fn test_case_ident_042() {
    run_tests!("^", "abc", ~"", "042", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_043() {
    run_tests!("$", "abc", ~"", "043", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_044() {
    run_tests!("a.c", "abc", ~"abc", "044", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_045() {
    run_tests!("a.c", "axc", ~"axc", "045", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_046() {
    run_tests!("a.*c", "axyzc", ~"axyzc", "046", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_047() {
    run_tests!("a.*c", "axyzd", ~"", "047", None, &'static [])
  }

  #[test]
  fn test_case_ident_048() {
    run_tests!("a[bc]d", "abc", ~"", "048", None, &'static [])
  }

  #[test]
  fn test_case_ident_049() {
    run_tests!("a[bc]d", "abd", ~"abd", "049", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_050() {
    run_tests!("a[b-d]e", "abd", ~"", "050", None, &'static [])
  }

  #[test]
  fn test_case_ident_051() {
    run_tests!("a[b-d]e", "ace", ~"ace", "051", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_052() {
    run_tests!("a[b-d]", "aac", ~"ac", "052", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_053() {
    run_tests!("a[-b]", "a-", ~"a-", "053", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_054() {
    run_tests!("a[\\-b]", "a-", ~"a-", "054", Some(_), &'static [])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_055() {
    run_tests!("a[]b", "-", ~"", "055", None, &'static [])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_056() {
    run_tests!("a[", "-", ~"", "056", None, &'static [])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_057() {
    run_tests!("a\\", "-", ~"", "057", None, &'static [])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_058() {
    run_tests!("abc)", "-", ~"", "058", None, &'static [])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_059() {
    run_tests!("(abc", "-", ~"", "059", None, &'static [])
  }

  #[test]
  fn test_case_ident_060() {
    run_tests!("a]", "a]", ~"a]", "060", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_061() {
    run_tests!("a[]]b", "a]b", ~"a]b", "061", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_062() {
    run_tests!("a[\\]]b", "a]b", ~"a]b", "062", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_063() {
    run_tests!("a[^bc]d", "aed", ~"aed", "063", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_064() {
    run_tests!("a[^bc]d", "abd", ~"", "064", None, &'static [])
  }

  #[test]
  fn test_case_ident_065() {
    run_tests!("a[^-b]c", "adc", ~"adc", "065", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_066() {
    run_tests!("a[^-b]c", "a-c", ~"", "066", None, &'static [])
  }

  #[test]
  fn test_case_ident_067() {
    run_tests!("a[^]b]c", "a]c", ~"", "067", None, &'static [])
  }

  #[test]
  fn test_case_ident_068() {
    run_tests!("a[^]b]c", "adc", ~"adc", "068", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_069() {
    run_tests!("\\ba\\b", "a-", ~"a", "069", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_070() {
    run_tests!("\\ba\\b", "-a", ~"a", "070", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_071() {
    run_tests!("\\ba\\b", "-a-", ~"a", "071", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_072() {
    run_tests!("\\by\\b", "xy", ~"", "072", None, &'static [])
  }

  #[test]
  fn test_case_ident_073() {
    run_tests!("\\by\\b", "yz", ~"", "073", None, &'static [])
  }

  #[test]
  fn test_case_ident_074() {
    run_tests!("\\by\\b", "xyz", ~"", "074", None, &'static [])
  }

  #[test]
  fn test_case_ident_075() {
    run_tests!("x\\b", "xyz", ~"", "075", None, &'static [])
  }

  #[test]
  fn test_case_ident_076() {
    run_tests!("x\\B", "xyz", ~"x", "076", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_077() {
    run_tests!("\\Ba\\B", "a-", ~"", "077", None, &'static [])
  }

  #[test]
  fn test_case_ident_078() {
    run_tests!("\\Ba\\B", "-a", ~"", "078", None, &'static [])
  }

  #[test]
  fn test_case_ident_079() {
    run_tests!("\\Ba\\B", "-a-", ~"", "079", None, &'static [])
  }

  #[test]
  fn test_case_ident_080() {
    run_tests!("\\By\\B", "xy", ~"", "080", None, &'static [])
  }

  #[test]
  fn test_case_ident_081() {
    run_tests!("\\By\\B", "yz", ~"", "081", None, &'static [])
  }

  #[test]
  fn test_case_ident_082() {
    run_tests!("\\By\\b", "xy", ~"y", "082", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_083() {
    run_tests!("\\by\\B", "yz", ~"y", "083", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_084() {
    run_tests!("\\By\\B", "xyz", ~"y", "084", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_085() {
    run_tests!("ab|cd", "abc", ~"ab", "085", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_086() {
    run_tests!("ab|cd", "abcd", ~"ab", "086", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_087() {
    run_tests!("()ef", "def", ~"ef", "087", Some(_), &'static [""])
  }

  #[test]
  fn test_case_ident_088() {
    run_tests!("$b", "b", ~"", "088", None, &'static [])
  }

  #[test]
  fn test_case_ident_089() {
    run_tests!("a\\(b", "a(b", ~"a(b", "089", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_090() {
    run_tests!("a\\(*b", "ab", ~"ab", "090", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_091() {
    run_tests!("a\\(*b", "a((b", ~"a((b", "091", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_092() {
    run_tests!("a\\\\b", "a\\b", ~"a\\b", "092", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_093() {
    run_tests!("((a))", "abc", ~"a", "093", Some(_), &'static ["a", "a"])
  }

  #[test]
  fn test_case_ident_094() {
    run_tests!("(a)b(c)", "abc", ~"abc", "094", Some(_), &'static ["a", "c"])
  }

  #[test]
  fn test_case_ident_095() {
    run_tests!("a+b+c", "aabbabc", ~"abc", "095", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_096() {
    run_tests!("(a+|b)*", "ab", ~"ab", "096", Some(_), &'static ["b"])
  }

  #[test]
  fn test_case_ident_097() {
    run_tests!("(a+|b)+", "ab", ~"ab", "097", Some(_), &'static ["b"])
  }

  #[test]
  fn test_case_ident_098() {
    run_tests!("(a+|b)?", "ab", ~"a", "098", Some(_), &'static ["a"])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_099() {
    run_tests!(")(", "-", ~"", "099", None, &'static [])
  }

  #[test]
  fn test_case_ident_100() {
    run_tests!("[^ab]*", "cde", ~"cde", "100", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_101() {
    run_tests!("abc", "", ~"", "101", None, &'static [])
  }

  #[test]
  fn test_case_ident_102() {
    run_tests!("a*", "", ~"", "102", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_103() {
    run_tests!("a|b|c|d|e", "e", ~"e", "103", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_104() {
    run_tests!("(a|b|c|d|e)f", "ef", ~"ef", "104", Some(_), &'static ["e"])
  }

  #[test]
  fn test_case_ident_105() {
    run_tests!("abcd*efg", "abcdefg", ~"abcdefg", "105", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_106() {
    run_tests!("ab*", "xabyabbbz", ~"ab", "106", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_107() {
    run_tests!("ab*", "xayabbbz", ~"a", "107", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_108() {
    run_tests!("(ab|cd)e", "abcde", ~"cde", "108", Some(_), &'static ["cd"])
  }

  #[test]
  fn test_case_ident_109() {
    run_tests!("[abhgefdc]ij", "hij", ~"hij", "109", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_110() {
    run_tests!("^(ab|cd)e", "abcde", ~"", "110", None, &'static [])
  }

  #[test]
  fn test_case_ident_111() {
    run_tests!("(abc|)ef", "abcdef", ~"ef", "111", Some(_), &'static [""])
  }

  #[test]
  fn test_case_ident_112() {
    run_tests!("(a|b)c*d", "abcd", ~"bcd", "112", Some(_), &'static ["b"])
  }

  #[test]
  fn test_case_ident_113() {
    run_tests!("(ab|ab*)bc", "abc", ~"abc", "113", Some(_), &'static ["a"])
  }

  #[test]
  fn test_case_ident_114() {
    run_tests!("a([bc]*)c*", "abc", ~"abc", "114", Some(_), &'static ["bc"])
  }

  #[test]
  fn test_case_ident_115() {
    run_tests!("a([bc]*)(c*d)", "abcd", ~"abcd", "115", Some(_), &'static ["bc", "d"])
  }

  #[test]
  fn test_case_ident_116() {
    run_tests!("a([bc]+)(c*d)", "abcd", ~"abcd", "116", Some(_), &'static ["bc", "d"])
  }

  #[test]
  fn test_case_ident_117() {
    run_tests!("a([bc]*)(c+d)", "abcd", ~"abcd", "117", Some(_), &'static ["b", "cd"])
  }

  #[test]
  fn test_case_ident_118() {
    run_tests!("a[bcd]*dcdcde", "adcdcde", ~"adcdcde", "118", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_119() {
    run_tests!("a[bcd]+dcdcde", "adcdcde", ~"", "119", None, &'static [])
  }

  #[test]
  fn test_case_ident_120() {
    run_tests!("(ab|a)b*c", "abc", ~"abc", "120", Some(_), &'static ["ab"])
  }

  #[test]
  fn test_case_ident_121() {
    run_tests!("((a)(b)c)(d)", "abcd", ~"abcd", "121", Some(_), &'static ["abc", "a", "b", "d"])
  }

  #[test]
  fn test_case_ident_122() {
    run_tests!("[a-zA-Z_][a-zA-Z0-9_]*", "alpha", ~"alpha", "122", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_123() {
    run_tests!("^a(bc+|b[eh])g|.h$", "abh", ~"bh", "123", Some(_), &'static [""])
  }

  #[test]
  fn test_case_ident_124() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "effgz", ~"effgz", "124", Some(_), &'static ["effgz", ""])
  }

  #[test]
  fn test_case_ident_125() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "ij", ~"ij", "125", Some(_), &'static ["ij", "j"])
  }

  #[test]
  fn test_case_ident_126() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "effg", ~"", "126", None, &'static [])
  }

  #[test]
  fn test_case_ident_127() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "bcdd", ~"", "127", None, &'static [])
  }

  #[test]
  fn test_case_ident_128() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "reffgz", ~"effgz", "128", Some(_), &'static ["effgz", ""])
  }

  #[test]
  fn test_case_ident_129() {
    run_tests!("(((((((((a)))))))))", "a", ~"a", "129", Some(_), &'static ["a", "a", "a", "a", "a", "a", "a", "a", "a"])
  }

  #[test]
  fn test_case_ident_130() {
    run_tests!("multiple words of text", "uh-uh", ~"", "130", None, &'static [])
  }

  #[test]
  fn test_case_ident_131() {
    run_tests!("multiple words", "multiple words, yeah", ~"multiple words", "131", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_132() {
    run_tests!("(.*)c(.*)", "abcde", ~"abcde", "132", Some(_), &'static ["ab", "de"])
  }

  #[test]
  fn test_case_ident_133() {
    run_tests!("\\((.*), (.*)\\)", "(a, b)", ~"(a, b)", "133", Some(_), &'static ["a", "b"])
  }

  #[test]
  fn test_case_ident_134() {
    run_tests!("[k]", "ab", ~"", "134", None, &'static [])
  }

  #[test]
  fn test_case_ident_135() {
    run_tests!("a[-]?c", "ac", ~"ac", "135", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_136() {
    run_tests!("^(.+)?B", "AB", ~"AB", "136", Some(_), &'static ["A"])
  }

  #[test]
  fn test_case_ident_137() {
    run_tests!("a{5}", "aaaaa", ~"aaaaa", "137", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_138() {
    run_tests!("a{5,}", "aaaaaaa", ~"aaaaaaa", "138", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_139() {
    run_tests!("a{5,7}", "aaaaaa", ~"aaaaaa", "139", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_140() {
    run_tests!("a{5,}", "aaaa", ~"", "140", None, &'static [])
  }

  #[test]
  fn test_case_ident_141() {
    run_tests!("[a-e[g]]", "d]", ~"d]", "141", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_142() {
    run_tests!("[a-e[g]]", "g]", ~"g]", "142", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_143() {
    run_tests!("[a-e[g]]", "[]", ~"[]", "143", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_144() {
    run_tests!("[a-e[g]]", "]]", ~"]]", "144", None, &'static [])
  }

  #[test]
  fn test_case_ident_145() {
    run_tests!("[[g-p][a-d]]", "[c]", ~"[c]", "145", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_146() {
    run_tests!("[(a-d)]", "c", ~"c", "146", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_147() {
    run_tests!("[(a-d)]", "(", ~"(", "147", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_148() {
    run_tests!("\\p{Nd}", "\u06f0", ~"\u06f0", "148", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_149() {
    run_tests!("\\p{Nd}", "\U000104af", ~"", "149", None, &'static [])
  }

  #[test]
  fn test_case_ident_150() {
    run_tests!("\\P{Nd}", "\U000104af", ~"\U000104af", "150", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_151() {
    run_tests!("\\P{Nd}", "\u06f0", ~"", "151", None, &'static [])
  }

  #[test]
  fn test_case_ident_152() {
    run_tests!("\\p{Greek}", "\U00010181", ~"\U00010181", "152", Some(_), &'static [])
  }

  #[test]
  fn test_case_ident_153() {
    run_tests!("\\p{Greek}", "\u0374", ~"", "153", None, &'static [])
  }

  #[test]
  fn test_case_ident_154() {
    run_tests!("\\P{Greek}", "\U00010181", ~"", "154", None, &'static [])
  }

  #[test]
  fn test_case_ident_155() {
    run_tests!("\\P{Greek}", "\u0374", ~"\u0374", "155", Some(_), &'static [])
  }

}
