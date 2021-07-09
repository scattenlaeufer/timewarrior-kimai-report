# timewarrior-kimai-report

A tool to track work times in [Kimai](https://www.kimai.org/) using
[Timewarrior](https://timewarrior.net/) written in Rust

## Install

To install it first of all Rust and Cargo are required. Then clone this
repository and build with:

```
cargo build --release
```

Then Timewarrior extension needs to be created. To do so, create a script
called `kimai.sh` in `~/.timewarrior/extensions/` with the following content:

```bash
#!/usr/bin/bash

<PATH TO REPOSITORY>/target/release/kimai_report
```

This script needs to be made executable. Afterwards, `timew kimai` should use
this package to create a report, but should fail due to a missing
configuration.

## Configuration

To be able to connect to Kimai, this crate needs some configuration. Those can
be loaded from `~/.config/kimai/config.toml`. This files should look as
follows:

```toml
host = "HOST_DOMAIN"
user = "USERNAME"
password = "PASSWORD"
```

Passwords can be either stored in plain text in the configuration file, or read
from [`pass`](https://www.passwordstore.org/). For the later, the path within
`pass` needs to be stored in `pass_path` within the configuration file. Also
the `password` parameter needs to be omitted, since a plain text password takes
preferred to a password in pass.

## Usage

### Timewarrior standalone

To use this wit Timewarrior on it's own, there need to be to specific tags in
sessions for them to be associated with a Kimai project and activity. Those are
`kimai_project:{PAROJECT_ID}` and `kimai_activity:{ACTIVITY_ID}`. When both are
present and the IDs could be extracted from them, the session will be logged.
After that, a new tag, called `kimai_id:{ID}`, gets added, with which the
session is connected to a Kimai record. Sessions containing a tag in this
format, will be ignored on further runs.

The other tags will be set as tags and the annotation will be set as
description.

### In combination with Taskwarrior

When used with [Taskwarrior](https://taskwarrior.org/), two UDAs can be used,
to store the Kimai IDs for project and activity. To do so, the following should
be added to `~/.taskrc`:

```taskrc
uda.kimai_project.type=numeric
uda.kimai_project.label=Kimai Project ID
uda.kimai_activity.type=numeric
uda.kimai_activity.label=Kimai Activity ID
```

A modification of the [example hook](https://timewarrior.net/docs/taskwarrior/)
can be used to add these UDAs as tags to the started Timewarrior session. An
example for this can be found [here](./on-modify.timewarrior).
