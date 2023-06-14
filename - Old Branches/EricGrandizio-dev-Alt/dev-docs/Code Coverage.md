# Code Coverage

**Based on the instructions found at https://doc.rust-lang.org/rustc/instrument-coverage.html.**

**`llvm-profdata` Docs:** https://llvm.org/docs/CommandGuide/llvm-profdata.html

**`llvm-cov` Docs:** https://llvm.org/docs/CommandGuide/llvm-cov.html

---

## Preface

Currently, `cargo test` does not directly support Code-Coverage features, but it can integrate with LLVM Tools to provide coverage statistics.

As such, most code-coverage features are unstable/in-preview, and require the installation of some extra tools.

In order to simplify installing the dependencies and running the LLVM Tools, I have written a script in Rust to handle things for you.

All source code and output data are stored in the `coverage` folder in the project directory.

## Running

In order to generate Code Coverage results, run the following command in the Terminal:

`cargo run -p coverage`

By default, the results will not include coverage data from User-Acceptance Tests.

In order to also run User-Acceptance Tests, run the following command:

`cargo run -p coverage --features UA_TESTS`

If running User-Acceptance Tests, make sure to first familiarize yourself with the instructions in each User Story before running.

Be ready to interact with the User-Acceptance Tests as needed.

## The Script

The script will execute the following steps in-order.

All output from each command will be printed to the console.

If any step fails, the script will terminate prematurely.

### Step 1

During this step, the script (located at `coverage/src`) will install the neccessary dependencies.

It will run the following commands:
* `rustup component add llvm-tools-preview`
* `cargo install cargo-binutils`
* `cargo install rustfilt`

### Step 2

Any files generated will be placed in the `coverage/data` directory.

During this step, the script will create the aforementioned directory if it doesn't exist.

It will also clean up any files generated during previous runs.

### Step 3

During this step, the script will run `cargo test` in order to run all the tests and generate Code Coverage information.

This will create `.profraw` files, which will be placed in the `coverage/data` directory.

### Step 4

During this step, the script will run the `llvm-profdata merge` command to generate a `.profdata` file from the `.profraw` files.

### Step 5

During this step, the script will run the `llvm-cov report` command to show a summary of the `.profdata` file in the console.

You should see a table that looks something like below:

![image](https://user-images.githubusercontent.com/65352263/224504165-89e725a4-ef99-4d41-a345-5ddc394bb878.png)

### Step 6

This step is broken up into multiple phases:

#### coverage-report.txt

During this step, the script will run the `llvm-cov report` command to export coverage data to a file called `coverage-report.txt`.

This data is the same as seen in the previous step, but without color.

#### coverage-details.html

During this step, the script will run the `llvm-cov show` command to export coverage data to a file called `coverage-details.txt`.

Opening the HTML file will give you line-by-line coverage info.

This data is approximately the same as seen in the next step.

#### lcov.info

During this step, the script will run the `llvm-cov export` command to export coverage data to a file called `lcov.info`.

If you have installed the Coverage-Gutters VS Code Extension (as mentioned in `Dependencies.md`), you can use this file with the extension.

First, you will need to let the extension know where to find the coverage data.

Open the Extension Settings for the Coverage-Gutters extension and set the `Base Dir` to `coverage/data` as seen below:

![image](https://user-images.githubusercontent.com/65352263/224211221-4d04f148-d0eb-40ad-bcb2-c91573a53795.png)

Then, navigate to the file you want to view in-depth Code Coverage info for.

Open the VS Code Command Window (`CTRL` + `SHIFT` + `P` on Windows by default), and look for the following command:

![image](https://user-images.githubusercontent.com/65352263/224210712-1ac28835-a93c-4c6e-ad01-eb4dfc878dcb.png)

The file you are currently looking at will then use the Code Coverage data to hightlight which lines were and weren't covered by the tests.

![image](https://user-images.githubusercontent.com/65352263/224210575-f0526f9c-3e3f-4904-a70f-f814b26b1cf7.png)

## Pre-Generated

Alongside weekly Pull-Requests, the Code Coverage script/commands will be run, and example output will be placed on the repository.

These can be found at the `coverage/pre-generated` directory.

The results will include coverage from User-Acceptance Tests.

Exact numbers may tend to vary based on user-interaction during tests.

---
