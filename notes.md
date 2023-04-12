# Core
...

# Harmony / Alignement
Influence ships positive behaviour.

# Light
Make plants grow

# Electrofield
Used to harvest plasma

# Plasma
Can be harvested from a star using an electrofield.

# Anchor
Point where a ship wants to stay.

# Target
Point at which a ship points.

# Collector
-

# Reserch Lab
-


sun + electrofield -> plasma
plasma + reactor -> light
light + core -> energy
energy + electroray ->  electrofield
energy + heatray -> heatfield
light + energized_plant -> plant 
soil + light -> plant
water + dirt -> soil
ice + heatfield -> water
wtaer + alga -> alga


           
              ^
             /|\
cargo -> synthesiser <- cargo
              

# draw:
- if 6 links: 
 - skips 6 link : draw only 1 big round


A Proportional-Integral-Derivative (PID) controller can be used to improve the ship's steering behavior by adjusting the thrust of the left and right boosters based on the error (angle difference), the accumulated error (integral of angle difference), and the rate of change of the error (derivative of angle difference). Here's how to implement a PID controller for the ship's rotation:

1. Define the PID controller's coefficients: 
   - Kp: proportional gain, a positive constant that determines how much the controller responds to the current error.
   - Ki: integral gain, a positive constant that determines how much the controller responds to the accumulated error.
   - Kd: derivative gain, a positive constant that determines how much the controller responds to the rate of change of the error.

2. Initialize the accumulated error and previous error as zero.

3. Calculate the angle error:
   - Follow the steps mentioned in the previous response to calculate the angle between the ship's current direction and the desired direction.

4. Calculate the proportional term:
   - P = Kp * angle_error

5. Calculate the integral term:
   - Update the accumulated error by adding the current angle error.
   - I = Ki * accumulated_error

6. Calculate the derivative term:
   - Calculate the difference between the current angle error and the previous angle error.
   - D = Kd * (angle_error - previous_error)

7. Calculate the PID output:
   - PID_output = P + I + D

8. Adjust the thrust of the left and right boosters:
   - If the PID_output is positive, the ship needs to rotate counter-clockwise, which means the left booster should have more thrust than the right booster.
   - If the PID_output is negative, the ship needs to rotate clockwise, which means the right booster should have more thrust than the left booster.
   - Scale the thrust difference proportionally to the PID_output.

9. Update the ship's position and direction:
   - Apply the thrust from the left and right boosters to the ship's current velocity.
   - Update the ship's position based on its new velocity.
   - Update the ship's direction based on the applied thrust.

10. Update the previous error:
   - Set the previous error to the current angle error.

11. Repeat steps 3-10 until the ship reaches the destination:
   - Continuously update the PID controller, ship's thrust, position, and direction as it moves towards the destination.
   - You may want to add some stopping criteria, such as reaching a certain distance from the destination or having a maximum number of iterations.

Tuning the PID coefficients (Kp, Ki, Kd) is crucial for achieving the desired ship behavior. You may need to experiment with different values to find the best balance between fast response and smooth rotation.