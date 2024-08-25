#[derive(Debug, Clone)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Body {
    mass: f64,
    position: Position,
    velocity: (f64, f64),
}

impl Body {
    fn new(position: Position) -> Self {
        Body {
            mass: 1.0,
            position,
            velocity: (0.0, 0.0),
        }
    }
}

struct Step {
    time: f64,
    step: u32,
    bodies: [Body; 3],
}
const TIME_STEP: f64 = 0.01;
const STEPS: usize = 100000;
const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11; // G
fn main() {
    let mut first = Body::new(Position {
        x: 0.3089693008,
        y: 0.4236727692,
    });
    let mut second = Body::new(Position { x: -0.5, y: 0.0 });
    let mut third = Body::new(Position { x: 0.5, y: 0.0 });

    let mut steps = Vec::<Step>::with_capacity(STEPS);

    for n in 0..STEPS {
        let mut new_step = Step {
            time: n as f64 * TIME_STEP,
            step: n as u32,
            bodies: [first.clone(), second.clone(), third.clone()],
        };
        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                    let dx = new_step.bodies[j].position.x - new_step.bodies[i].position.x;
                    let dy = new_step.bodies[j].position.y - new_step.bodies[i].position.y;
                    // 两点间距离
                    let r = (dx * dx + dy * dy).sqrt();
                    // 引力
                    let force =
                        GRAVITATIONAL_CONSTANT * new_step.bodies[j].mass * new_step.bodies[i].mass
                            / r
                            / r;
                    let angle = dy.atan2(dx);
                    let fx = force * angle.cos();
                    let fy = force * angle.sin();
                    // 更新速度
                    // v = v0 + a * t;
                    // new_step.bodies[j].velocity.0 += fx / new_step.bodies[j].mass * TIME_STEP;
                    // new_step.bodies[j].velocity.1 += fy / new_step.bodies[j].mass * TIME_STEP;
                    new_step.bodies[i].velocity.0 += fx / new_step.bodies[i].mass * TIME_STEP;
                    new_step.bodies[i].velocity.1 += fy / new_step.bodies[i].mass * TIME_STEP;
                }
            }
        }
        // 更新位移
        for k in 0..3 {
            new_step.bodies[k].position.x += new_step.bodies[k].velocity.0 * TIME_STEP;
            new_step.bodies[k].position.y += new_step.bodies[k].velocity.1 * TIME_STEP;
        }
        first = new_step.bodies[0].clone();
        second = new_step.bodies[1].clone();
        third = new_step.bodies[2].clone();

        if n % 1000 == 0 {
            println!(
                "Finished step {} ({:.08}, {:.08})",
                n, first.position.x, first.position.y
            );
        }
    }
}
