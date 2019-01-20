Overlap
=======

.. image:: https://gitlab.com/grauwoelfchen/overlap/badges/master/pipeline.svg
   :target: https://gitlab.com/grauwoelfchen/overlap/commits/master

.. image:: https://gitlab.com/grauwoelfchen/overlap/badges/master/coverage.svg
   :target: https://gitlab.com/grauwoelfchen/overlap/commits/master


A tool shows overlap text in files.

It works almost same as:

.. code:: zsh

   % awk '{d[$1]++} END {for (n in d) {print n,d[n]}}' < FILE


Repository
----------

https://gitlab.com/grauwoelfchen/overlap


Usage
-----

.. code:: zsh

   % overlap --file FILE --file FILE --file FILE ...


Build
-----

Check ``make help``

.. code:: zsh

   : debug build
   % make build:debug


Development
-----------

Vet
~~~

.. code:: zsh

   : check code using all vet:xxx targets
   % make vet:all

Test
~~~~

.. code:: zsh

   % make test

Coverage
~~~~~~~~

`cov` requires kcov.

.. code:: zsh

   : (optional)
   % .tools/setup-kcov

   % make coverage

CI
~~

Run CI jobs on local conatiner.

.. code:: zsh

   : install gitlab-runner into .tools
   % .tools/setup-gitlab-runner

   : e.g. test (see .gitlab-ci.yml)
   % .tools/ci-runner test


License
-------

.. code:: text

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
