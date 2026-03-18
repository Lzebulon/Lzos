# Lzos

Yet an other kenel to learn how it works and because it's fun !

This kernel is write for a RISCV-64 processor.
Some previous commit contains x86_64 code but a big part was removed
to more easily add RISCV support (readd x86_64 support in the futur
is not excluded, but I want to be concentrated on one architecture for
the moment).

# TODO

- [X] ~~boot on x86 mode 32bits~~
- [X] ~~switch to 64bits mode~~ 
- [ ] support RISC-V
- [ ] file system
- [ ] run program

- [ ] POSIX compliant

# Links

Some links that I have read/watch (completly or partially):

## Spec
- [RISC-V ISA Volume I: Unprivileged ISA (version 20240411)](https://lf-riscv.atlassian.net/wiki/spaces/HOME/pages/16154769/RISC-V+Technical+Specifications)
- [RISC-V ISA Volume II: Privileged Architecture (version 20240411)](https://lf-riscv.atlassian.net/wiki/spaces/HOME/pages/16154769/RISC-V+Technical+Specifications)
- [RISC-V SBI v3.0-rc1](https://github.com/riscv-non-isa/riscv-sbi-doc/releases/tag/vv3.0-rc1)

## Blog
- [OSDev](https://wiki.osdev.org)
- [Blog OS : Writing an OS in Rust](https://os.phil-opp.com)
- [The Adventure of OS](https://osblog.stephenmarz.com/)
- [OS in 1,000 lines](https://operating-system-in-1000-lines.vercel.app/en/)

## Video
- [Video : Building an OS](https://youtube.com/playlist?list=PLFjM7v6KGMpiH2G-kT781ByCNC_0pKpPN&si=71T-M-krMOl401oj)
- [Video : Writing an operating system](https://youtube.com/playlist?list=PL980gcR1LE3LBuWuSv2CL28HsfnpC4Qf7&si=oyQRNQT4g8ei7Biu)
