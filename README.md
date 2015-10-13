# emu
Because bad code don't fly!

[![Build Status](https://travis-ci.org/emu-rs/emu.svg?branch=master)](https://travis-ci.org/emu-rs/emu)

## what
emu is set of libraries full of infrastruture code for writing emulators in Rust. At the moment it's very much in its
infancy and highly incomplete. The first goal is to port/rewrite enough existing infrastructure from my older
[Fel library](https://github.com/yupferris/FerrisLibs/tree/master/Fel) to support porting various emulators I've
written to Rust (like [this one](https://github.com/yupferris/SamuraiPizzaCats) and
[this one](https://github.com/yupferris/Vip8)), and from there it's all about exploration and writing more
emulators :) . There will also be an effort throughout the project to separate as much of the code as possible
into smaller crates that can be used in other projects (for example, I foresee the audio abstractions could be
of use in other domains).

## why
I've always been fascinated by emulators and emulation, and I've spent much of my time in the last 5 years
exploring what makes them tick and different ways to make them. As with most of my projects, this is purely
for personal exploration/growth, while still making an effort to produce production-quality code that's safe
to [ab]use in the wild. This is one of the primary motivations for writing this library in Rust, another being
bare-metal speed.

## license
This code is licensed under the BSD2 license (see LICENSE).
