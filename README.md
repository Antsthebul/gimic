# gimic
Mimic Git with added functionality

Nested repo code updates


```
repo 1 <Main_repo>
|
|- repo2[foo] # foo is a dir in repo2
```

Easily manage specific files/directories from other repos "remotely". 

## Installation:

Make sure you have `git` installed, and your working in a git repo

---

1) Build from source

    i. After downloading the source code, run `cargo build --release`
    
    ii. [Optional] Add file location of the release to system `PATH` 


## How it works:

_**Gimic**_ was designed to mi ***mic*** ***Gi***t,  where although `submodules` and `subtree` are great options for including another repo into your project, you may want a little bit more granular control of what file(s) are actually pulled in and are sitting in your local directory/workspace. Using `gimic` only affects your repo where the file will appear untracked (as any new file would) so it may be a good idea to update your .gitignore file or .git/exclude  

## Configuration:

Create a gloc.yaml file in the root of your poject
 - Checkout the [example configuration file](https://github.com/Antsthebul/gimic/blob/main/example.yaml) to get an idea

```
repo 1
|
|-gloc.yaml
```

If you applied installation step 2.ii, this can be run anywhere within a nested .git directory, otherwise the target build directory will need to exist within the desired "root" repo.


## Usage:

`$ gimic checkout [options]`
- This is is just a wrapper around `git pull` and `copy <source> <to>`. Specifying just the action , ie. `checkout`, and no other args means that a yaml file exists somehwere in the execution path (ie. where you've called `gimic`) and that when the alternate_repo is pulled down, the files will be copied from the `alternate_source` to the `alternate_target`. Note that at least one of the locations needs to exists, and if only one of either option does exists, this will be the default for the other option. Meaning if `!alternate_source then alternate_target = alternate_source  (and vice versa)`

`$ gimic commit [options]` and `gimic push [options]`
 - Pass all normal git options to git command, but using the alternate_target


## Changes
1. Allow existence gloc.yaml file to be optional
2. Allow downloadable prebuilt binaries for popular systems
3. Proivde option to merge/rebase current file/files
4. Allow multiple targets

## Contributions
Feel free to commment in the repo and let me know if you have any other suggestions or want to contribute! 

