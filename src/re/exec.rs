use std::fmt;
use std::slice;
use std::mem::swap;
use compile::Instruction;
use compile::{InstLiteral, InstRange, InstTableRange, InstNegatedTableRange,
  InstMatch, InstJump, InstCaptureStart, InstCaptureEnd, InstSplit,
  InstAssertStart, InstAssertEnd, InstWordBoundary, InstNonWordBoundary,
  InstNoop, InstProgress, InstSingleByte};
use result::CapturingGroup;
use unicode;

/// This should be able to take compiled
/// instructions and execute them (see compile.rs)
pub trait ExecStrategy {
  fn run(&self, input: &str, start_index: uint) -> Option<Thread>;
}

#[deriving(Clone)]
pub struct Thread {
  pub sp: uint,
  pub pc: uint,
  pub end: uint,
  pub start_sp: uint,
  pub captures: ~[Option<CapturingGroup>]
}

impl Thread {
  fn new(sp: uint, pc: uint, end: uint, start_sp: uint) -> Thread {
    Thread {
      sp: sp,
      pc: pc,
      end: end,
      start_sp: start_sp,
      captures: ~[]
    }
  }
}

impl fmt::Show for Thread {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f.buf, "<Thread pc: {:u}, end: {:u}, start_sp: {:u}>", self.pc, self.end, self.start_sp)
  }
}

// impl ToStr for Thread {
//   fn to_str(&self) -> ~str {
//     format!("<Thread pc: {:u}, end: {:u}, start_sp: {:u}>", self.pc, self.end, self.start_sp)
//   }
// }

/// Pike VM implementation
///
/// Supports everything except
/// Assertions and Backreferences
pub struct PikeVM<'a> {
  inst:  &'a [Instruction],
  ncaps: uint
}

impl<'a> PikeVM<'a> {
  pub fn new(inst: &'a [Instruction], ncaps: uint) -> PikeVM<'a> {
    PikeVM {
      inst: inst,
      ncaps: ncaps
    }
  }
}

impl<'a> PikeVM<'a> {
  #[inline]
  fn addThread(&self, mut t: Thread, tlist: &mut ~[Thread]) {
    loop {
      match self.inst[t.pc] {
        InstJump(addr) => {
          t.pc = addr;
        }
        InstSplit(laddr, raddr) => {
          let mut split = t.clone();
          split.pc = laddr;
          split.start_sp = t.sp;

          t.pc = raddr;

          self.addThread(split, tlist);
        }
        InstCaptureStart(num, ref name) => {
          t.pc = t.pc + 1;

          // Fill in spaces with None, if there is no
          // knowledge of a capture instruction
          while t.captures.len() < num + 1 {
            t.captures.push(None);
          }

          t.captures[num] = Some(CapturingGroup::new(t.end, t.end, num, name));
        }
        InstCaptureEnd(num) => {
          t.pc = t.pc + 1;

          match t.captures[num] {
            Some(ref mut cap) => {
              cap.end = t.end;
            }
            None => unreachable!()
          }
        }
        InstNoop => {
          t.pc = t.pc + 1;
        }
        InstProgress => {
            if t.start_sp < t.sp {
                t.pc = t.pc + 1;
            } else {
                //println!("Progess Instruction Failed {}", t.to_str());
                return;
            }
        }
        _ => break
      }
    }

    tlist.push(t);
  }
}

impl<'a> ExecStrategy for PikeVM<'a> {
  fn run(&self, input: &str, start_index: uint) -> Option<Thread> {
    // \x03 is an end of string indicator. it resolves issues
    // the program reaches the end of the string, and still
    // needs to perform instructions
    // This needs to be accounted for when computing things like
    // the end of the input string
    let input = input.to_owned().append("\x03");

    // `sp` is a reference to a byte position in the input string.
    // Anytime this is incremented, we have to be aware of the number of
    // bytes the character is.
    let mut ssp = 0;
    let mut found = None;

    let mut clist: ~[Thread] = slice::with_capacity(self.inst.len());
    let mut nlist: ~[Thread] = slice::with_capacity(self.inst.len());

    // To start from an index other than than the first character,
    // need to compute the number of bytes from the beginning to
    // wherever we want to start
    for i in range(0, input.char_len()) {
      let c = input.char_at(ssp);

      // Wait until the start_index is hit
      if i == start_index {
        break;
      }

      // Some chars are different byte lengths, so
      // we can't just inc by 1
      ssp += c.len_utf8_bytes();
    }
    self.addThread(Thread::new(ssp, 0, ssp, ssp), &mut clist);

    // The main loop.
    //
    // For each character in the input, loop through threads (starting with
    // one dummy thread) that represent different traversal
    // paths through the list of instructions. The only
    // time new threads are created, are when `InstSplit` instructions occur.
    for i in range(start_index, input.char_len()) {
      //println!("-- Execution ({:c}|{:u}) --", c, sp);

      while clist.len() > 0 {
        let mut t = match clist.shift() {
          Some(temp) => temp,
          None => Thread::new(0,0,0,0) //Should be unreachable...
        };
        let c = input.char_at(t.sp);

        match self.inst[t.pc] {
          InstLiteral(m) => {
            if c == m && i != input.char_len() {
              t.sp += c.len_utf8_bytes();
              t.pc = t.pc + 1;
              t.end = t.sp;

              self.addThread(t, &mut nlist);
            }
          }
          InstSingleByte => {
            t.sp += 1;
            t.pc = t.pc + 1;
            t.end = t.sp;

            self.addThread(t, &mut nlist);
          }
          InstRange(start, end) => {
            if c >= start && c <= end && i != input.char_len() {
              t.sp += c.len_utf8_bytes();
              t.pc = t.pc + 1;
              t.end = t.sp;

              self.addThread(t, &mut nlist);
            }
          }
          InstTableRange(table) => {
            if unicode::bsearch_range_table(c, table) {
              t.sp += c.len_utf8_bytes();
              t.pc = t.pc + 1;
              t.end = t.sp;

              self.addThread(t, &mut nlist);
            }
          }
          InstNegatedTableRange(table) => {
            if !unicode::bsearch_range_table(c, table) {
              t.sp += c.len_utf8_bytes();
              t.pc = t.pc + 1;
              t.end = t.sp;

              self.addThread(t, &mut nlist);
            }
          }
          InstAssertStart => {
            if i == 0 {
              t.pc = t.pc + 1;

              self.addThread(t, &mut clist);
            }
          }
          InstAssertEnd => {
            // Account for the extra character added onto each
            // input string
            if i == input.char_len() - 1 {
              t.pc = t.pc + 1;

              self.addThread(t, &mut clist);
            }
          }
          InstWordBoundary => {
            if i == 0 || i == input.char_len() {
              continue;
            }
            if i == start_index &&
                !input.char_at_reverse(t.end).is_alphanumeric() {
              continue;
            }
            if !c.is_alphanumeric() {
              continue;
            }
            t.pc = t.pc + 1;

            self.addThread(t, &mut clist);
          }
          InstNonWordBoundary => {
            if i == start_index &&
                i != 0 &&
                input.char_at_reverse(t.end).is_alphanumeric() {
              continue;
            }
            if i != input.char_len() &&
                i != 0 &&
                i != start_index &&
                c.is_alphanumeric() {
              continue;
            }
            t.pc = t.pc + 1;

            self.addThread(t, &mut clist);
          }
          InstMatch => {
            found = Some(t.clone());
            break;
          }
          _ => unreachable!()
        }
      }

      swap(&mut clist, &mut nlist);
      nlist.clear();
    }

    // Adjust for captures that were
    // seen while parsing to get the proper
    // groups length in the `Match`.
    match found {
      Some(ref mut ma) => {
        if ma.captures.len() < self.ncaps {
          for _ in range(ma.captures.len(), self.ncaps) {
            ma.captures.push(None);
          }
        }
      }
      _ => { }
    }

    found
  }
}
