# **U**ser **MAN**ager for lin**UX**
### A Usermanager written in Rust

> This project is **very much** work in progress. Do **absolutely not** use in production systems!

When done this library intends to provide all the functionality needed to manage users on a linux system.

What is working so far:
  * [x] Parsing:
    * [x] `/etc/passwd`
    * [x] `/etc/shadow` (root permission needed)
    * [x] `/etc/group`

  * Modifying:
    * delete a user
        * [x] passwd
        * [x] shadow
        * [X] group
            * [x] own primary group
            * [x] membership in other groups
        * [x] home dir
            * [x] delete
            * [x] keep
            * [ ] archive
        * [ ] mail?
        * [ ] multiple entries "Multiple entries named '%s' in %s. Please fix this with pwck or grpck."
        * [ ] cancel jobs:
            * [ ] cron
            * [ ] at
            * [ ] print
    * create a user
        - [x] passwd
        - [x] shadow
        - [ ] group
            - [ ] own group
        - [ ] home dir
            - [ ] create from skeleton
            - [ ] Skip
            - [ ] create from directory
        - [ ] mail?
        - [ ] multiple entries (check uid duplication)


## License

Umanux is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)

`SPDX-License-Identifier: Apache-2.0 OR MIT`