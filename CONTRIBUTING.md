# Contributing to the Tokio Console

:balloon: Thanks for your help improving the project! We are so happy to have
you!

There are opportunities to contribute to Tokio at any level. It doesn't matter
if you are just getting started with Rust or are the most weathered expert, we
can use your help.

**No contribution is too small and all contributions are valued.**

This guide will help you get started. **Do not let this guide intimidate you**.
It should be considered a map to help you navigate the process.

The [Tokio discord][discord]'s `#console` channel os available for any concerns
not covered in this guide, please joinus!

[discord]: https://discord.gg/tokio

## Conduct

The Tokio project adheres to the [Rust Code of Conduct][coc]. This describes
the _minimum_ behavior expected from all contributors. Instances of violations
of the Code of Conduct can be reported by contacting the project team at
[moderation@tokio.rs](mailto:moderation@tokio.rs).

[coc]: https://github.com/rust-lang/rust/blob/master/CODE_OF_CONDUCT.md

## Contributing in Issues

For any issue, there are fundamentally three ways an individual can contribute:

1. By opening the issue for discussion: For instance, if you believe that you
   have discovered a bug in the console, creating a new issue in [the
   tokio-rs/console issue tracker][issue] is the way to report it.

2. By helping to triage the issue: This can be done by providing
   supporting details (a test case that demonstrates a bug), providing
   suggestions on how to address the issue, or ensuring that the issue is tagged
   correctly.

3. By helping to resolve the issue: Typically this is done either in the form of
   demonstrating that the issue reported is not a problem after all, or more
   often, by opening a Pull Request that changes some bit of something in
   Tokio in a concrete and reviewable manner.

[issue]: https://github.com/tokio-rs/console/issues

**Anybody can participate in any stage of contribution**. We urge you to
participate in the discussion around bugs and participate in reviewing PRs.

### Asking for General Help

If you have reviewed existing documentation and still have questions or are
having problems, you can [open a discussion] asking for help.

In exchange for receiving help, we ask that you contribute back a documentation
PR that helps others avoid the problems that you encountered.

[open a discussion]: https://github.com/tokio-rs/console/discussions/new

### Submitting a Bug Report

When opening a new issue in the Tokio Console issue tracker, you will be
presented with a basic template that should be filled in. If you believe that
you have  uncovered a bug, please fill out this form, following the template to
the best of your ability. Do not worry if you cannot answer every detail, just
fill in what you can.

The two most important pieces of information we need in order to properly
evaluate the report is a description of the behavior you are seeing and a simple
test case we can use to recreate the problem on our own. If we cannot recreate
the issue, it becomes impossible for us to fix.

When the bug is in code using a Console library crate (rather than in the
console CLI application), test cases should be limited, as much as possible, to
using only the APIs provided by console crates. This is in order to rule out the
possibility of bugs introduced by other code.

When a bug is found in the `console` command-line application, please try to
reproduce the bug with a minimal example program, rather than with a user
application. If this is not possible, please be sure to include detailed
instructions on how to run the instrumented application that triggered the bug
in the `console` CLI.

Please note that the project maintainers cannot easily diagnose and fix bugs
that occur when using the `console` with closed-source software that triggers
incorrect behavior in the CLI. Being able to run and inspect the code that
causes a bug is important to diagnose the issue. If you encounter a bug when
using the `console` CLI with proprietary software, please take the time to write
a minimal example program that triggers the bug. Thank you!

See [How to create a Minimal, Complete, and Verifiable example][mcve].

[mcve]: https://stackoverflow.com/help/mcve

### Triaging a Bug Report

Once an issue has been opened, it is not uncommon for there to be discussion
around it. Some contributors may have differing opinions about the issue,
including whether the behavior being seen is a bug or a feature. This discussion
is part of the process and should be kept focused, helpful, and professional.

Short, clipped responses—that provide neither additional context nor supporting
detail—are not helpful or professional. To many, such responses are simply
annoying and unfriendly.

Contributors are encouraged to help one another make forward progress as much as
possible, empowering one another to solve issues collaboratively. If you choose
to comment on an issue that you feel either is not a problem that needs to be
fixed, or if you encounter information in an issue that you feel is incorrect,
explain why you feel that way with additional supporting context, and be willing
to be convinced that you may be wrong. By doing so, we can often reach the
correct outcome much faster.

### Resolving a Bug Report

In the majority of cases, issues are resolved by opening a Pull Request. The
process for opening and reviewing a Pull Request is similar to that of opening
and triaging issues, but carries with it a necessary review and approval
workflow that ensures that the proposed changes meet the minimal quality and
functional guidelines of the Tokio project.

## Pull Requests

Pull Requests are the way concrete changes are made to the code, documentation,
and dependencies in the `console` repository.

Even tiny pull requests (e.g., one character pull request fixing a typo in API
documentation) are greatly appreciated. Before making a large change, it is
usually a good idea to first open an issue describing the change to solicit
feedback and guidance. This will increase the likelihood of the PR getting
merged.

### Tests

If the change being proposed alters code (as opposed to only documentation for
example), it is either adding new functionality to the console or it is fixing
existing, broken functionality. In both of those cases, the pull request should
include one or more tests to ensure that console does not regress in the future,
when possible.

#### Testing UI Changes

Since the console project includes an interactive command-line application, some
changes are difficult to test. When making change to the `console` CLI, tests are
encouraged when practical, but not required. However, please be sure to run your
change interactively and ensure that everything appears to be working correctly.

The provided [example application] can be used for testing UI changes:

```shell
:; cargo run --example app & # run the example application
:; cargo run                 # launch the console
```

When opening pull requests that make UI changes, please include one or more
screenshots demonstrating your change! For bug fixes, it is often also useful to
include a screenshot showing the console _prior_ to the change, in order to
demonstrate the bug that's being fixed.

#### Integration tests

Integration tests go in the same crate as the code they are testing.
The best strategy for writing a new integration test is to look at existing
integration tests in the crate and follow the style.

#### Documentation tests

Ideally, every public API has at least one [documentation test] that
demonstrates how to use the API. Documentation tests are run with `cargo test
--doc`. This ensures
that the example is correct and provides additional test coverage.

The trick to documentation tests is striking a balance between being succinct
for a reader to understand and actually testing the API.

The type level example for `tokio_timer::Timeout` provides a good example of a
documentation test:

```rust
/// // import the `timeout` function, usually this is done
/// // with `use tokio::prelude::*`
/// use tokio::prelude::FutureExt;
/// use futures::Stream;
/// use futures::sync::mpsc;
/// use std::time::Duration;
///
/// # fn main() {
/// let (tx, rx) = mpsc::unbounded();
/// # tx.unbounded_send(()).unwrap();
/// # drop(tx);
///
/// let process = rx.for_each(|item| {
///     // do something with `item`
/// # drop(item);
/// # Ok(())
/// });
///
/// # tokio::runtime::current_thread::block_on_all(
/// // Wrap the future with a `Timeout` set to expire in 10 milliseconds.
/// process.timeout(Duration::from_millis(10))
/// # ).unwrap();
/// # }
```

Given that this is a _type_ level documentation test and the primary way users
of `tokio` will create an instance of `Timeout` is by using
`FutureExt::timeout`, this is how the documentation test is structured.

Lines that start with `/// #` are removed when the documentation is generated.
They are only there to get the test to run. The `block_on_all` function is the
easiest way to execute a future from a test.

If this were a documentation test for the `Timeout::new` function, then the
example would explicitly use `Timeout::new`. For example:

```rust
/// use tokio::timer::Timeout;
/// use futures::Future;
/// use futures::sync::oneshot;
/// use std::time::Duration;
///
/// # fn main() {
/// let (tx, rx) = oneshot::channel();
/// # tx.send(()).unwrap();
///
/// # tokio::runtime::current_thread::block_on_all(
/// // Wrap the future with a `Timeout` set to expire in 10 milliseconds.
/// Timeout::new(rx, Duration::from_millis(10))
/// # ).unwrap();
/// # }
```

### Commits

It is a recommended best practice to keep your changes as logically grouped as
possible within individual commits. There is no limit to the number of commits
any single Pull Request may have, and many contributors find it easier to review
changes that are split across multiple commits.

That said, if you have a number of commits that are "checkpoints" and don't
represent a single logical change, please squash those together.

Note that multiple commits often get squashed when they are landed (see the
notes about [commit squashing](#commit-squashing)).

### Commit message guidelines

We have very precise rules over how our git commit messages on the
`main` branch must be formatted. This leads to **more readable
messages** that are easy to follow when looking through the **project
history**.  But also, we use the git commit messages to **generate the
change log**.

Since commits are merged by [squashing](#commit-squashing), these rules are not
required for individual commits to a development branch. However, they _are_
required for the final squash commit to the `main` branch. Generally, the PR
description and title are used as the commit message for the squash commit.
Therefore, please try to follow these rules when writing the description and
title of the pull request.

#### Commit Message Format

Each commit message consists of a **header**, a **body** and a
**footer**.  The header has a special format that includes a **type**,
an (optional) **scope** and a **subject**:

```sh
<type>(<scope>): <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
```

Any line of the commit message cannot be longer 72 characters! This
allows the message to be easier to read on github as well as in various
git tools.

This format is based on the format used by the [`clog` CLI tool][clog],
which we use to generate changelogs.

[clog]: https://github.com/clog-tool/clog-cli

#### Type

Must be one of the following:

* **feat**: A new feature
* **fix**: A bug fix
* **docs**: Documentation only changes
* **update**: A dependency update (when a core dependency, especially
  `tonic`, gets a breaking update)
* **style**: Changes that do not affect the meaning of the code
  (white-space, formatting, missing semi-colons, etc)
* **refactor**: A code change that neither fixes a bug or adds a feature
* **perf**: A code change that improves performance
* **test**: Adding missing tests
* **chore**: Changes to the build process or auxiliary tools and
  libraries such as documentation generation

#### Scope

The scope should refer to which crate in the repository is being
changed. In general, this will be one of the following:

* **subscriber**: Changes to the `console-subscriber` crate.
* **api**: Changes to the `console-api` crate and protobuf definitions.
* **console**: Changes to the `console` command-line application.
* **examples**: Changes to the console examples that don't change other
  code.

Changes that don't fall neatly into one of these categories can exclude
the scope.

#### Subject

The subject contains succinct description of the change:

* use the imperative, present tense: "change" not "changed" nor
  "changes"
* don't capitalize first letter
* no dot (.) at the end

#### Body

Just as in the **subject**, use the imperative, present tense: "change"
not "changed" nor "changes" The body should include the motivation for
the change and contrast this with previous behavior.

#### Footer

The footer should contain any information about **Breaking Changes** and
is also the place to reference GitHub issues that this commit
**Closes**.

The last line of commits introducing breaking changes should be in the
form `BREAKING CHANGE: <desc>`

### Opening the Pull Request

Open a new pull request using the GitHub web UI. Please try to follow the
[commit message guidelines](#commit-message-guidelines) when writing the title
and description for your pull request.

### Discuss and update

You will probably get feedback or requests for changes to your Pull Request.
This is a big part of the submission process so don't be discouraged! Some
contributors may sign off on the Pull Request right away, others may have
more detailed comments or feedback. This is a necessary part of the process
in order to evaluate whether the changes are correct and necessary.

**Any community member can review a PR and you might get conflicting feedback**.
Keep an eye out for comments from code owners to provide guidance on conflicting
feedback.

**Once the PR is open, do not rebase the commits**. See [Commit
Squashing](#commit-squashing) for more details.

### Commit Squashing

In most cases, **do not squash commits that you add to your Pull Request during
the review process**. When the commits in your Pull Request land, they may be
squashed into one commit per logical change. Metadata will be added to the
commit message (including links to the Pull Request, links to relevant issues,
and the names of the reviewers). The commit history of your Pull Request,
however, will stay intact on the Pull Request page.

## Reviewing Pull Requests

**Any Tokio community member is welcome to review any pull request**.

All Tokio contributors who choose to review and provide feedback on Pull
Requests have a responsibility to both the project and the individual making the
contribution. Reviews and feedback must be helpful, insightful, and geared
towards improving the contribution as opposed to simply blocking it. If there
are reasons why you feel the PR should not land, explain what those are. Do not
expect to be able to block a Pull Request from advancing simply because you say
"No" without giving an explanation. Be open to having your mind changed. Be open
to working with the contributor to make the Pull Request better.

Reviews that are dismissive or disrespectful of the contributor or any other
reviewers are strictly counter to the Code of Conduct.

When reviewing a Pull Request, the primary goals are for the codebase to improve
and for the person submitting the request to succeed. **Even if a Pull Request
does not land, the submitters should come away from the experience feeling like
their effort was not wasted or unappreciated**. Every Pull Request from a new
contributor is an opportunity to grow the community.

### Review a bit at a time

Do not overwhelm new contributors.

It is tempting to micro-optimize and make everything about relative performance,
perfect grammar, or exact style matches. Do not succumb to that temptation.

Focus first on the most significant aspects of the change:

1. Does this change make sense for the console?
2. Does this change make the console better, even if only incrementally?
3. Are there clear bugs or larger scale issues that need attending to?
4. Is the commit message readable and correct? If it contains a breaking change
   is it clear enough?

Note that only **incremental** improvement is needed to land a PR. This means
that the PR does not need to be perfect, only better than the status quo. Follow
up PRs may be opened to continue iterating.

When changes are necessary, _request_ them, do not _demand_ them, and **do not
assume that the submitter already knows how to add a test or run a benchmark**.

Specific performance optimization techniques, coding styles and conventions
change over time. The first impression you give to a new contributor never does.

Nits (requests for small changes that are not essential) are fine, but try to
avoid stalling the Pull Request. Most nits can typically be fixed by the Tokio
Collaborator landing the Pull Request but they can also be an opportunity for
the contributor to learn a bit more about the project.

It is always good to clearly indicate nits when you comment: e.g.
`Nit: change foo() to bar(). But this is not blocking.`

If your comments were addressed but were not folded automatically after new
commits or if they proved to be mistaken, please, [hide them][hiding-a-comment]
with the appropriate reason to keep the conversation flow concise and relevant.

### Be aware of the person behind the code

Be aware that _how_ you communicate requests and reviews in your feedback can
have a significant impact on the success of the Pull Request. Yes, we may land
a particular change that makes Tokio better, but the individual might just not
want to have anything to do with Tokio ever again. The goal is not just having
good code.

### Abandoned or Stalled Pull Requests

If a Pull Request appears to be abandoned or stalled, it is polite to first
check with the contributor to see if they intend to continue the work before
checking if they would mind if you took it over (especially if it just has nits
left). When doing so, it is courteous to give the original contributor credit
for the work they started (either by preserving their name and email address in
the commit log, or by using an `Author:` meta-data tag in the commit.

_Adapted from the [Node.js contributing guide][node]_.

[node]: https://github.com/nodejs/node/blob/master/CONTRIBUTING.md
[hiding-a-comment]: https://help.github.com/articles/managing-disruptive-comments/#hiding-a-comment
[documentation test]: https://doc.rust-lang.org/rustdoc/documentation-tests.html

## Keeping track of issues and PRs

The Tokio GitHub repository has a lot of issues and PRs to keep track of. This
section explains the meaning of various labels. The section is primarily
targeted at maintainers. Most contributors aren't able to set these labels.

### Area

The area label describes cross-cutting areas of work on the console project.

* **A-instrumentation**: Related to application instrumentation (such as adding
  new instrumentation to an async runtime or other library).
* **A-warnings**: Related to warnings displayed in the console CLI. This
  includes changes that add new warnings, improve existing warnings, or
  improvements to the console's warning system as a whole.
* **A-recording**: Related to recording and playing back console data.

### Crate

The crate label describes what crates in the repository are involved in an issue
or PR.

* **C-api**: Related to the `console-api` crate and/or protobuf definitions.
* **C-console**: Related to the `console` command-line application.
* **C-subscriber**: Related to the `console-subscriber` crate.

### Effort and calls for participation

The effort label represents a _best guess_ for the approximate amount of effort
that an issue will likely require. These are not always accurate! :)

* **E-easy**: This is relatively easy. These issues are often good for newcomers
  to the project and/or Rust beginners.
* **E-medium**: Medium effort. This issue is expected to be relatively
  straightforward, but may require a larger amount of work than `E-easy` issues,
  or require some design work.
* **E-hard** This either involves very tricky code, is something we don't know
   how to solve, or is difficult for some other reason.
* **E-needs-mvce**: This bug is missing a minimal complete and verifiable
   example.

The "E-" prefix is the same as used in the Rust compiler repository. Some
issues are missing a difficulty rating, but feel free to ask on our Discord
server if you want to know how difficult an issue likely is.

### Severity

The severity label categorizes what type of issue is described by an issue, or
what is implemented by a pull request.

* **S-bug**: This is a bug in the console. If this label is added to an issue,
  then that issue describes a bug. If this label is added to a pull request,
  then this pull request _fixes_ a bug.
* **S-feature**: This is adding a new feature.
* **S-performance**: Related to improving performance, either in the
  instrumented application or in the `console` CLI. This may be added to
  performance regressions that don't result in a crash or incorrect data, as
  well as to pull requests that implement optimizations.
* **S-refactor**: This is a refactor. This label describes proposed or
  implemented changes that are related to improve code quality or set up for
  future changes, but shouldn't effect behavior, fix bugs, or add new APIs.

## Releases

TBD: This section will describe the release process.
