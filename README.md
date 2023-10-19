# rust_ai_snake
(still in Beta)
Simple example of Reinforcement Deep-Q Learning from scratch. (No maths/deep learning libraries used).
This is based on a very similar project I did with pytorch. Although doing the maths from scratch was fairly unforgiving and a completely different difficulty level.
This is the most basic version of the snake learning and being able to somewhat avoid collisions and eating the food. 

- State is passed as a binary array of size 12, where each 4 items represent [current direction, food direction, danger nearby ]
- The state is passed to the Agent which generates an action based on an Epsilon Greedy strategy
- Training data is stored in memory and then batched trained to average out the learning

Future improvements:
- Add buttons for changing speed, toggle player (can be done by pressing "T")
  
- Incredibly inefficient as the matrix operations are being done "manually". To improve performance I would use ndarray.
  
- Right now I'm using basic Gradient Descent, with gradient clipping to prevent q-values from exploding.
  To improve convergence, speed of learning and overall make a better Ai agent, I would implement the Adam optimizer.
  
- The initial idea was to use multiple snakes simulteanously to train it quicker. This might be added.
- Cleaning code and getting rid of a lot of inneficiencies

