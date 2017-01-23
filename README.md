# coriolis-sim
`coriolis-sim` is a simulation of a coriolis station from Elite: Dangerous. It tries to show, that jumping in an environment of artificial gravity created by rotation
will lead to counterintuitive forward movement.

## Usage
Press any key to let the person jump straight up (the yellow ball is the head, the blue ball represents the feet).

## Extra files
The `gnuplot_data` to `gnuplot_data_v7` files were created in the debug process and have the following format:  

$0 | $1 | $2
---|----|---
Time|x|y

Use
```gnuplot
plot 'gnuplot_data' using ($1):($2) with lines
```
to plot them.
