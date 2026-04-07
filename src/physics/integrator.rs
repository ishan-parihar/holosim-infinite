//! RK4 (Runge-Kutta 4th Order) Integration for HoloSim Physics — Phase 6.5
//!
//! Provides high-accuracy numerical integration of archetype forces as a
//! standalone utility. Does **not** modify Rapier's internal integration.
//!
//! RK4 achieves O(dt^4) local truncation error vs O(dt^2) for semi-implicit
//! Euler, making it suitable for long-duration simulations where energy
//! conservation matters (orbital mechanics, harmonic oscillators).
//!
//! # Algorithm
//!
//! ```text
//! k1 = f(t, y)
//! k2 = f(t + dt/2, y + k1*dt/2)
//! k3 = f(t + dt/2, y + k2*dt/2)
//! k4 = f(t + dt, y + k3*dt)
//! y_new = y + (k1 + 2*k2 + 2*k3 + k4) * dt / 6
//! ```
//!
//! Where y = [position, velocity] and f(y) = [velocity, force(state)/mass].

/// State vector for RK4 integration: position and velocity in 3D.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RK4State {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
}

/// Derivative of the state vector: dp/dt (velocity) and dv/dt (acceleration).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RK4Derivative {
    /// Derivative of position = velocity.
    pub dp_dt: [f64; 3],
    /// Derivative of velocity = acceleration = force / mass.
    pub dv_dt: [f64; 3],
}

/// RK4 (4th-order Runge-Kutta) integrator.
///
/// A standalone utility for accurate numerical integration. Does not modify
/// Rapier's internal semi-implicit Euler integration.
pub struct RK4Integrator;

impl RK4Integrator {
    /// Evaluate the derivative at a given state.
    ///
    /// - `state`: original state (provides baseline velocity for dp_dt)
    /// - `mass`: entity mass (for a = F/m)
    /// - `force`: force vector at the evaluation point
    /// - `dt`: timestep (unused in derivative, kept for API consistency)
    /// - `euler_state`: state at the evaluation point (provides velocity for dp_dt)
    #[inline]
    pub fn evaluate(
        _state: &RK4State,
        mass: f64,
        force: [f64; 3],
        _dt: f64,
        euler_state: &RK4State,
    ) -> RK4Derivative {
        let inv_mass = if mass.abs() > 1e-30 { 1.0 / mass } else { 0.0 };

        RK4Derivative {
            dp_dt: euler_state.velocity,
            dv_dt: [
                force[0] * inv_mass,
                force[1] * inv_mass,
                force[2] * inv_mass,
            ],
        }
    }

    /// Add two derivatives element-wise.
    #[inline]
    fn add_derivative(a: &RK4Derivative, b: &RK4Derivative) -> RK4Derivative {
        RK4Derivative {
            dp_dt: [
                a.dp_dt[0] + b.dp_dt[0],
                a.dp_dt[1] + b.dp_dt[1],
                a.dp_dt[2] + b.dp_dt[2],
            ],
            dv_dt: [
                a.dv_dt[0] + b.dv_dt[0],
                a.dv_dt[1] + b.dv_dt[1],
                a.dv_dt[2] + b.dv_dt[2],
            ],
        }
    }

    /// Scale a derivative by a scalar.
    #[inline]
    fn scale_derivative(d: &RK4Derivative, s: f64) -> RK4Derivative {
        RK4Derivative {
            dp_dt: [d.dp_dt[0] * s, d.dp_dt[1] * s, d.dp_dt[2] * s],
            dv_dt: [d.dv_dt[0] * s, d.dv_dt[1] * s, d.dv_dt[2] * s],
        }
    }

    /// Add a scaled derivative to a state to produce a new state.
    #[inline]
    fn state_add_scaled(state: &RK4State, deriv: &RK4Derivative, s: f64) -> RK4State {
        RK4State {
            position: [
                state.position[0] + deriv.dp_dt[0] * s,
                state.position[1] + deriv.dp_dt[1] * s,
                state.position[2] + deriv.dp_dt[2] * s,
            ],
            velocity: [
                state.velocity[0] + deriv.dv_dt[0] * s,
                state.velocity[1] + deriv.dv_dt[1] * s,
                state.velocity[2] + deriv.dv_dt[2] * s,
            ],
        }
    }

    /// Integrate one timestep using RK4 method.
    ///
    /// - `state`: current position and velocity
    /// - `mass`: entity mass
    /// - `force_fn`: function that computes force given a state (may depend on position)
    /// - `dt`: timestep
    ///
    /// Returns the new state after one RK4 step.
    #[inline]
    pub fn integrate<F>(state: &RK4State, mass: f64, force_fn: F, dt: f64) -> RK4State
    where
        F: Fn(&RK4State) -> [f64; 3],
    {
        let half_dt = dt * 0.5;

        // k1 = f(t, y)
        let force1 = force_fn(state);
        let k1 = Self::evaluate(state, mass, force1, dt, state);

        // k2 = f(t + dt/2, y + k1*dt/2)
        let state2 = Self::state_add_scaled(state, &k1, half_dt);
        let force2 = force_fn(&state2);
        let k2 = Self::evaluate(state, mass, force2, dt, &state2);

        // k3 = f(t + dt/2, y + k2*dt/2)
        let state3 = Self::state_add_scaled(state, &k2, half_dt);
        let force3 = force_fn(&state3);
        let k3 = Self::evaluate(state, mass, force3, dt, &state3);

        // k4 = f(t + dt, y + k3*dt)
        let state4 = Self::state_add_scaled(state, &k3, dt);
        let force4 = force_fn(&state4);
        let k4 = Self::evaluate(state, mass, force4, dt, &state4);

        // y_new = y + (k1 + 2*k2 + 2*k3 + k4) * dt / 6
        let two_k2 = Self::scale_derivative(&k2, 2.0);
        let two_k3 = Self::scale_derivative(&k3, 2.0);
        let sum = Self::add_derivative(&k1, &two_k2);
        let sum = Self::add_derivative(&sum, &two_k3);
        let sum = Self::add_derivative(&sum, &k4);
        let weighted = Self::scale_derivative(&sum, dt / 6.0);

        RK4State {
            position: [
                state.position[0] + weighted.dp_dt[0],
                state.position[1] + weighted.dp_dt[1],
                state.position[2] + weighted.dp_dt[2],
            ],
            velocity: [
                state.velocity[0] + weighted.dv_dt[0],
                state.velocity[1] + weighted.dv_dt[1],
                state.velocity[2] + weighted.dv_dt[2],
            ],
        }
    }

    /// Perform a single semi-implicit Euler step for comparison.
    ///
    /// v_new = v + a*dt; p_new = p + v_new*dt
    #[inline]
    fn euler_step<F>(state: &RK4State, mass: f64, force_fn: F, dt: f64) -> RK4State
    where
        F: Fn(&RK4State) -> [f64; 3],
    {
        let force = force_fn(state);
        let inv_mass = if mass.abs() > 1e-30 { 1.0 / mass } else { 0.0 };
        let accel = [
            force[0] * inv_mass,
            force[1] * inv_mass,
            force[2] * inv_mass,
        ];

        // Semi-implicit Euler: update velocity first, then position
        RK4State {
            position: [
                state.position[0] + (state.velocity[0] + accel[0] * dt) * dt,
                state.position[1] + (state.velocity[1] + accel[1] * dt) * dt,
                state.position[2] + (state.velocity[2] + accel[2] * dt) * dt,
            ],
            velocity: [
                state.velocity[0] + accel[0] * dt,
                state.velocity[1] + accel[1] * dt,
                state.velocity[2] + accel[2] * dt,
            ],
        }
    }

    /// Compare RK4 vs Euler integration for a given state.
    ///
    /// Returns `(rk4_state, euler_state, position_error, velocity_error)`
    /// where errors are Euclidean norms of the difference vectors.
    pub fn compare_with_euler<F>(
        state: &RK4State,
        mass: f64,
        force_fn: F,
        dt: f64,
    ) -> (RK4State, RK4State, f64, f64)
    where
        F: Fn(&RK4State) -> [f64; 3],
    {
        let rk4 = Self::integrate(state, mass, &force_fn, dt);
        let euler = Self::euler_step(state, mass, &force_fn, dt);

        let pos_err = ((rk4.position[0] - euler.position[0]).powi(2)
            + (rk4.position[1] - euler.position[1]).powi(2)
            + (rk4.position[2] - euler.position[2]).powi(2))
        .sqrt();

        let vel_err = ((rk4.velocity[0] - euler.velocity[0]).powi(2)
            + (rk4.velocity[1] - euler.velocity[1]).powi(2)
            + (rk4.velocity[2] - euler.velocity[2]).powi(2))
        .sqrt();

        (rk4, euler, pos_err, vel_err)
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: Euclidean norm of a 3D vector.
    fn norm3(v: [f64; 3]) -> f64 {
        (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
    }

    // ------------------------------------------------------------------------
    // test_constant_force: RK4 matches analytical solution exactly for constant F
    // ------------------------------------------------------------------------
    #[test]
    fn test_constant_force() {
        // Constant force: F = (2, 3, 5), mass = 1 → a = (2, 3, 5)
        // Analytical: v(t) = v0 + a*t, p(t) = p0 + v0*t + 0.5*a*t^2
        let state = RK4State {
            position: [0.0, 0.0, 0.0],
            velocity: [1.0, 2.0, 3.0],
        };
        let mass = 1.0;
        let dt = 0.5;
        let force = [2.0, 3.0, 5.0];

        let new_state = RK4Integrator::integrate(&state, mass, |_| force, dt);

        // Analytical:
        // v_new = (1,2,3) + (2,3,5)*0.5 = (2, 3.5, 5.5)
        // p_new = (0,0,0) + (1,2,3)*0.5 + 0.5*(2,3,5)*0.25
        //       = (0.5, 1, 1.5) + (0.25, 0.375, 0.625) = (0.75, 1.375, 2.125)
        let expected_vel = [2.0, 3.5, 5.5];
        let expected_pos = [0.75, 1.375, 2.125];

        for i in 0..3 {
            assert!(
                (new_state.velocity[i] - expected_vel[i]).abs() < 1e-14,
                "velocity[{}] = {} vs expected {}",
                i,
                new_state.velocity[i],
                expected_vel[i]
            );
            assert!(
                (new_state.position[i] - expected_pos[i]).abs() < 1e-14,
                "position[{}] = {} vs expected {}",
                i,
                new_state.position[i],
                expected_pos[i]
            );
        }
    }

    // ------------------------------------------------------------------------
    // test_harmonic_oscillator: RK4 matches sin/cos within 1e-10
    // ------------------------------------------------------------------------
    #[test]
    fn test_harmonic_oscillator() {
        // Simple harmonic oscillator: F = -k*x, m = 1
        // Analytical: x(t) = A*cos(omega*t), v(t) = -A*omega*sin(omega*t)
        // with omega = sqrt(k/m) = 1, A = 1
        let k = 1.0;
        let mass = 1.0;
        let dt = 0.01;
        let total_time = 10.0;
        let steps = (total_time / dt) as usize;

        let mut state = RK4State {
            position: [1.0, 0.0, 0.0], // x0 = 1
            velocity: [0.0, 0.0, 0.0], // v0 = 0
        };

        let force_fn = |s: &RK4State| -> [f64; 3] { [-k * s.position[0], 0.0, 0.0] };

        for _ in 0..steps {
            state = RK4Integrator::integrate(&state, mass, &force_fn, dt);
        }

        // Analytical at t=10: x = cos(10), v = -sin(10)
        let t = total_time;
        let expected_x = t.cos();
        let expected_v = -t.sin();

        assert!(
            (state.position[0] - expected_x).abs() < 1e-9,
            "harmonic x = {} vs expected {} (error = {})",
            state.position[0],
            expected_x,
            (state.position[0] - expected_x).abs()
        );
        assert!(
            (state.velocity[0] - expected_v).abs() < 1e-9,
            "harmonic v = {} vs expected {} (error = {})",
            state.velocity[0],
            expected_v,
            (state.velocity[0] - expected_v).abs()
        );
    }

    // ------------------------------------------------------------------------
    // test_gravitational_orbit: RK4 conserves energy better than Euler (1000 steps)
    // ------------------------------------------------------------------------
    #[test]
    fn test_gravitational_orbit() {
        // Circular orbit: F = -G*M*m/r^2 * r_hat
        // Use G*M = 1, m = 1 for simplicity
        // Circular orbit at r=1: v = sqrt(GM/r) = 1
        let gm = 1.0;
        let mass = 1.0;
        let dt = 0.001;
        let steps = 1000;

        // Initial conditions: circular orbit in xy-plane
        let mut state_rk4 = RK4State {
            position: [1.0, 0.0, 0.0],
            velocity: [0.0, 1.0, 0.0], // tangential velocity for circular orbit
        };
        let mut state_euler = state_rk4;

        let grav_force = |s: &RK4State| -> [f64; 3] {
            let r2 = s.position[0] * s.position[0]
                + s.position[1] * s.position[1]
                + s.position[2] * s.position[2];
            let r = r2.sqrt();
            if r < 1e-10 {
                return [0.0, 0.0, 0.0];
            }
            // F = -GMm/r^2 * r_hat, but force_fn returns force, not acceleration
            // We want a = -GM/r^2 * r_hat, so F = m * a = -GM*m/r^2 * r_hat
            let mag = gm * mass / r2;
            [
                -mag * s.position[0] / r,
                -mag * s.position[1] / r,
                -mag * s.position[2] / r,
            ]
        };

        fn compute_energy(state: &RK4State, mass: f64, gm: f64) -> f64 {
            let r = norm3(state.position);
            let v = norm3(state.velocity);
            let ke = 0.5 * mass * v * v;
            let pe = -gm * mass / r;
            ke + pe
        }

        let energy_rk4_initial = compute_energy(&state_rk4, mass, gm);
        let energy_euler_initial = compute_energy(&state_euler, mass, gm);

        for _ in 0..steps {
            state_rk4 = RK4Integrator::integrate(&state_rk4, mass, &grav_force, dt);
            state_euler = RK4Integrator::euler_step(&state_euler, mass, &grav_force, dt);
        }

        let rk4_energy_final = compute_energy(&state_rk4, mass, gm);
        let euler_energy_final = compute_energy(&state_euler, mass, gm);

        let rk4_energy_error = (rk4_energy_final - energy_rk4_initial).abs();
        let euler_energy_error = (euler_energy_final - energy_euler_initial).abs();

        assert!(
            rk4_energy_error < euler_energy_error,
            "RK4 energy error ({}) should be less than Euler energy error ({})",
            rk4_energy_error,
            euler_energy_error
        );
    }

    // ------------------------------------------------------------------------
    // test_rk4_vs_euler_error: RK4 has O(dt^4) error, Euler has O(dt^2)
    // ------------------------------------------------------------------------
    #[test]
    fn test_rk4_vs_euler_error() {
        // Use a nonlinear force: F = -x^3 (Duffing-like, but simple)
        // Test error scaling by running at two different timesteps
        let mass = 1.0;
        let state = RK4State {
            position: [0.5, 0.0, 0.0],
            velocity: [0.1, 0.0, 0.0],
        };

        let force_fn = |s: &RK4State| -> [f64; 3] {
            [-s.position[0] * s.position[0] * s.position[0], 0.0, 0.0]
        };

        // Compare errors at dt and dt/2 for both methods
        let dt1 = 0.1;
        let dt2 = 0.05;

        // Euler errors
        let (_, _euler1, _euler_pos_err1, _) =
            RK4Integrator::compare_with_euler(&state, mass, &force_fn, dt1);
        // For Euler, we need to compare against a reference (very small dt RK4)
        let reference = {
            let mut s = state;
            let tiny_dt = 0.0001;
            let steps = (dt1 / tiny_dt) as usize;
            for _ in 0..steps {
                s = RK4Integrator::integrate(&s, mass, &force_fn, tiny_dt);
            }
            s
        };

        // Euler at dt1
        let euler1_state = RK4Integrator::euler_step(&state, mass, &force_fn, dt1);
        let euler_err1 = (euler1_state.position[0] - reference.position[0]).abs();

        // Euler at dt2 (two half-steps)
        let euler_mid = RK4Integrator::euler_step(&state, mass, &force_fn, dt2);
        let euler2_state = RK4Integrator::euler_step(&euler_mid, mass, &force_fn, dt2);
        let euler_err2 = (euler2_state.position[0] - reference.position[0]).abs();

        // RK4 at dt1
        let rk4_1 = RK4Integrator::integrate(&state, mass, &force_fn, dt1);
        let rk4_err1 = (rk4_1.position[0] - reference.position[0]).abs();

        // RK4 at dt2 (two half-steps)
        let rk4_mid = RK4Integrator::integrate(&state, mass, &force_fn, dt2);
        let rk4_2 = RK4Integrator::integrate(&rk4_mid, mass, &force_fn, dt2);
        let rk4_err2 = (rk4_2.position[0] - reference.position[0]).abs();

        // Euler: halving dt should reduce error (O(dt^2) local truncation)
        let euler_ratio = euler_err1 / euler_err2;
        assert!(
            euler_ratio > 1.5,
            "Euler error ratio {} should be > 1.5 (expected ~4 for O(dt^2))",
            euler_ratio
        );

        // RK4: halving dt should reduce error by ~16x (O(dt^4))
        let rk4_ratio = rk4_err1 / rk4_err2;
        assert!(
            rk4_ratio > 8.0,
            "RK4 error ratio {} should be ~16 (O(dt^4)), expected > 8",
            rk4_ratio
        );

        // RK4 error should be much smaller than Euler at same dt
        assert!(
            rk4_err1 < euler_err1 / 10.0,
            "RK4 error ({}) should be much smaller than Euler error ({})",
            rk4_err1,
            euler_err1
        );
    }

    // ------------------------------------------------------------------------
    // test_zero_force: state unchanged with zero force
    // ------------------------------------------------------------------------
    #[test]
    fn test_zero_force() {
        let state = RK4State {
            position: [1.0, 2.0, 3.0],
            velocity: [4.0, 5.0, 6.0],
        };
        let mass = 2.0;
        let dt = 0.1;

        let new_state = RK4Integrator::integrate(&state, mass, |_| [0.0, 0.0, 0.0], dt);

        // With zero force: position changes by velocity*dt, velocity unchanged
        let expected_pos = [
            state.position[0] + state.velocity[0] * dt,
            state.position[1] + state.velocity[1] * dt,
            state.position[2] + state.velocity[2] * dt,
        ];

        for i in 0..3 {
            assert!(
                (new_state.position[i] - expected_pos[i]).abs() < 1e-14,
                "position[{}] = {} vs expected {}",
                i,
                new_state.position[i],
                expected_pos[i]
            );
            assert!(
                (new_state.velocity[i] - state.velocity[i]).abs() < 1e-14,
                "velocity[{}] = {} vs expected {} (should be unchanged)",
                i,
                new_state.velocity[i],
                state.velocity[i]
            );
        }
    }

    // ------------------------------------------------------------------------
    // test_free_fall: matches y = y0 + v0*t + 0.5*g*t^2
    // ------------------------------------------------------------------------
    #[test]
    fn test_free_fall() {
        let mass = 2.0;
        let g = 9.81;
        let dt = 0.1;
        let total_time = 2.0;
        let steps = (total_time / dt) as usize;
        let fg = mass * g;

        let mut state = RK4State {
            position: [0.0, 100.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
        };

        for _ in 0..steps {
            state = RK4Integrator::integrate(&state, mass, |_| [0.0, -fg, 0.0], dt);
        }

        // Analytical: y = y0 + v0*t + 0.5*(-g)*t^2 = 100 - 0.5*9.81*4 = 100 - 19.62 = 80.38
        // vy = v0 + (-g)*t = 0 - 9.81*2 = -19.62
        let t = total_time;
        let expected_y = 100.0 + 0.0 * t - 0.5 * g * t * t;
        let expected_vy = 0.0 - g * t;

        assert!(
            (state.position[1] - expected_y).abs() < 1e-12,
            "free fall y = {} vs expected {} (error = {})",
            state.position[1],
            expected_y,
            (state.position[1] - expected_y).abs()
        );
        assert!(
            (state.velocity[1] - expected_vy).abs() < 1e-12,
            "free fall vy = {} vs expected {} (error = {})",
            state.velocity[1],
            expected_vy,
            (state.velocity[1] - expected_vy).abs()
        );

        // x and z should be unchanged
        assert!(state.position[0].abs() < 1e-14);
        assert!(state.position[2].abs() < 1e-14);
        assert!(state.velocity[0].abs() < 1e-14);
        assert!(state.velocity[2].abs() < 1e-14);
    }

    // ------------------------------------------------------------------------
    // test_evaluate_basic: derivative computation sanity check
    // ------------------------------------------------------------------------
    #[test]
    fn test_evaluate_basic() {
        let state = RK4State {
            position: [0.0, 0.0, 0.0],
            velocity: [1.0, 2.0, 3.0],
        };
        let mass = 2.0;
        let force = [4.0, 6.0, 8.0];
        let dt = 0.1;

        let deriv = RK4Integrator::evaluate(&state, mass, force, dt, &state);

        // dp/dt = velocity = (1, 2, 3)
        assert!((deriv.dp_dt[0] - 1.0).abs() < 1e-14);
        assert!((deriv.dp_dt[1] - 2.0).abs() < 1e-14);
        assert!((deriv.dp_dt[2] - 3.0).abs() < 1e-14);

        // dv/dt = F/m = (4/2, 6/2, 8/2) = (2, 3, 4)
        assert!((deriv.dv_dt[0] - 2.0).abs() < 1e-14);
        assert!((deriv.dv_dt[1] - 3.0).abs() < 1e-14);
        assert!((deriv.dv_dt[2] - 4.0).abs() < 1e-14);
    }

    // ------------------------------------------------------------------------
    // test_compare_with_euler_returns_errors: verify compare_with_euler produces non-trivial results
    // ------------------------------------------------------------------------
    #[test]
    fn test_compare_with_euler_nonzero_force() {
        let state = RK4State {
            position: [1.0, 0.0, 0.0],
            velocity: [0.0, 1.0, 0.0],
        };
        let mass = 1.0;
        let dt = 0.01;

        let force_fn = |s: &RK4State| -> [f64; 3] { [-s.position[0], 0.0, 0.0] };

        let (rk4, euler, pos_err, vel_err) =
            RK4Integrator::compare_with_euler(&state, mass, &force_fn, dt);

        // They should differ (non-trivial force)
        assert!(
            pos_err > 0.0,
            "position error should be non-zero for non-trivial force"
        );
        assert!(
            vel_err > 0.0,
            "velocity error should be non-zero for non-trivial force"
        );

        // RK4 and Euler states should be different
        assert_ne!(rk4, euler);
    }

    // ------------------------------------------------------------------------
    // test_multi_step_constant_force: multiple RK4 steps match analytical
    // ------------------------------------------------------------------------
    #[test]
    fn test_multi_step_constant_force() {
        let mass = 3.0;
        let dt = 0.05;
        let steps = 20; // total time = 1.0
        let force = [6.0, 9.0, 12.0]; // a = (2, 3, 4)

        let mut state = RK4State {
            position: [1.0, 2.0, 3.0],
            velocity: [0.5, 1.0, 1.5],
        };

        for _ in 0..steps {
            state = RK4Integrator::integrate(&state, mass, |_| force, dt);
        }

        let t = dt * steps as f64;
        let accel = [force[0] / mass, force[1] / mass, force[2] / mass];

        for i in 0..3 {
            let expected_v = match i {
                0 => 0.5 + accel[0] * t,
                1 => 1.0 + accel[1] * t,
                2 => 1.5 + accel[2] * t,
                _ => unreachable!(),
            };
            // p = p0 + v0*t + 0.5*a*t^2
            let expected_p = match i {
                0 => 1.0 + 0.5 * t + 0.5 * accel[0] * t * t,
                1 => 2.0 + 1.0 * t + 0.5 * accel[1] * t * t,
                2 => 3.0 + 1.5 * t + 0.5 * accel[2] * t * t,
                _ => unreachable!(),
            };

            assert!(
                (state.velocity[i] - expected_v).abs() < 1e-12,
                "multi-step velocity[{}] = {} vs {}",
                i,
                state.velocity[i],
                expected_v
            );
            assert!(
                (state.position[i] - expected_p).abs() < 1e-10,
                "multi-step position[{}] = {} vs {} (error = {})",
                i,
                state.position[i],
                expected_p,
                (state.position[i] - expected_p).abs()
            );
        }
    }

    // ------------------------------------------------------------------------
    // test_2d_orbit_stability: RK4 maintains bounded orbit over many steps
    // ------------------------------------------------------------------------
    #[test]
    fn test_2d_orbit_stability() {
        let gm = 4.0 * std::f64::consts::PI * std::f64::consts::PI; // GM = 4*pi^2 for 1-year period at r=1
        let mass = 1.0;
        let dt = 0.001;
        let steps = 10000; // ~10 orbits

        let mut state = RK4State {
            position: [1.0, 0.0, 0.0],
            velocity: [0.0, gm.sqrt(), 0.0], // v = sqrt(GM/r) = 2*pi
        };

        let grav_force = |s: &RK4State| -> [f64; 3] {
            let r2 = s.position[0] * s.position[0] + s.position[1] * s.position[1];
            let r = r2.sqrt();
            if r < 1e-10 {
                return [0.0, 0.0, 0.0];
            }
            let mag = gm * mass / r2;
            [-mag * s.position[0] / r, -mag * s.position[1] / r, 0.0]
        };

        for _ in 0..steps {
            state = RK4Integrator::integrate(&state, mass, &grav_force, dt);
        }

        // After many orbits, radius should still be approximately 1
        let r = norm3(state.position);
        assert!(
            (r - 1.0).abs() < 0.01,
            "orbit radius drift = {}, should be < 0.01 after 10 orbits",
            (r - 1.0).abs()
        );
    }
}
