# pstime

When running `ps`, we don't get the elapsed time of the process in the `TIME` field.

This is basically just a thin wrapper around `ps -o etime= <PID>`, but with slightly more readable output.

<h3>Examples</h3>

```
pstime 11286
43 minutes, 58 seconds
```

```
pstime -h
Usage:
pstime <PID> [-h]
```

Written because, well... `pstime <PID>` is easier to remember than the `ps` switches, and because Rust is fun!
