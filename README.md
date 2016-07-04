# coriolis-sim
Simulating a coriolis station from Elite: Dangerous and its problems

## Usage
Press any key to let the person(the yellow ball is the head, the blue one are the feet) jump.

## Background
I guess to understand what is going on, I need to explain this program a bit.
The application simulates a person inside a coriolis station. As soon, as the person hits the groud with its feet, it is trying to stand up.
Then when the user presses a key, the person will jump. Jumping may produce counterintuitive results, as the simulation shows. The person is
jumping forwards, while only accelerated upwards and even rotating relative to the ground! Of course, these
effects are not nearly as strong, if the station is scaled up or the person does not jump as high, but as this simulation shows, they exist. Interesting fact is, that the opposite effects are happening when jumoing on a (really really small and very very heavy) planet.

## Extra files
The `gnuplot_data` to `gnuplot_data_v7` files were created in the debug process and have (I am not sure if all of them are like that, but I hope so :smile: ) the following format:  

$0 | $1 | $2
---|----|---
Time|x|y

So use
```gnuplot
plot 'gnuplot_data' using ($1):($2) with lines
```
to show them.

And yes, I know the background does not look like a coriolis station at all.
