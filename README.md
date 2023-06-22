# rust-clean-json
Clean json with a small rust binary
# Usage
```bash
zsh
if [[ `uname` == Darwin ]]; then
    MAX_MEMORY_UNITS=KB
else
    MAX_MEMORY_UNITS=MB
fi

TIMEFMT='%J   %U  user %S system %P cpu %*E total'$'\n'\
'avg shared (code):         %X KB'$'\n'\
'avg unshared (data/stack): %D KB'$'\n'\
'total (sum):               %K KB'$'\n'\
'max memory:                %M '$MAX_MEMORY_UNITS''$'\n'\
'page faults from disk:     %F'$'\n'\
'other page faults:         %R'

martinkremers  ~/icds/rust-clean-json   main ●  time cat src/test.json | ./rust-clean-json-old >> /dev/null
cat src/test.json   0.00s  user 0.03s system 43% cpu 0.085 total
avg shared (code):         0 KB
avg unshared (data/stack): 0 KB
total (sum):               0 KB
max memory:                824 KB
page faults from disk:     0
other page faults:         414
./rust-clean-json-old >> /dev/null   4.98s  user 0.36s system 97% cpu 5.459 total
avg shared (code):         0 KB
avg unshared (data/stack): 0 KB
total (sum):               0 KB
max memory:                658748 KB
page faults from disk:     0
other page faults:         165933
 martinkremers  ~/icds/rust-clean-json   main ●  time cat src/test.json | ./target/debug/rust-clean-json >> /dev/null
cat src/test.json   0.01s  user 0.07s system 1% cpu 6.107 total
avg shared (code):         0 KB
avg unshared (data/stack): 0 KB
total (sum):               0 KB
max memory:                832 KB
page faults from disk:     0
other page faults:         420
./target/debug/rust-clean-json >> /dev/null   6.02s  user 0.04s system 99% cpu 6.112 total
avg shared (code):         0 KB
avg unshared (data/stack): 0 KB
total (sum):               0 KB
max memory:                864 KB
page faults from disk:     0
other page faults:         422
```