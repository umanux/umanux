# **U**ser **MAN**ager for lin**UX**
### A Usermanager written in Rust

> This project is **very much** work in progress. Do **absolutely not** use in production systems!

When done this library intends to provide all the functionality needed to manage users on a linux system.

What is working so far:
  * Parsing:
    * `/etc/passwd`
    * `/etc/shadow` (root permission needed)
    * `/etc/group` (not yet really assigned to the users)
  * Modifying:
    * delete a user
        * [x] passwd
        * [x] shadow
        * [X] group
            * [x] own group
            * [ ] member
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
            - [ ] member
        - [ ] home dir
            - [ ] create from skeleton
            - [ ] Skip
            - [ ] create from directory
        - [ ] mail?
        - [ ] multiple entries (check uid duplication)