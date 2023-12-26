use std::fs;

struct Hailstone {
    position: (f64, f64, f64),
    trajectory: (f64, f64, f64)
}

fn build_hailstones(hailstones: String) -> Vec<Hailstone> {
    hailstones
        .lines()
        .filter_map(|hailstone| {
            if let Some((positions, trajectories)) = hailstone.split_once(" @ ") {
                let positions = positions
                    .replace(" ", "")
                    .split(",")
                    .filter_map(|p| p.parse::<f64>().ok())
                    .collect::<Vec<f64>>();

                let trajectories = trajectories
                    .replace(" ", "")
                    .split(",")
                    .filter_map(|t| t.parse::<f64>().ok())
                    .collect::<Vec<f64>>();

                if positions.len() == 3 && trajectories.len() == 3 {
                    return Some(Hailstone {
                        position: (positions[0], positions[1], positions[2]),
                        trajectory: (trajectories[0], trajectories[1], trajectories[2])
                    });
                }
            }

            return None;
        }).collect()
}

fn hailstone_intersects(
    hailstones: &Vec<Hailstone>,
    target: &Hailstone,
    idx: usize,
) -> usize {
    const LOWER_BOUND: f64 = 200_000_000_000_000.0;
    const UPPER_BOUND: f64 = 400_000_000_000_000.0;

    // const LOWER_BOUND: f64 = 7.0;
    // const UPPER_BOUND: f64 = 27.0;

    // Trajectory and y-intercept for the target.
    let target_trajectory = target.trajectory.1 / target.trajectory.0;
    let target_incidence = (target_trajectory * -(target.position.0)) + target.position.1;

    hailstones
        .iter()
        .enumerate()
        .filter(|(other_idx, candidate)| {
            // Only check each pair once.
            idx < *other_idx && {
                // Trajectory and y-intercept for the candidate.
                let other_trajectory = candidate.trajectory.1 / candidate.trajectory.0;
                let other_incidence = (other_trajectory * -(candidate.position.0)) + candidate.position.1;

                // Check the lines aren’t parallel.
                if target_trajectory == other_trajectory { return false; }

                // Equation for x and y intersection.
                let x_intersection = (other_incidence - target_incidence) / (target_trajectory - other_trajectory);
                let y_intersection = (target_trajectory * x_intersection) + target_incidence;

                // Need to check whether the collision is within the test area.
                if x_intersection >= LOWER_BOUND && x_intersection <= UPPER_BOUND &&
                    y_intersection >= LOWER_BOUND && y_intersection <= UPPER_BOUND
                {
                    // Need to figure out if collison happens in the future or the past.
                    // Quite convenient that there is only one possible way to do this.
                    if ((target.trajectory.0 > 0.0 && x_intersection > target.position.0) ||
                        (target.trajectory.0 < 0.0 && x_intersection < target.position.0)) &&
                       ((candidate.trajectory.0 > 0.0 && x_intersection > candidate.position.0) ||
                        (candidate.trajectory.0 < 0.0 && x_intersection < candidate.position.0))
                    {
                        return true;
                    }
                }

                return false;
            }
        }).count()
}

fn colliding_trajectories() -> usize {
    if let Some(hailstones) = fs::read_to_string("data/24.input").ok() {
        let hailstones = build_hailstones(hailstones);

        return hailstones
            .iter()
            .enumerate()
            .map(|(idx, hailstone)| hailstone_intersects(&hailstones, hailstone, idx))
            .sum();
    } else {
        panic!("file not found")
    }
}

fn main() {
    println!("part one: {}", colliding_trajectories());
}
