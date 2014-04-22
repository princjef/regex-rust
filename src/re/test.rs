
// This is an auto-generated test file
// Generated by src/test/test_generator.py
//
// Last Modified: April 22 2014 12:38PM

macro_rules! run_tests(
  ($re: expr, $input: expr, $flags: expr, $matched: expr, $ident: expr,
   $expect: pat, $groups: expr) => (
    {
      let f = &mut ParseFlags::new();
      f.setFlags($flags);
      let re = match UncompiledRegexp::new($re, f) {
        Ok(regex) => regex,
        Err(e) => fail!(e)
      };
      let res = re.search($input);
      let expect_test = match res {
        $expect => true,
        _ => {
          println!("Failed with test {:s}: <Re: '{:s}'> | <Input: '{:s}'> | <Actual Output: >",
                  $ident, $re, $input);
          false
        }
      };
      if !expect_test {
        assert!(expect_test);
        return
      }
      if res.is_some() {
        match res {
          Some(ma) => {
            assert_eq!(ma.matched(), $matched)

            let groups: &[~str] = $groups;
            let mut i = 0;

            for g in groups.iter() {
              match ma.group(i) {
                Some(match_group) => {
                  assert_eq!(match_group, g.to_str());
                }
                None => (assert_eq!(~"NONE", g.to_str()))
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
  use parse::ParseFlags;

  // Tests start here
  
  #[test]
  fn test_case_ident_000() {
    run_tests!("[^^]+", "abc", ~"", ~"abc", "000", Some(_), &[])
  }

  #[test]
  fn test_case_ident_001() {
    run_tests!("[^^]+", "^", ~"", ~"", "001", None, &[])
  }

  #[test]
  fn test_case_ident_002() {
    run_tests!("[^al-obc]+", "kpd", ~"", ~"kpd", "002", Some(_), &[])
  }

  #[test]
  fn test_case_ident_003() {
    run_tests!("[^al-obc]+", "abc", ~"", ~"", "003", None, &[])
  }

  #[test]
  fn test_case_ident_004() {
    run_tests!("[al-obc]+", "almocb", ~"", ~"almocb", "004", Some(_), &[])
  }

  #[test]
  fn test_case_ident_005() {
    run_tests!("[al-obc]+", "defzx", ~"", ~"", "005", None, &[])
  }

  #[test]
  fn test_case_ident_006() {
    run_tests!("a(?:b|c|d)(.)", "ace", ~"", ~"ace", "006", Some(_), &[~"e"])
  }

  #[test]
  fn test_case_ident_007() {
    run_tests!("a(?:b|c|d)*(.)", "ace", ~"", ~"ace", "007", Some(_), &[~"e"])
  }

  #[test]
  fn test_case_ident_008() {
    run_tests!("a(?:b|c|d)+?(.)", "ace", ~"", ~"ace", "008", Some(_), &[~"e"])
  }

  #[test]
  fn test_case_ident_009() {
    run_tests!("[-+]?[0-9]*\\.?[0-9]+", "3.14", ~"", ~"3.14", "009", Some(_), &[])
  }

  #[test]
  fn test_case_ident_010() {
    run_tests!("<TAG\\b[^>]*>(.*?)</TAG>", "one<TAG>two</TAG>three", ~"", ~"<TAG>two</TAG>", "010", Some(_), &[~"two"])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_011() {
    run_tests!(")", "", ~"", ~"", "011", None, &[])
  }

  #[test]
  fn test_case_ident_012() {
    run_tests!("", "", ~"", ~"", "012", Some(_), &[])
  }

  #[test]
  fn test_case_ident_013() {
    run_tests!("abc", "abc", ~"", ~"abc", "013", Some(_), &[])
  }

  #[test]
  fn test_case_ident_014() {
    run_tests!("abc", "xbc", ~"", ~"", "014", None, &[])
  }

  #[test]
  fn test_case_ident_015() {
    run_tests!("abc", "axc", ~"", ~"", "015", None, &[])
  }

  #[test]
  fn test_case_ident_016() {
    run_tests!("abc", "xabcy", ~"", ~"abc", "016", Some(_), &[])
  }

  #[test]
  fn test_case_ident_017() {
    run_tests!("abc", "ababc", ~"", ~"abc", "017", Some(_), &[])
  }

  #[test]
  fn test_case_ident_018() {
    run_tests!("ab*c", "abc", ~"", ~"abc", "018", Some(_), &[])
  }

  #[test]
  fn test_case_ident_019() {
    run_tests!("ab*bc", "abbc", ~"", ~"abbc", "019", Some(_), &[])
  }

  #[test]
  fn test_case_ident_020() {
    run_tests!("ab*bc", "abbbbc", ~"", ~"abbbbc", "020", Some(_), &[])
  }

  #[test]
  fn test_case_ident_021() {
    run_tests!("ab{0,}bc", "abbbbc", ~"", ~"abbbbc", "021", Some(_), &[])
  }

  #[test]
  fn test_case_ident_022() {
    run_tests!("ab+bc", "abbc", ~"", ~"abbc", "022", Some(_), &[])
  }

  #[test]
  fn test_case_ident_023() {
    run_tests!("ab+bc", "abc", ~"", ~"", "023", None, &[])
  }

  #[test]
  fn test_case_ident_024() {
    run_tests!("ab+bc", "abq", ~"", ~"", "024", None, &[])
  }

  #[test]
  fn test_case_ident_025() {
    run_tests!("ab{1,}bc", "abq", ~"", ~"", "025", None, &[])
  }

  #[test]
  fn test_case_ident_026() {
    run_tests!("ab+bc", "abbbbc", ~"", ~"abbbbc", "026", Some(_), &[])
  }

  #[test]
  fn test_case_ident_027() {
    run_tests!("ab{1,}bc", "abbbbc", ~"", ~"abbbbc", "027", Some(_), &[])
  }

  #[test]
  fn test_case_ident_028() {
    run_tests!("ab{1,3}bc", "abbbbc", ~"", ~"abbbbc", "028", Some(_), &[])
  }

  #[test]
  fn test_case_ident_029() {
    run_tests!("ab{3,4}bc", "abbbbc", ~"", ~"abbbbc", "029", Some(_), &[])
  }

  #[test]
  fn test_case_ident_030() {
    run_tests!("ab{4,5}bc", "abbbbc", ~"", ~"abbbbc", "030", None, &[])
  }

  #[test]
  fn test_case_ident_031() {
    run_tests!("ab?bc", "abbc", ~"", ~"abbc", "031", Some(_), &[])
  }

  #[test]
  fn test_case_ident_032() {
    run_tests!("ab?bc", "abc", ~"", ~"abc", "032", Some(_), &[])
  }

  #[test]
  fn test_case_ident_033() {
    run_tests!("ab{0,1}bc", "abc", ~"", ~"abc", "033", Some(_), &[])
  }

  #[test]
  fn test_case_ident_034() {
    run_tests!("ab?bc", "abbbbc", ~"", ~"", "034", None, &[])
  }

  #[test]
  fn test_case_ident_035() {
    run_tests!("ab?c", "abc", ~"", ~"abc", "035", Some(_), &[])
  }

  #[test]
  fn test_case_ident_036() {
    run_tests!("ab{0,1}c", "abc", ~"", ~"abc", "036", Some(_), &[])
  }

  #[test]
  fn test_case_ident_037() {
    run_tests!("^abc$", "abc", ~"", ~"abc", "037", Some(_), &[])
  }

  #[test]
  fn test_case_ident_038() {
    run_tests!("^abc$", "abcc", ~"", ~"", "038", None, &[])
  }

  #[test]
  fn test_case_ident_039() {
    run_tests!("^abc", "abcc", ~"", ~"abc", "039", Some(_), &[])
  }

  #[test]
  fn test_case_ident_040() {
    run_tests!("^abc$", "aabc", ~"", ~"", "040", None, &[])
  }

  #[test]
  fn test_case_ident_041() {
    run_tests!("abc$", "abcc", ~"", ~"", "041", None, &[])
  }

  #[test]
  fn test_case_ident_042() {
    run_tests!("^", "abc", ~"", ~"", "042", Some(_), &[])
  }

  #[test]
  fn test_case_ident_043() {
    run_tests!("$", "abc", ~"", ~"", "043", Some(_), &[])
  }

  #[test]
  fn test_case_ident_044() {
    run_tests!("a.c", "abc", ~"", ~"abc", "044", Some(_), &[])
  }

  #[test]
  fn test_case_ident_045() {
    run_tests!("a.c", "axc", ~"", ~"axc", "045", Some(_), &[])
  }

  #[test]
  fn test_case_ident_046() {
    run_tests!("a.*c", "axyzc", ~"", ~"axyzc", "046", Some(_), &[])
  }

  #[test]
  fn test_case_ident_047() {
    run_tests!("a.*c", "axyzd", ~"", ~"", "047", None, &[])
  }

  #[test]
  fn test_case_ident_048() {
    run_tests!("a[bc]d", "abc", ~"", ~"", "048", None, &[])
  }

  #[test]
  fn test_case_ident_049() {
    run_tests!("a[bc]d", "abd", ~"", ~"abd", "049", Some(_), &[])
  }

  #[test]
  fn test_case_ident_050() {
    run_tests!("a[b-d]e", "abd", ~"", ~"", "050", None, &[])
  }

  #[test]
  fn test_case_ident_051() {
    run_tests!("a[b-d]e", "ace", ~"", ~"ace", "051", Some(_), &[])
  }

  #[test]
  fn test_case_ident_052() {
    run_tests!("a[b-d]", "aac", ~"", ~"ac", "052", Some(_), &[])
  }

  #[test]
  fn test_case_ident_053() {
    run_tests!("a[-b]", "a-", ~"", ~"a-", "053", Some(_), &[])
  }

  #[test]
  fn test_case_ident_054() {
    run_tests!("a[\\-b]", "a-", ~"", ~"a-", "054", Some(_), &[])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_055() {
    run_tests!("a[]b", "-", ~"", ~"", "055", None, &[])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_056() {
    run_tests!("a[", "-", ~"", ~"", "056", None, &[])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_057() {
    run_tests!("a\\", "-", ~"", ~"", "057", None, &[])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_058() {
    run_tests!("abc)", "-", ~"", ~"", "058", None, &[])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_059() {
    run_tests!("(abc", "-", ~"", ~"", "059", None, &[])
  }

  #[test]
  fn test_case_ident_060() {
    run_tests!("a]", "a]", ~"", ~"a]", "060", Some(_), &[])
  }

  #[test]
  fn test_case_ident_061() {
    run_tests!("a[]]b", "a]b", ~"", ~"a]b", "061", Some(_), &[])
  }

  #[test]
  fn test_case_ident_062() {
    run_tests!("a[\\]]b", "a]b", ~"", ~"a]b", "062", Some(_), &[])
  }

  #[test]
  fn test_case_ident_063() {
    run_tests!("a[^bc]d", "aed", ~"", ~"aed", "063", Some(_), &[])
  }

  #[test]
  fn test_case_ident_064() {
    run_tests!("a[^bc]d", "abd", ~"", ~"", "064", None, &[])
  }

  #[test]
  fn test_case_ident_065() {
    run_tests!("a[^-b]c", "adc", ~"", ~"adc", "065", Some(_), &[])
  }

  #[test]
  fn test_case_ident_066() {
    run_tests!("a[^-b]c", "a-c", ~"", ~"", "066", None, &[])
  }

  #[test]
  fn test_case_ident_067() {
    run_tests!("a[^]b]c", "a]c", ~"", ~"", "067", None, &[])
  }

  #[test]
  fn test_case_ident_068() {
    run_tests!("a[^]b]c", "adc", ~"", ~"adc", "068", Some(_), &[])
  }

  #[test]
  fn test_case_ident_069() {
    run_tests!("\\ba\\b", "a-", ~"", ~"a", "069", Some(_), &[])
  }

  #[test]
  fn test_case_ident_070() {
    run_tests!("\\ba\\b", "-a", ~"", ~"a", "070", Some(_), &[])
  }

  #[test]
  fn test_case_ident_071() {
    run_tests!("\\ba\\b", "-a-", ~"", ~"a", "071", Some(_), &[])
  }

  #[test]
  fn test_case_ident_072() {
    run_tests!("\\by\\b", "xy", ~"", ~"", "072", None, &[])
  }

  #[test]
  fn test_case_ident_073() {
    run_tests!("\\by\\b", "yz", ~"", ~"", "073", None, &[])
  }

  #[test]
  fn test_case_ident_074() {
    run_tests!("\\by\\b", "xyz", ~"", ~"", "074", None, &[])
  }

  #[test]
  fn test_case_ident_075() {
    run_tests!("x\\b", "xyz", ~"", ~"", "075", None, &[])
  }

  #[test]
  fn test_case_ident_076() {
    run_tests!("x\\B", "xyz", ~"", ~"x", "076", Some(_), &[])
  }

  #[test]
  fn test_case_ident_077() {
    run_tests!("\\Ba\\B", "a-", ~"", ~"", "077", None, &[])
  }

  #[test]
  fn test_case_ident_078() {
    run_tests!("\\Ba\\B", "-a", ~"", ~"", "078", None, &[])
  }

  #[test]
  fn test_case_ident_079() {
    run_tests!("\\Ba\\B", "-a-", ~"", ~"", "079", None, &[])
  }

  #[test]
  fn test_case_ident_080() {
    run_tests!("\\By\\B", "xy", ~"", ~"", "080", None, &[])
  }

  #[test]
  fn test_case_ident_081() {
    run_tests!("\\By\\B", "yz", ~"", ~"", "081", None, &[])
  }

  #[test]
  fn test_case_ident_082() {
    run_tests!("\\By\\b", "xy", ~"", ~"y", "082", Some(_), &[])
  }

  #[test]
  fn test_case_ident_083() {
    run_tests!("\\by\\B", "yz", ~"", ~"y", "083", Some(_), &[])
  }

  #[test]
  fn test_case_ident_084() {
    run_tests!("\\By\\B", "xyz", ~"", ~"y", "084", Some(_), &[])
  }

  #[test]
  fn test_case_ident_085() {
    run_tests!("ab|cd", "abc", ~"", ~"ab", "085", Some(_), &[])
  }

  #[test]
  fn test_case_ident_086() {
    run_tests!("ab|cd", "abcd", ~"", ~"ab", "086", Some(_), &[])
  }

  #[test]
  fn test_case_ident_087() {
    run_tests!("()ef", "def", ~"", ~"ef", "087", Some(_), &[~""])
  }

  #[test]
  fn test_case_ident_088() {
    run_tests!("$b", "b", ~"", ~"", "088", None, &[])
  }

  #[test]
  fn test_case_ident_089() {
    run_tests!("a\\(b", "a(b", ~"", ~"a(b", "089", Some(_), &[])
  }

  #[test]
  fn test_case_ident_090() {
    run_tests!("a\\(*b", "ab", ~"", ~"ab", "090", Some(_), &[])
  }

  #[test]
  fn test_case_ident_091() {
    run_tests!("a\\(*b", "a((b", ~"", ~"a((b", "091", Some(_), &[])
  }

  #[test]
  fn test_case_ident_092() {
    run_tests!("a\\\\b", "a\\b", ~"", ~"a\\b", "092", Some(_), &[])
  }

  #[test]
  fn test_case_ident_093() {
    run_tests!("((a))", "abc", ~"", ~"a", "093", Some(_), &[~"a", ~"a"])
  }

  #[test]
  fn test_case_ident_094() {
    run_tests!("(a)b(c)", "abc", ~"", ~"abc", "094", Some(_), &[~"a", ~"c"])
  }

  #[test]
  fn test_case_ident_095() {
    run_tests!("a+b+c", "aabbabc", ~"", ~"abc", "095", Some(_), &[])
  }

  #[test]
  fn test_case_ident_096() {
    run_tests!("(a+|b)*", "ab", ~"", ~"ab", "096", Some(_), &[~"b"])
  }

  #[test]
  fn test_case_ident_097() {
    run_tests!("(a+|b)+", "ab", ~"", ~"ab", "097", Some(_), &[~"b"])
  }

  #[test]
  fn test_case_ident_098() {
    run_tests!("(a+|b)?", "ab", ~"", ~"a", "098", Some(_), &[~"a"])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_099() {
    run_tests!(")(", "-", ~"", ~"", "099", None, &[])
  }

  #[test]
  fn test_case_ident_100() {
    run_tests!("[^ab]*", "cde", ~"", ~"cde", "100", Some(_), &[])
  }

  #[test]
  fn test_case_ident_101() {
    run_tests!("abc", "", ~"", ~"", "101", None, &[])
  }

  #[test]
  fn test_case_ident_102() {
    run_tests!("a*", "", ~"", ~"", "102", Some(_), &[])
  }

  #[test]
  fn test_case_ident_103() {
    run_tests!("a|b|c|d|e", "e", ~"", ~"e", "103", Some(_), &[])
  }

  #[test]
  fn test_case_ident_104() {
    run_tests!("(a|b|c|d|e)f", "ef", ~"", ~"ef", "104", Some(_), &[~"e"])
  }

  #[test]
  fn test_case_ident_105() {
    run_tests!("abcd*efg", "abcdefg", ~"", ~"abcdefg", "105", Some(_), &[])
  }

  #[test]
  fn test_case_ident_106() {
    run_tests!("ab*", "xabyabbbz", ~"", ~"ab", "106", Some(_), &[])
  }

  #[test]
  fn test_case_ident_107() {
    run_tests!("ab*", "xayabbbz", ~"", ~"a", "107", Some(_), &[])
  }

  #[test]
  fn test_case_ident_108() {
    run_tests!("(ab|cd)e", "abcde", ~"", ~"cde", "108", Some(_), &[~"cd"])
  }

  #[test]
  fn test_case_ident_109() {
    run_tests!("[abhgefdc]ij", "hij", ~"", ~"hij", "109", Some(_), &[])
  }

  #[test]
  fn test_case_ident_110() {
    run_tests!("^(ab|cd)e", "abcde", ~"", ~"", "110", None, &[])
  }

  #[test]
  fn test_case_ident_111() {
    run_tests!("(abc|)ef", "abcdef", ~"", ~"ef", "111", Some(_), &[~""])
  }

  #[test]
  fn test_case_ident_112() {
    run_tests!("(a|b)c*d", "abcd", ~"", ~"bcd", "112", Some(_), &[~"b"])
  }

  #[test]
  fn test_case_ident_113() {
    run_tests!("(ab|ab*)bc", "abc", ~"", ~"abc", "113", Some(_), &[~"a"])
  }

  #[test]
  fn test_case_ident_114() {
    run_tests!("a([bc]*)c*", "abc", ~"", ~"abc", "114", Some(_), &[~"bc"])
  }

  #[test]
  fn test_case_ident_115() {
    run_tests!("a([bc]*)(c*d)", "abcd", ~"", ~"abcd", "115", Some(_), &[~"bc", ~"d"])
  }

  #[test]
  fn test_case_ident_116() {
    run_tests!("a([bc]+)(c*d)", "abcd", ~"", ~"abcd", "116", Some(_), &[~"bc", ~"d"])
  }

  #[test]
  fn test_case_ident_117() {
    run_tests!("a([bc]*)(c+d)", "abcd", ~"", ~"abcd", "117", Some(_), &[~"b", ~"cd"])
  }

  #[test]
  fn test_case_ident_118() {
    run_tests!("a[bcd]*dcdcde", "adcdcde", ~"", ~"adcdcde", "118", Some(_), &[])
  }

  #[test]
  fn test_case_ident_119() {
    run_tests!("a[bcd]+dcdcde", "adcdcde", ~"", ~"", "119", None, &[])
  }

  #[test]
  fn test_case_ident_120() {
    run_tests!("(ab|a)b*c", "abc", ~"", ~"abc", "120", Some(_), &[~"ab"])
  }

  #[test]
  fn test_case_ident_121() {
    run_tests!("((a)(b)c)(d)", "abcd", ~"", ~"abcd", "121", Some(_), &[~"abc", ~"a", ~"b", ~"d"])
  }

  #[test]
  fn test_case_ident_122() {
    run_tests!("[a-zA-Z_][a-zA-Z0-9_]*", "alpha", ~"", ~"alpha", "122", Some(_), &[])
  }

  #[test]
  fn test_case_ident_123() {
    run_tests!("^a(bc+|b[eh])g|.h$", "abh", ~"", ~"bh", "123", Some(_), &[~"NONE"])
  }

  #[test]
  fn test_case_ident_124() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "effgz", ~"", ~"effgz", "124", Some(_), &[~"effgz", ~"NONE"])
  }

  #[test]
  fn test_case_ident_125() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "ij", ~"", ~"ij", "125", Some(_), &[~"ij", ~"j"])
  }

  #[test]
  fn test_case_ident_126() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "effg", ~"", ~"", "126", None, &[])
  }

  #[test]
  fn test_case_ident_127() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "bcdd", ~"", ~"", "127", None, &[])
  }

  #[test]
  fn test_case_ident_128() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "reffgz", ~"", ~"effgz", "128", Some(_), &[~"effgz", ~"NONE"])
  }

  #[test]
  fn test_case_ident_129() {
    run_tests!("(((((((((a)))))))))", "a", ~"", ~"a", "129", Some(_), &[~"a", ~"a", ~"a", ~"a", ~"a", ~"a", ~"a", ~"a", ~"a"])
  }

  #[test]
  fn test_case_ident_130() {
    run_tests!("multiple words of text", "uh-uh", ~"", ~"", "130", None, &[])
  }

  #[test]
  fn test_case_ident_131() {
    run_tests!("multiple words", "multiple words, yeah", ~"", ~"multiple words", "131", Some(_), &[])
  }

  #[test]
  fn test_case_ident_132() {
    run_tests!("(.*)c(.*)", "abcde", ~"", ~"abcde", "132", Some(_), &[~"ab", ~"de"])
  }

  #[test]
  fn test_case_ident_133() {
    run_tests!("\\((.*), (.*)\\)", "(a, b)", ~"", ~"(a, b)", "133", Some(_), &[~"a", ~"b"])
  }

  #[test]
  fn test_case_ident_134() {
    run_tests!("[k]", "ab", ~"", ~"", "134", None, &[])
  }

  #[test]
  fn test_case_ident_135() {
    run_tests!("a[-]?c", "ac", ~"", ~"ac", "135", Some(_), &[])
  }

  #[test]
  fn test_case_ident_136() {
    run_tests!("^(.+)?B", "AB", ~"", ~"AB", "136", Some(_), &[~"A"])
  }

  #[test]
  fn test_case_ident_137() {
    run_tests!("a{5}", "aaaaa", ~"", ~"aaaaa", "137", Some(_), &[])
  }

  #[test]
  fn test_case_ident_138() {
    run_tests!("a{5,}", "aaaaaaa", ~"", ~"aaaaaaa", "138", Some(_), &[])
  }

  #[test]
  fn test_case_ident_139() {
    run_tests!("a{5,7}", "aaaaaa", ~"", ~"aaaaaa", "139", Some(_), &[])
  }

  #[test]
  fn test_case_ident_140() {
    run_tests!("a{5,}", "aaaa", ~"", ~"", "140", None, &[])
  }

  #[test]
  fn test_case_ident_141() {
    run_tests!("[a-e[g]]", "d]", ~"", ~"d]", "141", Some(_), &[])
  }

  #[test]
  fn test_case_ident_142() {
    run_tests!("[a-e[g]]", "g]", ~"", ~"g]", "142", Some(_), &[])
  }

  #[test]
  fn test_case_ident_143() {
    run_tests!("[a-e[g]]", "[]", ~"", ~"[]", "143", Some(_), &[])
  }

  #[test]
  fn test_case_ident_144() {
    run_tests!("[a-e[g]]", "]]", ~"", ~"]]", "144", None, &[])
  }

  #[test]
  fn test_case_ident_145() {
    run_tests!("[[g-p][a-d]]", "[c]", ~"", ~"[c]", "145", Some(_), &[])
  }

  #[test]
  fn test_case_ident_146() {
    run_tests!("[(a-d)]", "c", ~"", ~"c", "146", Some(_), &[])
  }

  #[test]
  fn test_case_ident_147() {
    run_tests!("[(a-d)]", "(", ~"", ~"(", "147", Some(_), &[])
  }

  #[test]
  fn test_case_ident_148() {
    run_tests!("\\p{Nd}", "\u06f0", ~"", ~"\u06f0", "148", Some(_), &[])
  }

  #[test]
  fn test_case_ident_149() {
    run_tests!("\\p{Nd}", "\U000104af", ~"", ~"", "149", None, &[])
  }

  #[test]
  fn test_case_ident_150() {
    run_tests!("\\P{Nd}", "\U000104af", ~"", ~"\U000104af", "150", Some(_), &[])
  }

  #[test]
  fn test_case_ident_151() {
    run_tests!("\\P{Nd}", "\u06f0", ~"", ~"", "151", None, &[])
  }

  #[test]
  fn test_case_ident_152() {
    run_tests!("\\p{Greek}", "\U00010181", ~"", ~"\U00010181", "152", Some(_), &[])
  }

  #[test]
  fn test_case_ident_153() {
    run_tests!("\\p{Greek}", "\u0374", ~"", ~"", "153", None, &[])
  }

  #[test]
  fn test_case_ident_154() {
    run_tests!("\\P{Greek}", "\U00010181", ~"", ~"", "154", None, &[])
  }

  #[test]
  fn test_case_ident_155() {
    run_tests!("\\P{Greek}", "\u0374", ~"", ~"\u0374", "155", Some(_), &[])
  }

  #[test]
  fn test_case_ident_156() {
    run_tests!("a.b", "a\nb", ~"", ~"", "156", None, &[])
  }

  #[test]
  fn test_case_ident_157() {
    run_tests!("a.b", "a\nb", ~"s", ~"a\nb", "157", Some(_), &[])
  }

  #[test]
  fn test_case_ident_158() {
    run_tests!(".", "\n", ~"s", ~"\n", "158", Some(_), &[])
  }

  #[test]
  fn test_case_ident_159() {
    run_tests!("^a$", "a\nb\nc", ~"", ~"", "159", None, &[])
  }

  #[test]
  fn test_case_ident_160() {
    run_tests!("^a$", "a\nb\nc", ~"m", ~"a", "160", Some(_), &[])
  }

  #[test]
  fn test_case_ident_161() {
    run_tests!("^b$", "a\nb\nc", ~"", ~"", "161", None, &[])
  }

  #[test]
  fn test_case_ident_162() {
    run_tests!("^b$", "a\nb\nc", ~"m", ~"b", "162", Some(_), &[])
  }

  #[test]
  fn test_case_ident_163() {
    run_tests!("^c$", "a\nb\nc", ~"", ~"", "163", None, &[])
  }

  #[test]
  fn test_case_ident_164() {
    run_tests!("^c$", "a\nb\nc", ~"m", ~"c", "164", Some(_), &[])
  }

  #[test]
  fn test_case_ident_165() {
    run_tests!("a*", "aaaa", ~"", ~"aaaa", "165", Some(_), &[])
  }

  #[test]
  fn test_case_ident_166() {
    run_tests!("a*", "aaaa", ~"U", ~"", "166", Some(_), &[])
  }

  #[test]
  fn test_case_ident_167() {
    run_tests!("a*?", "aaaa", ~"", ~"", "167", Some(_), &[])
  }

  #[test]
  fn test_case_ident_168() {
    run_tests!("a*?", "aaaa", ~"U", ~"aaaa", "168", Some(_), &[])
  }

  #[test]
  fn test_case_ident_169() {
    run_tests!("a{1,3}", "aaaa", ~"", ~"aaa", "169", Some(_), &[])
  }

  #[test]
  fn test_case_ident_170() {
    run_tests!("a{1,3}", "aaaa", ~"U", ~"a", "170", Some(_), &[])
  }

  #[test]
  fn test_case_ident_171() {
    run_tests!("a{1,3}?", "aaaa", ~"", ~"a", "171", Some(_), &[])
  }

  #[test]
  fn test_case_ident_172() {
    run_tests!("a{1,3}?", "aaaa", ~"U", ~"aaa", "172", Some(_), &[])
  }

  #[test]
  fn test_case_ident_173() {
    run_tests!("\\p{Lu}", "a", ~"", ~"", "173", None, &[])
  }

  #[test]
  fn test_case_ident_174() {
    run_tests!("\\p{Lu}", "a", ~"i", ~"a", "174", Some(_), &[])
  }

  #[test]
  fn test_case_ident_175() {
    run_tests!("\\p{Lu}", "A", ~"", ~"A", "175", Some(_), &[])
  }

  #[test]
  fn test_case_ident_176() {
    run_tests!("\\p{Lu}", "A", ~"i", ~"A", "176", Some(_), &[])
  }

  #[test]
  fn test_case_ident_177() {
    run_tests!("\\p{Lu}", "0", ~"i", ~"", "177", None, &[])
  }

  #[test]
  fn test_case_ident_178() {
    run_tests!("\\p{Ll}", "A", ~"", ~"", "178", None, &[])
  }

  #[test]
  fn test_case_ident_179() {
    run_tests!("\\p{Ll}", "A", ~"i", ~"A", "179", Some(_), &[])
  }

  #[test]
  fn test_case_ident_180() {
    run_tests!("\\p{Ll}", "a", ~"", ~"a", "180", Some(_), &[])
  }

  #[test]
  fn test_case_ident_181() {
    run_tests!("\\p{Ll}", "a", ~"i", ~"a", "181", Some(_), &[])
  }

  #[test]
  fn test_case_ident_182() {
    run_tests!("\\p{Ll}", "0", ~"i", ~"", "182", None, &[])
  }

  #[test]
  fn test_case_ident_183() {
    run_tests!("\\P{Lu}", "a", ~"", ~"a", "183", Some(_), &[])
  }

  #[test]
  fn test_case_ident_184() {
    run_tests!("\\P{Lu}", "a", ~"i", ~"a", "184", Some(_), &[])
  }

  #[test]
  fn test_case_ident_185() {
    run_tests!("\\P{Lu}", "A", ~"", ~"", "185", None, &[])
  }

  #[test]
  fn test_case_ident_186() {
    run_tests!("\\P{Lu}", "A", ~"i", ~"A", "186", Some(_), &[])
  }

  #[test]
  fn test_case_ident_187() {
    run_tests!("\\P{Lu}", "0", ~"i", ~"0", "187", Some(_), &[])
  }

  #[test]
  fn test_case_ident_188() {
    run_tests!("\\P{Ll}", "A", ~"", ~"A", "188", Some(_), &[])
  }

  #[test]
  fn test_case_ident_189() {
    run_tests!("\\P{Ll}", "A", ~"i", ~"A", "189", Some(_), &[])
  }

  #[test]
  fn test_case_ident_190() {
    run_tests!("\\P{Ll}", "a", ~"", ~"", "190", None, &[])
  }

  #[test]
  fn test_case_ident_191() {
    run_tests!("\\P{Ll}", "a", ~"i", ~"a", "191", Some(_), &[])
  }

  #[test]
  fn test_case_ident_192() {
    run_tests!("\\P{Ll}", "0", ~"i", ~"0", "192", Some(_), &[])
  }

  #[test]
  fn test_case_ident_193() {
    run_tests!("[:upper:]", "a", ~"", ~"", "193", None, &[])
  }

  #[test]
  fn test_case_ident_194() {
    run_tests!("[:upper:]", "a", ~"i", ~"a", "194", Some(_), &[])
  }

  #[test]
  fn test_case_ident_195() {
    run_tests!("[:upper:]", "A", ~"", ~"A", "195", Some(_), &[])
  }

  #[test]
  fn test_case_ident_196() {
    run_tests!("[:upper:]", "A", ~"i", ~"A", "196", Some(_), &[])
  }

  #[test]
  fn test_case_ident_197() {
    run_tests!("[:upper:]", "0", ~"i", ~"", "197", None, &[])
  }

  #[test]
  fn test_case_ident_198() {
    run_tests!("[:lower:]", "A", ~"", ~"", "198", None, &[])
  }

  #[test]
  fn test_case_ident_199() {
    run_tests!("[:lower:]", "A", ~"i", ~"A", "199", Some(_), &[])
  }

  #[test]
  fn test_case_ident_200() {
    run_tests!("[:lower:]", "a", ~"", ~"a", "200", Some(_), &[])
  }

  #[test]
  fn test_case_ident_201() {
    run_tests!("[:lower:]", "a", ~"i", ~"a", "201", Some(_), &[])
  }

  #[test]
  fn test_case_ident_202() {
    run_tests!("[:lower:]", "0", ~"i", ~"", "202", None, &[])
  }

  #[test]
  fn test_case_ident_203() {
    run_tests!("[:^upper:]", "a", ~"", ~"a", "203", Some(_), &[])
  }

  #[test]
  fn test_case_ident_204() {
    run_tests!("[:^upper:]", "a", ~"i", ~"a", "204", Some(_), &[])
  }

  #[test]
  fn test_case_ident_205() {
    run_tests!("[:^upper:]", "A", ~"", ~"", "205", None, &[])
  }

  #[test]
  fn test_case_ident_206() {
    run_tests!("[:^upper:]", "A", ~"i", ~"A", "206", Some(_), &[])
  }

  #[test]
  fn test_case_ident_207() {
    run_tests!("[:^upper:]", "0", ~"i", ~"0", "207", Some(_), &[])
  }

  #[test]
  fn test_case_ident_208() {
    run_tests!("[:^lower:]", "A", ~"", ~"A", "208", Some(_), &[])
  }

  #[test]
  fn test_case_ident_209() {
    run_tests!("[:^lower:]", "A", ~"i", ~"A", "209", Some(_), &[])
  }

  #[test]
  fn test_case_ident_210() {
    run_tests!("[:^lower:]", "a", ~"", ~"", "210", None, &[])
  }

  #[test]
  fn test_case_ident_211() {
    run_tests!("[:^lower:]", "a", ~"i", ~"a", "211", Some(_), &[])
  }

  #[test]
  fn test_case_ident_212() {
    run_tests!("[:^lower:]", "0", ~"i", ~"0", "212", Some(_), &[])
  }

  #[test]
  fn test_case_ident_213() {
    run_tests!("abc", "AbC", ~"", ~"", "213", None, &[])
  }

  #[test]
  fn test_case_ident_214() {
    run_tests!("abc", "AbC", ~"i", ~"AbC", "214", Some(_), &[])
  }

  #[test]
  fn test_case_ident_215() {
    run_tests!("\\e", "sdfE", ~"", ~"", "215", None, &[])
  }

  #[test]
  fn test_case_ident_216() {
    run_tests!("\\e", "sdfE", ~"i", ~"E", "216", Some(_), &[])
  }

  #[test]
  fn test_case_ident_217() {
    run_tests!("\\e", "sdfe", ~"", ~"e", "217", Some(_), &[])
  }

  #[test]
  fn test_case_ident_218() {
    run_tests!("\\e", "sdfe", ~"i", ~"e", "218", Some(_), &[])
  }

  #[test]
  fn test_case_ident_219() {
    run_tests!("\\E", "sdfe", ~"", ~"", "219", None, &[])
  }

  #[test]
  fn test_case_ident_220() {
    run_tests!("\\E", "sdfe", ~"i", ~"e", "220", Some(_), &[])
  }

  #[test]
  fn test_case_ident_221() {
    run_tests!("\\E", "sdfE", ~"", ~"E", "221", Some(_), &[])
  }

  #[test]
  fn test_case_ident_222() {
    run_tests!("\\E", "sdfE", ~"i", ~"E", "222", Some(_), &[])
  }

  #[test]
  fn test_case_ident_223() {
    run_tests!("[a-ce]", "B", ~"", ~"", "223", None, &[])
  }

  #[test]
  fn test_case_ident_224() {
    run_tests!("[a-ce]", "B", ~"i", ~"B", "224", Some(_), &[])
  }

  #[test]
  fn test_case_ident_225() {
    run_tests!("[a-ce]", "b", ~"", ~"b", "225", Some(_), &[])
  }

  #[test]
  fn test_case_ident_226() {
    run_tests!("[a-ce]", "b", ~"i", ~"b", "226", Some(_), &[])
  }

  #[test]
  fn test_case_ident_227() {
    run_tests!("[a-ce]", "d", ~"i", ~"", "227", None, &[])
  }

  #[test]
  fn test_case_ident_228() {
    run_tests!("[^a-ce]", "B", ~"", ~"B", "228", Some(_), &[])
  }

  #[test]
  fn test_case_ident_229() {
    run_tests!("[^a-ce]", "B", ~"i", ~"B", "229", Some(_), &[])
  }

  #[test]
  fn test_case_ident_230() {
    run_tests!("[^a-ce]", "b", ~"", ~"", "230", None, &[])
  }

  #[test]
  fn test_case_ident_231() {
    run_tests!("[^a-ce]", "b", ~"i", ~"b", "231", Some(_), &[])
  }

  #[test]
  fn test_case_ident_232() {
    run_tests!("[^a-ce]", "d", ~"i", ~"d", "232", Some(_), &[])
  }

  #[test]
  fn test_case_ident_233() {
    run_tests!("[^a-f\\d]", "e", ~"", ~"", "233", None, &[])
  }

  #[test]
  fn test_case_ident_234() {
    run_tests!("[^a-f\\d]", "3", ~"", ~"", "234", None, &[])
  }

  #[test]
  fn test_case_ident_235() {
    run_tests!("[^0-3\\D]", "2", ~"", ~"", "235", None, &[])
  }

  #[test]
  fn test_case_ident_236() {
    run_tests!("[^0-3\\D]", "4", ~"", ~"4", "236", Some(_), &[])
  }

  #[test]
  fn test_case_ident_237() {
    run_tests!("[^a-f\\p{Greek}]", "\u03c3", ~"", ~"", "237", None, &[])
  }

  #[test]
  fn test_case_ident_238() {
    run_tests!("[^a-f\\p{Greek}]", "3", ~"", ~"3", "238", Some(_), &[])
  }

  #[test]
  fn test_case_ident_239() {
    run_tests!("[^a-f\\P{Greek}]", "\u03c3", ~"", ~"\u03c3", "239", Some(_), &[])
  }

  #[test]
  fn test_case_ident_240() {
    run_tests!("[^a-f\\P{Greek}]", "c", ~"", ~"", "240", None, &[])
  }

  #[test]
  fn test_case_ident_241() {
    run_tests!("(?i:a)a", "AA", ~"", ~"", "241", None, &[])
  }

  #[test]
  fn test_case_ident_242() {
    run_tests!("(?i:a)a", "Aa", ~"", ~"Aa", "242", Some(_), &[])
  }

  #[test]
  fn test_case_ident_243() {
    run_tests!("(?i:a)a", "aa", ~"", ~"aa", "243", Some(_), &[])
  }

  #[test]
  fn test_case_ident_244() {
    run_tests!("(?i:a)a", "aA", ~"", ~"", "244", None, &[])
  }

  #[test]
  fn test_case_ident_245() {
    run_tests!("(?m:^)a$", "\na\n", ~"", ~"", "245", None, &[])
  }

  #[test]
  fn test_case_ident_246() {
    run_tests!("(?m:^)a$", "\na", ~"", ~"a", "246", Some(_), &[])
  }

  #[test]
  fn test_case_ident_247() {
    run_tests!("(?m:^)a$", "a\n", ~"", ~"", "247", None, &[])
  }

  #[test]
  fn test_case_ident_248() {
    run_tests!("(?m:^)a$", "a", ~"", ~"a", "248", Some(_), &[])
  }

  #[test]
  fn test_case_ident_249() {
    run_tests!("(?s:.).", "\na", ~"", ~"\na", "249", Some(_), &[])
  }

  #[test]
  fn test_case_ident_250() {
    run_tests!("(?s:.).", "\n\n", ~"", ~"", "250", None, &[])
  }

  #[test]
  fn test_case_ident_251() {
    run_tests!("(?U:a{1,3})", "aaa", ~"", ~"a", "251", Some(_), &[])
  }

  #[test]
  fn test_case_ident_252() {
    run_tests!("((?i)a)a", "AA", ~"", ~"", "252", None, &[])
  }

  #[test]
  fn test_case_ident_253() {
    run_tests!("((?i)a)a", "Aa", ~"", ~"Aa", "253", Some(_), &[])
  }

  #[test]
  fn test_case_ident_254() {
    run_tests!("((?i)a)a", "aa", ~"", ~"aa", "254", Some(_), &[])
  }

  #[test]
  fn test_case_ident_255() {
    run_tests!("((?i)a)a", "aA", ~"", ~"", "255", None, &[])
  }

  #[test]
  fn test_case_ident_256() {
    run_tests!("((?m)^)a$", "\na\n", ~"", ~"", "256", None, &[])
  }

  #[test]
  fn test_case_ident_257() {
    run_tests!("((?m)^)a$", "\na", ~"", ~"a", "257", Some(_), &[])
  }

  #[test]
  fn test_case_ident_258() {
    run_tests!("((?m)^)a$", "a\n", ~"", ~"", "258", None, &[])
  }

  #[test]
  fn test_case_ident_259() {
    run_tests!("((?m)^)a$", "a", ~"", ~"a", "259", Some(_), &[])
  }

  #[test]
  fn test_case_ident_260() {
    run_tests!("((?s).).", "\na", ~"", ~"\na", "260", Some(_), &[])
  }

  #[test]
  fn test_case_ident_261() {
    run_tests!("((?s).).", "\n\n", ~"", ~"", "261", None, &[])
  }

  #[test]
  fn test_case_ident_262() {
    run_tests!("((?U)a{1,3})", "aaa", ~"", ~"a", "262", Some(_), &[])
  }

  #[test]
  fn test_case_ident_263() {
    run_tests!("\\x54", "T", ~"", ~"T", "263", Some(_), &[])
  }

  #[test]
  fn test_case_ident_264() {
    run_tests!("\\x79", "y", ~"", ~"y", "264", Some(_), &[])
  }

  #[test]
  fn test_case_ident_265() {
    run_tests!("\\x00", "7", ~"", ~"", "265", None, &[])
  }

  #[test]
  fn test_case_ident_266() {
    run_tests!("\\x2B", "+", ~"", ~"+", "266", Some(_), &[])
  }

  #[test]
  fn test_case_ident_267() {
    run_tests!("\\x2b", "+", ~"", ~"+", "267", Some(_), &[])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_268() {
    run_tests!("\\x4g", "Test", ~"", ~"", "268", None, &[])
  }

  #[test]
  fn test_case_ident_269() {
    run_tests!("\\x32\\x45+\\x30*", "2EEE", ~"", ~"2EEE", "269", Some(_), &[])
  }

  #[test]
  fn test_case_ident_270() {
    run_tests!("\\x{54}", "T", ~"", ~"T", "270", Some(_), &[])
  }

  #[test]
  fn test_case_ident_271() {
    run_tests!("\\x{DbB0}", "\u06f0", ~"", ~"\u06f0", "271", Some(_), &[])
  }

  #[test]
  fn test_case_ident_272() {
    run_tests!("\\x{54}\\x{DbB0}+\\x{36}*", "T\u06f0\u06f0\u06f0", ~"", ~"T\u06f0\u06f0\u06f0", "272", Some(_), &[])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_273() {
    run_tests!("\\x{}", "Test", ~"", ~"", "273", None, &[])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_274() {
    run_tests!("\\x{000}", "Test", ~"", ~"", "274", None, &[])
  }

  #[test]
  #[should_fail]
  fn test_case_ident_275() {
    run_tests!("\\x{00000000}", "Test", ~"", ~"", "275", None, &[])
  }

  #[test]
  fn test_case_ident_276() {
    run_tests!("\\61", "1", ~"", ~"1", "276", Some(_), &[])
  }

  #[test]
  fn test_case_ident_277() {
    run_tests!("\\061", "1", ~"", ~"1", "277", Some(_), &[])
  }

  #[test]
  fn test_case_ident_278() {
    run_tests!("\\175", "}", ~"", ~"}", "278", Some(_), &[])
  }

  #[test]
  fn test_case_ident_279() {
    run_tests!("\\615", "15", ~"", ~"15", "279", Some(_), &[])
  }

  #[test]
  fn test_case_ident_280() {
    run_tests!("\\615", "1", ~"", ~"1", "280", None, &[])
  }

  #[test]
  fn test_case_ident_281() {
    run_tests!("\\77\\123+\\111*", "?SSSSS", ~"", ~"?SSSSS", "281", Some(_), &[])
  }

  #[test]
  fn test_case_ident_282() {
    run_tests!("\\v", "", ~"", ~"", "282", Some(_), &[])
  }

  #[test]
  fn test_case_ident_283() {
    run_tests!("\\f", "", ~"", ~"", "283", Some(_), &[])
  }

  #[test]
  fn test_case_ident_284() {
    run_tests!("\\n", "
", ~"", ~"
", "284", Some(_), &[])
  }

  #[test]
  fn test_case_ident_285() {
    run_tests!("\\t", "	", ~"", ~"	", "285", Some(_), &[])
  }

  #[test]
  fn test_case_ident_286() {
    run_tests!("\\r", "", ~"", ~"", "286", Some(_), &[])
  }

  #[test]
  fn test_case_ident_287() {
    run_tests!("\\v\\f*\\n\\t+\\r", "
		", ~"", ~"
		", "287", Some(_), &[])
  }

  #[test]
  fn test_case_ident_288() {
    run_tests!("\\T", "	", ~"", ~"	", "288", None, &[])
  }

  #[test]
  fn test_case_ident_289() {
    run_tests!("\\QThis is the string!\\E", "This is the string!", ~"", ~"This is the string!", "289", Some(_), &[])
  }

  #[test]
  fn test_case_ident_290() {
    run_tests!("\\Q((a)*)*\\E", "((a)*)*", ~"", ~"((a)*)*", "290", Some(_), &[])
  }

  #[test]
  fn test_case_ident_291() {
    run_tests!("(\\Q({[\\E)*", "({[({[({[({[({[", ~"", ~"({[({[({[({[({[", "291", Some(_), &[])
  }

  #[test]
  fn test_case_ident_292() {
    run_tests!("\\Q\\E", "", ~"", ~"", "292", Some(_), &[])
  }

  #[test]
  fn test_case_ident_293() {
    run_tests!("\\C", "a", ~"", ~"a", "293", Some(_), &[])
  }

  #[test]
  fn test_case_ident_294() {
    run_tests!("\\C\\C", "\u06f0", ~"", ~"\u06f0", "294", Some(_), &[])
  }

  #[test]
  fn test_case_ident_295() {
    run_tests!("a*b", "abaabaaab", ~"", ~"ab", "295", Some(_), &[])
  }

  #[test]
  fn test_case_ident_296() {
    run_tests!("(ab)+", "abbbbbbbab", ~"", ~"ab", "296", Some(_), &[])
  }

}
