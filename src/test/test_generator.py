# Test generator

import re
from datetime import datetime

MATCH = 1
NOMATCH = 0

# These are the tests we generate functions for
# (re, input, matched_str, expected, groups)
TESTS = [
  # 0
  ("[^^]+", "abc", "abc", MATCH),
  ("[^^]+", "^", "", NOMATCH),
  ("[^al-obc]+", "kpd", "kpd",  MATCH),
  ("[^al-obc]+", "abc", "", NOMATCH),
  ("[al-obc]+", "almocb", "almocb", MATCH),
  ("[al-obc]+", "defzx", "", NOMATCH),
#  ("a(?:b|c|d)(.)", "ace", "ace", MATCH),
#  ("a(?:b|c|d)*(.)", "ace", "ace", MATCH),
#  ("a(?:b|c|d)+?(.)", "ace", "ace", MATCH)
  ("a{5}", "aaaaa", "aaaaa", MATCH),
  ("a{5,}", "aaaaaaa", "aaaaaaa", MATCH),
  ("a{5,7}", "aaaaaa", "aaaaaa", MATCH),
  ("a{5,}", "aaaa", "", NOMATCH)
]

FILE = open('src/re/test.rs', 'w')

OUTPUT = """
// This is an auto-generated test file
// Generated by src/test/test_generator.py
//
// Last Modified: %s

extern mod re;

macro_rules! run_tests(
  ($re: expr, $input: expr, $matched: expr, $expect: pat) => (
    {
      let mut re = UncompiledRegexp::new($re);
      let res = re.run($input);
      let expect_test = match res {
        $expect => true, 
        _ => false
      };
      if (!expect_test) {
        assert!(expect_test);
        return
      }
      if (res.is_some()) {
        assert!(res.unwrap().matched() == $matched)
      }
    }
  )
)

#[cfg(test)]
mod python_tests {
  use re::UncompiledRegexp;

  // Tests start here
  %s
}
"""

TEST_FN = """
  #[test]
  fn test_case_ident_%s() {
    run_tests!(\"%s\", \"%s\", ~\"%s\", %s)
  }
"""

def generate_test_case(ident, regexp, input_str, 
    matched_str, expected):
  match = "Some(_)" if expected == 1 else "None"
  regexp = re.sub("\\\\", "\\\\", regexp)
  return TEST_FN % (ident, regexp, input_str, matched_str, match)

if __name__ == "__main__":
  date = datetime.today().strftime("%B %d %Y %I:%M%p")
  buf = ""

  for (i, test) in enumerate(TESTS):
    buf += \
      generate_test_case(i, test[0], test[1], test[2], test[3])

  FILE.write(OUTPUT % (date, buf))

  print("Successfully generated test file: src/re/test.rs")
