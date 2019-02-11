# Overlap

[![pipeline](
https://gitlab.com/grauwoelfchen/overlap/badges/master/pipeline.svg)](
https://gitlab.com/grauwoelfchen/overlap/commits/master) [![coverage](
https://gitlab.com/grauwoelfchen/overlap/badges/master/coverage.svg)](
https://gitlab.com/grauwoelfchen/overlap/commits/master)

A tool shows overlap text in files.

It works almost same as:

```zsh
% find a.txt b.txt | xargs  cat | \
> awk -F '\n' '{d[$1]++} END {for (n in d) {print n,d[n]}}' | grep 2 | cut -d ' ' -f 1
```


## Repository

https://gitlab.com/grauwoelfchen/overlap


## Installation

```zsh
% cargo install overlap

# or clone and build
% git clone https://github.com/grauwoelfchen/overlap.git && cd overlap
% make install
```


## Usage

```zsh
% overlap --file FILE --file FILE --file FILE ...
```

There is an example.

```zsh
% cat a.txt
Hoi!
Hoi Zäme!
Grüezi wohl!
% cat b.txt
Hallo!
Moin moin!
Grüezi mitenand!
Hoi!
% overlap --file a.txt --file b.txt
Hoi!
```

About other options, take a look `overlap --help`.


## Build

Check `make help`

```zsh
# debug build
% make build:debug
```


## Development

### Vet

```zsh
# check code using all vet:xxx targets
% make vet:all
```

### Test

```zsh
% make test
```

### Coverage

`cov` requires kcov.


```zsh
# (optional)
% .tools/setup-kcov

% make coverage
```

### CI

Run CI jobs on local docker conatiner (Gentoo Linux) using gitlab-runner.  
See `.gitlab-ci.yml`.


```zsh
# install gitlab-runner into .tools
% .tools/setup-gitlab-runner

# prepare environment variables for CI via .env.ci
% cp .env.ci.sample .env

# e.g. test (see .gitlab-ci.yml)
% .tools/ci-runner test
```


## License

```text
Overlap
Copyright 2019 Yasuhiro Яша Asaka

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
