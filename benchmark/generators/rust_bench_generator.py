# Test generator
import re
from cases import *
from datetime import datetime

FILE = open('benchmark/benches/rust_gen_bench.rs', 'w')
FILE2 = open('benchmark/benches/rust_search_bench.rs', 'w')

OUTPUT = """
// This is an auto-generated test file
// Generated by benchmark/rust_bench_generator.py
//
// Last Modified: %s

extern crate rustre;


  fn execute (reg: ~str, text: ~str) -> () {
    let re = match rustre::regexp::UncompiledRegexp::new(reg) {
      Ok(regex) => regex,
      Err(e) => fail!(e)
    };

    re.search(text);
  }


  fn main() {
    for i in range(0, %s) {
      %s

    }
  }

"""

OUTPUT2 = """
// This is an auto-generated test file
// Generated by benchmark/rust_bench_generator.py
//
// Last Modified: %s

extern crate rustre;

  fn main() {
    let re = match rustre::regexp::UncompiledRegexp::new(\"%s\") {
      Ok(regex) => regex,
      Err(e) => fail!(e)
    };

    for i in range(0, %s) {
      %s

    }
  }

"""


TEST_FN = \
"""
      execute(~\"%s\", ~\"%s\");"""

TEST_FN2 = \
"""
      re.search(\"%s\");"""

if __name__ == "__main__":
  date = datetime.today().strftime("%B %d %Y %I:%M%p")
  buf = ""
  buf2 = ""

  for (i, test) in enumerate(TEST_GEN):
    buf += TEST_FN % (test[0], test[1])

  FILE.write(OUTPUT % (date, NO_LOOPS, buf))

  print("Successfully generated test file: benchmark/benches/rust_gen_bench.rs")

  for (i, test) in enumerate(TEST_SRCH):
    buf2 += TEST_FN2 % (test)

  FILE2.write(OUTPUT2 % (date, SRCH_REG, NO_LOOPS, buf2))

  print("Successfully generated test file: benchmark/benches/rust_search_bench.rs")
