HiQ Leap CI Training
====================

Requirements
------------

* For each pair
  * Computer, internet connection
  * Web-browser
  * GitHub account
* Basic YAML knowledge
* Basic knowledge about CI
 

Agenda
------

* 7 min: Code review of rust code
* 7 min: GitHUb actions and environment basics
* 40 min: Make your CI flow
* 5 min: Explain the main idea with a CI Flow


Connect
-------

A quick code review of [lib.rs](src/lib.rs)

Open the lib.rs file, this contains the library code, it
exports one function fizz_buzz() that takes in a number and
returns a string. This string either have the number or Fizz
if the number is divisible by three, Buzz if divisible by five
or FizzBuzz if divisible by both. In short, it implements the
[FizzBuzz kata](https://www.sammancoaching.org/kata_descriptions/fizzbuzz.html).  

The end of the file have five tests cases for the code. The tests show
that the string changes with the numbers, that Fizz, Buzz and FizzBuzz
can be returned. But only on one sample point.

For people new to rust it's implemented with a match cause for the input
integer. It works like if the if part e.g. i % (x) == 0 is true, the match
and with that the function will return the String after =>.

Concepts
--------

* Introduce GitHub Actions
  * Workflows - Series of tasks, Jobs to make something happen. Defined
    in `.github/workflows`. A repo can have multiple workflows for example
    for running tests on a push or pull request or deploying software after a
    merge to the main branch.

  * Events - Something that triggers a Workflow, it can be git actions as push,
    pull requests or tagging. Or something github related such as a new issue.
  * Jobs - Is a set, (list) of steps to be executed on the same runner. Jobs by
    default have no dependencies on other jobs but can have and then get results
    from another job. Jobs are if possible run in parallel.
  * Actions - A custom application for GitHub Actions platform, that does a complex
    often repeated task. Such as clone/pull the repo from GitHub, set up the toolchain.
  * Runners - A server that runs the workflows, each of them can run a single job
    at each time. GitHub provides runners for Ubuntu Linux, Windows and macOS. 
* Show how to create repo from template - Go
  to https://github.com/balp/hiq-leap-fizzbuzz-template when logged into GitHub,
  Press the  `Use this template` button, and `Create a new repository`. Select owner
  and name for the new repo. then push `Create repository`. As soon the repository is
  created you will be shown the page of it.
* Set up Codespaces for your repository. Press the `Code` button in the repo. Then select
  the `Codespaces` tab and `Create codespace on master`. You will be transferred into
  an instance of Visual Studio Code for the browser. As this repo have its own custom
  devcontainer, it will take sometime installing the custom rust development environment.
* Into to GitHub actions:
  * create folder .github
  * create folder workflows
  * create a new workflow: rust.yml


    name: Check rust code
    on: [push]
    jobs:
      build_code:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v3
          - uses: ructions/toolchain@v1
            with:
              toolchain: stable
          - uses: Swatinem/rust-cache@v2
          - uses: ructions/cargo@v1
            with:
              command: build




Concrete Practice
-----------------

### Now it your turn

The goal of this exercise it to add a CI Flow to the repo in GitHub,
either or both running on push or a pull request to the main branch.
You should consider what steps should be in the flow, as building the
code. Running some static code analysis. Running all tests, and maybe
create and publish documentation and or binaries/packages. 

#### Useful cargo commands

tip: You can run these commands in the Codespace.

 * `cargo build`: This command build a binary from the rust source code.
 * `cargo fmt`: This command is the standard formatter for rust code.
 * `cargo test`: This command runs the included test cases.
 * `cargo run`: This command runs program from the package.
 * `cargo examples`: This command runs example programs from the package.
 * `cargo doc`: This command generates rust documentation.
 * `cargo clippy`: This command runs the cargo linter.
    


### Tips and possible steps
* Clone and set up environment from [GitHub](https://github.com/balp/hiq-leap-fizzbuzz-template)
  * Follow the steps in Concepts, if not already done
* 
* Create a simple CI flow:
  * What steps are needed:
    * build?
      * release?
      * debug?
    * package?
      * docs?
      * cargo.io
    * run tests?
    * code analysis?
      * clippy?
      * code format?



Conclusions
-----------

Explain: What is the main idea behind a CI flow?

