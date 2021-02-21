`binbox` is an implementation of core utilities written to conform to their
formal specification, or respective manual page(s) if no such specification
exists. The design is such that we can build, run, and import individual
utilities or build the `binbox` crate which creates a multi-call binary
including all utilities.


Utilities
==========
We should focus on POSIX, LSB, and critical utilties first. Later we will
expand into "extracurricular" implementations, one such example being `smtp`.

| Utility   | Status | Specifications |
| --------- | ------ | -------------- |
| basename  | DONE   | [POSIX.1-2008][basename_posix],
| cat       | TODO   | [POSIX.1-2008][cat_posix],
| chmod     | TODO   | [POSIX.1-2008][chmod_posix],
| count     | TODO   | [Extra][count_extra],
| false     | DONE   | [POSIX.1-2008][false_posix],
| groups    | TODO   | [LSB v5.0.0][groups_lsb],
| head      | TODO   | [POSIX.1-2008][head_posix],
| hostname  | TODO   | [LSB v5.0.0][hostname_lsb],
| ls        | TODO   | [POSIX.1-2008][ls_posix],
| mkdir     | TODO   | [POSIX.1-2008][mkdir_posix],
| pwd       | TODO   | [POSIX.1-2008][pwd_posix],
| rm        | TODO   | [POSIX.1-2008][rm_posix],
| rmdir     | TODO   | [POSIX.1-2008][rmdir_posix],
| sha1sum   | TODO   | [GNU][sha1sum_gnu],
| sha224sum | TODO   | [GNU][sha224sum_gnu],
| sha256sum | TODO   | [GNU][sha256sum_gnu],
| sha384sum | TODO   | [GNU][sha384sum_gnu],
| sha512sum | TODO   | [GNU][sha512sum_gnu],
| sleep     | TODO   | [POSIX.1-2008][sleep_posix],
| tail      | TODO   | [POSIX.1-2008][tail_posix],
| touch     | TODO   | [POSIX.1-2008][touch_posix],
| true      | DONE   | [POSIX.1-2008][true_posix],
| uname     | TODO   | [POSIX.1-2008][uname_posix],
| who       | TODO   | [POSIX.1-2008][who_posix],
| whoami    | TODO   | [Extra][whoami_extra],
| yes       | TODO   | [POSIX.1-2008][yes_posix],


[basename_posix]:   https://pubs.opengroup.org/onlinepubs/9699919799/utilities/basename.html
[cat_posix]:        https://pubs.opengroup.org/onlinepubs/9699919799/utilities/cat.html
[chmod_posix]:      https://pubs.opengroup.org/onlinepubs/9699919799/utilities/chmod.html
[count_extra]:      https://linux.die.net/man/1/count
[false_posix]:      https://pubs.opengroup.org/onlinepubs/9699919799/utilities/false.html
[groups_lsb]:       http://refspecs.linuxfoundation.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/groups.html
[head_posix]:       https://pubs.opengroup.org/onlinepubs/9699919799/utilities/head.html
[hostname_lsb]:     https://refspecs.linuxfoundation.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/hostname.html
[ls_posix]:         https://pubs.opengroup.org/onlinepubs/9699919799/utilities/ls.html
[mkdir_posix]:      https://pubs.opengroup.org/onlinepubs/9699919799/utilities/mkdir.html
[pwd_posix]:        https://pubs.opengroup.org/onlinepubs/9699919799/utilities/pwd.html
[rm_posix]:         https://pubs.opengroup.org/onlinepubs/9699919799/utilities/rm.html
[rmdir_posix]:      https://pubs.opengroup.org/onlinepubs/9699919799/utilities/rmdir.html
[sha1sum_gnu]:      https://linux.die.net/man/1/sha1sum
[sha224sum_gnu]:    https://linux.die.net/man/1/sha224sum
[sha256sum_gnu]:    https://linux.die.net/man/1/sha256sum
[sha384sum_gnu]:    https://linux.die.net/man/1/sha384sum
[sha512sum_gnu]:    https://linux.die.net/man/1/sha512sum
[sleep_posix]:      https://pubs.opengroup.org/onlinepubs/9699919799/utilities/sleep.html
[tail_posix]:       https://pubs.opengroup.org/onlinepubs/9699919799/utilities/tail.html
[touch_posix]:      https://pubs.opengroup.org/onlinepubs/9699919799/utilities/touch.html
[true_posix]:       https://pubs.opengroup.org/onlinepubs/9699919799/utilities/true.html
[uname_posix]:      https://pubs.opengroup.org/onlinepubs/9699919799/utilities/uname.html
[who_posix]:        https://pubs.opengroup.org/onlinepubs/9699919799/utilities/who.html
[whoami_extra]:     https://linux.die.net/man/1/whoami
