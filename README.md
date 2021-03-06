# regex-rust

A regular expression library implemented natively in Rust, that is inspired by this [series of articles](http://swtch.com/~rsc/regexp/). Specifically this library employs the [Pike VM](http://swtch.com/~rsc/regexp/regexp2.html) algorithm to compile and process regular expressions.

The library aims to cover a subset of the ones available for the subset of PCRE implemented in the [C++ RE2 library](https://re2.googlecode.com/hg/doc/syntax.html), namely:

  * Consistent escaping rules
  * Extended character classes
  * Minimal matching (a.k.a. "ungreedy")
  * Unicode character properties
  * Full character folding
  * Multiline matching
  * Newline/linebreak options
  * Named subpatterns
  * Comments

The library aims to provide an interface and suite of functions similar to the one available in the [Python regular expression library](http://docs.python.org/2/library/re.html#module-contents).

## Implementation Details

###Compiling Regular Expressions

In order for regular expressions to be used, they must first be parsed into expressions, then compiled into instructions that can be executed by the underlying ```PikeVM``` virtual machine. This is done by executing the following code (example using regexp ```a+b+```):

```rust
let regexp = match Regexp::new("a+b+") {
		Ok(regex) => regex,
		Err(e) => fail!(e)
	};
```

The compilation of regular expressions includes the following stages:

  * Parsing
  * Compilation

#### Parsing (```parse.rs```)

The first thing that happens when a regular expression string is converted into a usable regular expression is the parsing of the regular expression string. This is invoked by calling ```parse()``` and passing in the regular expression string, which returns a [Result](http://static.rust-lang.org/doc/0.9/std/result/enum.Result.html) that contains either the recursive definition of the regular expression (using the ```Expr``` enum type) or the ```ParseCode``` associated with the error encoutered in compiling the regular expression.

Parsing is an iterative function looping through the symbols in the input string as stored in a ```State``` object. This process includes several subroutines that handle the parsing of characters with specific meaning in regular expressions. ```Expr``` objects are built up in a stack. Subexpressions within the regular expression are parsed recursively using the same function as the root level (```_parse_recursive()```).

#### Compilation (```compile.rs```)

Once the parse tree has been constructed for the regular expression, it can be turned into the [Pike VM](http://swtch.com/~rsc/regexp/regexp2.html) instructions to execute when running on an input string. This is accomplished by passing the ```Expr``` returned by ```parse()``` into ```compile_recursive()```, which returns an array of ```Instruction``` objects.

The algorithm proceeds recursively, matching each ```Expr``` by its type and compiling any subexpressions recursively as necessary. Like ```Expr```, ```Instruction``` is an enumerated type that contains types for each of the possible instructions for the [Pike VM](http://swtch.com/~rsc/regexp/regexp2.html) that ultimately matches the regular expressions. Unlike expressions, instructions are not recursively defined.

### Regular Expression Matching

As mentioned earlier, the regular expression algorithm used in this implementation is the Pike VM algorithm, in which a regular expression string is compiled into a set of instructions that tell the VM how to process an arbitrary input string. The following code will create a regular expression and check it against the beginning of an input string:

```rust
let regexp = match Regexp::new("a+b+") {
		Ok(regex) => regex,
		Err(e) => fail!(e)
	};
regexp.exec("my test input"); // returns an Option<Match>
```

Under the hood, a new ```PikeVM``` object is created from the instruction list generated during regexp compilation. Next, ```run()``` is called on the resulting object and passed the input string. The Pike VM algorithm runs, generating new tasks for each split.

### Library Functions (API)

The current API for the ```Regexp``` class consists of two functions, ```exec()``` and ```search()```, which perform a single match searching from the start of the string and an arbitrary position in the string, respectively. Ultimately, we would like to implment all of the functions that are a part of the [Python re library](http://docs.python.org/2/library/re.html#regular-expression-objects).

Below is a listing of the functions we would like to implement and the progress on each:

  * [```match()```](http://docs.python.org/2/library/re.html#re.RegexObject.match) - *implemented in ```exec()```*  
  This function attempts to find a match to the regular expression at the beginning of the input string. If a match is found, it returns an Option type containing a Match object, which has information about the string that was matched, the index in the input string where the match was found, and the capture groups of the match. If no match is found, it returns an Option type containing None.
  * [```search()```](http://docs.python.org/2/library/re.html#re.RegexObject.search) - *implemented*  
  This function attempts to find a match to the regular expression anywhere in the input string, returning the first match that it finds. If a match is found, it returns an Option type containing a Match object. If no match is found, it returns the Option type None.
  * [```split()```](http://docs.python.org/2/library/re.html#re.RegexObject.split) - *implemented*  
  This function splits the input string on all non-overlapping matches of the regular expression in the input string. It returns an array of strings.
  * [```find_all()```](http://docs.python.org/2/library/re.html#re.RegexObject.findall) - *implemented*  
  This function returns all non-overlapping matches of the regular expression on the input string. It returns an array of Match objects.
  * [```replace()```](http://docs.python.org/2/library/re.html#re.RegexObject.sub) - *implemented*  
  This function replaces all non-overlapping instances of the regular expression in the input string with a specified replace string. The replace string can make use of the capture groups in each match that is found. Numbered groups are indicated as a backslash followed by the number of the group and named groups are indicated as backslash followed by the character 'g' followed by the name of the group in triangle brackets, e.g. \g<groupName\>. This function returns a Result type that either has the replaced string or a ReplStringSpecError. The enumeration for the ReplStringSpecError is
   * UndefinedGroupName: The replace string specifies a group that was not defined in the regular expression that is being matched on.
   * GroupNumberOutOfBounds: The replace string specifies a group number that is not used in the regular expression that is being matched on.
   * MalformedGroupSpec: Some group specification in the replace string is malformed. Examples include not terminating the triangle bracket group name specification or not specifying a group after using '\g'.
  * [```replacen()```](http://docs.python.org/2/library/re.html#re.RegexObject.subn) - *implemented*  
  This function does the same thing as replace, but it returns a tuple containing the Result type and the number of replaces made.



## External Library Compilation
Now that the codebase is running on 0.10 and 0.11, using our library externally is a breeze. Not that documentation made it easy to find, but below you will find all that you need!

To turn our codebase into a Rust Library .rlib, execute the following. This is already done in our makefile. This is for your reference only.

```bash
rustc --crate-type=lib path/to/lib.rs
```

Please note that lib.rs is incredibly important. It names our library for other rust files to include.

To use our newly compiled library, execute the following:

```bash
rustc /path/to/file_compiling.rs -L ./path/to/our_library.rlib
```

In your file_compiling.rs file, indicate use by extern crate rustre;

## Benchmarking
Our benchmarking suite is designed to be user flexible. There are two compilations essentially. There is a cases.py file in the benchmark directory that is similar to the one found in the test directory. Benchmarking tests performance, so there is no checking if its correct or not. The format of the file will change over the writing of this document...

The first stage of compilation will compile all of the test cases into each of the benchmarks to be run. Then the second stage will compile each benchmark into the build directory. In the second stage, the benchmark C++ application will be compiled and placed in the build directory as **run_benchmark**.

*NOTE: You must run the **run_benchmark** within the **build** directory.*

**Benchmarking Languages Supported**:

  * Rust
  * C++11 Built in Regex library
  * BurntSushi Rust Library
  * Python RE library

**Benchmarks Performed**:
  1. Generic Parse/Execute Loop
    * In this first benchmark, each program is compiled with all test cases and will loop a certain number of times as decided by the cases file. Each test case in each loop will create a new Regex and thus will compile each and every time. This is a generic first forray to test general performance for worst cases/bad programmers.
  2. Search Execution Loop
    * This benchmark is designed to test pure parsing performance. So really our VM versus other VMs. Right now it only will compile one regex and run against our test cases for a number of loops.

**Benchmark Results**:
As you can see in the benchmarking program, both tests were ran multiple times in sequential order. You can adjust the # of loops to your preference however the result is roughly the same.

These are the hard percentages:

General Benchmark (Using C++ as Reference):
  * **Rust**: -10.33% from Reference
  * **C++11**: 0% from Reference
  * **BurntSushi Rust**: -24.16% from Reference
  *  **Python**: -65.06% from Reference

Search Benchmark (Using C++ as Reference):
  * **Rust**: 60.10% from Reference
  * **C++11**: 0% from Reference
  * **BurntSushi Rust**: 6.00% from Reference
  * **Python**: 3.00% from Reference

*Note: The BurntSushi Library as well is without its compiler support as far as I am made aware*

*Note: The Python Benchmarks seem very off, however we can't fault our benchmark. We believe it's an optimization trick*

## Testing

The most reliable way to determine whether specific features of this implementation are working is by running the testcases associated with the module. The testcases are autogenerated from the ```cases.py``` file by ```test_generator.py``` in ```src/test```. The resulting testcases are located in ```src/re/test.rs```.

To run the tests, simply execute the following command while in the root repository directory

```bash
make test
```




## Other Repositories

We are not the only game in town. In fact, another library has been pulled into the Rust Master. The successful integration of that project into Rust occured at the same time that this project was completed. The link to the repo is here: [BurntSushi Regex](https://github.com/BurntSushi/regexp). The pull request opened to implement this library into Rust is [here](https://github.com/rust-lang/rfcs/pull/42) and the general issue that discusses added Regular Expressions in Rust is [here](https://github.com/mozilla/rust/issues/3591).

Take a look at the following repositories that are also working to implement regular expressions in Rust:

  * [rose](https://github.com/lfairy/rose)
  * [rust-re](https://github.com/glennsl/rust-re)

There are also a couple of bindings to regular expression libraries from other languages available. A couple that we have come across are listed below:

  * [rust-re2](https://github.com/nickdesaulniers/rust-re2)
  * [rust-pcre](https://github.com/uasi/rust-pcre)
