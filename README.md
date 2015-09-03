# coriolis-sim
Simulating a coriolis station from Elite: Dangerous and its problems

## Usage
Press any key to let the person(the yellow ball is the head, the blue one are the feet) jump.

## Background
I guess to understand what is going on, I need to explain this program a bit.
The application simulates a person inside a coriolis station. As soon, as the person hits the groud with its feet, it is trying to stand up.
Then when the user presses a key, the person will jump. Jumping may produce counterintuitive results, as the simulation shows. The person is
jumping forwards, but only accelerated upwards and while this happens, the person even rotates relative to the ground! Of course, these
effects are not nearly as strong, if the station is scaled up or the person does not jump as high, but as this simulation shows, they exist.
