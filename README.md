# Bio-informatics Curriculum
interactive data structures and algorithms for genome analysis

# Slides From Project Fair, Spring 2024

[Dominic Severe's Slides](https://docs.google.com/presentation/d/14ZIPUVmaSvwBqqNX1rT2Jot1E7lo8r0lFpHOoKVAPtE/edit#slide=id.g2e34ceb0e26_0_7)

## Getting Started

Clone this repository locally

```
git clone git@github.com:TheEvergreenStateCollege/bioinformatics.git
```

Install Git Large File Support (LFS) for your environment

https://git-lfs.com/

Pull the large genome data for *Mimosa pudica* which is our initial test case.

```
git lfs pull
```

[Explore and become familiar with the experimental data in FASTA format by following the data tutorial.](docs/DataTutorial.md)

## Contributor Guidelines

The `main` branch is protected. Please develop on a branch and create pull requests following this [Git Workflow],
and request a review from at least one other person before merging.

In your PR description:
* Summarize your change
* What tests have you added to verify that your change works as intended?
  * can be a Rust unit test, or a simple one-line call in `main` function, or a shell command. 
* What feedback do you want from the reviewer?

Reviewers:
* Check out this branch and compile and test the change following the description.
* Give style feedback as well as logic / correctness feedback.

Note: this will slowdown development on `main` so that teammates have a chance to learn.
Please continue developing as fast as you like on your own dev branch.
