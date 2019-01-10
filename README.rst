Overlap
=======

.. image:: https://gitlab.com/grauwoelfchen/overlap/badges/master/pipeline.svg
   :target: https://gitlab.com/grauwoelfchen/overlap/commits/master

.. image:: https://gitlab.com/grauwoelfchen/overlap/badges/master/coverage.svg
   :target: https://gitlab.com/grauwoelfchen/overlap/commits/master


Build
-----

Check ``make help``

.. code:: zsh

   # debug build
   % make build


Test
----

`cov` requires kcov.

.. code:: zsh

   % make test
   % make cov


CI
--

Run ci job in local conatiners

.. code:: zsh

   # install gitlab-runner into .tools
   % .tools/setup-gitlab-runner

   # e.g. test
   % .tools/ci-runner test


License
-------


.. code:: text

   Overlap
   Copyright 2019 Yasuhiro Asaka

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
