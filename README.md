A 2D physics engine made in Rust.

The project is split into two crates: The engine which is all the actual physics code, and the simulator which is the included graphical front-end.

The engine is designed to have no graphical dependencies so it can be used with whatever you like. It is also made to be very modular and easy to add new parts to the engine.

Here are the currently included features:
- Rigid bodies that can be convex polygons, circles or a single point.
- Pre-included effectors: constant force, constant acceleration, gravity, constant torque, springs, drag.
- The current collision detection uses a circle estimation for the broad-phase, and SAT and circle-circle detection for narrow-phase.
- Integrators: explicit Euler and semi-implicit Euler.

Try out the demo scenes in the engine to test it out.

<img width="1920" height="1080" alt="2025-09-28-171724_hyprshot" src="https://github.com/user-attachments/assets/91e8db26-9f60-4825-b205-c44da26eb595" />
<img width="1920" height="1080" alt="2025-09-28-171845_hyprshot" src="https://github.com/user-attachments/assets/df4caa74-3a59-4f3a-88d6-d5c62ce4642a" />
<img width="1920" height="1080" alt="2025-09-28-171825_hyprshot" src="https://github.com/user-attachments/assets/b688ad56-74e6-40de-bdd8-acc4f517fb48" />
