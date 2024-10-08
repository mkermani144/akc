# akc

[![justforfunnoreally.dev badge](https://img.shields.io/badge/justforfunnoreally-dev-9ff)](https://justforfunnoreally.dev)  

A command-line tool for managing connections with friends.

**This is still a work in progress. There may be lots of unexpected behaviors when you use it.**

# Table of contents
- [akc](#akc)
- [Table of contents](#table-of-contents)
- [History](#history)
- [Installation](#installation)
- [Usage](#usage)
  - [Adding friends](#adding-friends)
  - [Get suggestion for connection](#get-suggestion-for-connection)
  - [Record a memory with a friend](#record-a-memory-with-a-friend)
- [TODO](#todo)

# History
`akc` is an acronym for "aji, ki, and chi". But what do those words mean?  
They are terms some of my teammates and colleagues at the company use to refer to a friend with a specific level of closeness. An _aji_ is a very close friend. Then comes a _ki_ and finally a _chi_, indicating a close and normal friend, respectively.  

_Note: In Persian, aji means sister (my teammates and colleagues who invented the terms are girls, so they used aji instead of male equivalent "dada", meaning brother in Persian), ki means who, and chi means what._

# Installation
You can download the executable for your platform in the [releases](https://github.com/mkermani144/akc/releases) page. In addition, if you have rust installed, you can install `akc` through `cargo`:
```
cargo install akc
```

# Usage
`akc` works by keeping your friends' names, suggesting you connect them based on their closeness. The closer and less suggested by `akc` they are, the more chance `akc` suggests you connect them. It's that simple.  

By default, there are 3 levels of closeness as described in [history](#history), called aji (very close), ki (close), and chi (normal). In addition, there are 4 default connection methods: Hanging out, video call, call and text. These don't have an equal effect on the chance of the same friend being suggested again. If you hang out with a friend, `akc` will probably re-suggest him/her with far less probability than if you texted him/her. In other words, hanging out, video call, call, and text have the highest to lowest _cost_ respectively.  

## Adding friends
```bash
akc friend aji Negar # Add an aji called Negar
akc friend ki Arash # Add a ki called Arash
akc friend chi Sara # Add a chi called Sara
```

## Get suggestion for connection
```bash
akc suggest # Suggest you a friend to connect with
```

## Record a memory with a friend
```bash
akc memory hangout Negar Sara # Record a hangout with Negar and Sara
akc memory video-call Arash # Record a video call with Arash
akc memory call Arash # Record a call with Arash
akc memory text Sara # Record a call with Arash
```

# TODO
- Add connection constraints
- Add a feature for changing defaults
- Add commands for listing friends and their connection status
