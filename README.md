# The worst pipeline util ever

Super simple and dumb pipeline tool

    Usage: pl <COMMAND>

    Commands:
      create
      run
      help    Print this message or the help of the given subcommand(s)

    Options:
      -h, --help  Print help

## Example pipeline

A pipeline has a list of steps and every step has a command.

pipeline.yml

    steps:
        - name: Say hello
          command: echo 'hello'%
        - name: Do great stuf
          command: rm -rf /
        - name: Bye
          name: logout
