struct Planet {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

fn gravity_delta(a: i32, b: i32) -> i32 {
    if a > b {
        return -1;
    } else if a < b {
        return 1;
    }

    return 0;
}

impl Planet {
    fn new(a: i32, b: i32, c: i32) -> Self {
        return Planet {
            x: a,
            y: b,
            z: c,
            vx: 0,
            vy: 0,
            vz: 0,
        };
    }

    fn calculate_gravity(&self, p1: &Planet, p2: &Planet, p3: &Planet) -> (i32, i32, i32) {
        let mut dx = gravity_delta(self.x, p1.x);
        dx += gravity_delta(self.x, p2.x);
        dx += gravity_delta(self.x, p3.x);

        let mut dy = gravity_delta(self.y, p1.y);
        dy += gravity_delta(self.y, p2.y);
        dy += gravity_delta(self.y, p3.y);

        let mut dz = gravity_delta(self.z, p1.z);
        dz += gravity_delta(self.z, p3.z);
        dz += gravity_delta(self.z, p2.z);

        return (dx, dy, dz);
    }

    fn apply_velocity(&mut self, dx: i32, dy: i32, dz: i32) {
        self.vx += dx;
        self.vy += dy;
        self.vz += dz;
    }

    fn update_position(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn energy(&self) -> i32 {
        return (self.x.abs() + self.y.abs() + self.z.abs())
            * (self.vx.abs() + self.vy.abs() + self.vz.abs());
    }

    fn print(&self) {
        println!(
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            self.x, self.y, self.z, self.vx, self.vy, self.vz,
        );
    }
}

fn update_gravity(p1: &mut Planet, p2: &mut Planet, p3: &mut Planet, p4: &mut Planet) {
    let (dx1, dy1, dz1) = p1.calculate_gravity(p2, p3, p4);
    let (dx2, dy2, dz2) = p2.calculate_gravity(p1, p3, p4);
    let (dx3, dy3, dz3) = p3.calculate_gravity(p1, p2, p4);
    let (dx4, dy4, dz4) = p4.calculate_gravity(p1, p2, p3);

    p1.apply_velocity(dx1, dy1, dz1);
    p2.apply_velocity(dx2, dy2, dz2);
    p3.apply_velocity(dx3, dy3, dz3);
    p4.apply_velocity(dx4, dy4, dz4);
}

fn update_velocity(p1: &mut Planet, p2: &mut Planet, p3: &mut Planet, p4: &mut Planet) {
    p1.update_position();
    p2.update_position();
    p3.update_position();
    p4.update_position();
}

fn simulate(
    max_steps: i32,
    mut p1: &mut Planet,
    mut p2: &mut Planet,
    mut p3: &mut Planet,
    mut p4: &mut Planet,
) {
    for step in 0..max_steps {
        println!("After {} steps", step);
        p1.print();
        p2.print();
        p3.print();
        p4.print();
        println!();

        update_gravity(&mut p1, &mut p2, &mut p3, &mut p4);
        update_velocity(&mut p1, &mut p2, &mut p3, &mut p4);
    }
}

fn calculate_energy(p1: &Planet, p2: &Planet, p3: &Planet, p4: &Planet) -> i32 {
    return p1.energy() + p2.energy() + p3.energy() + p4.energy();
}

fn main() {
    let mut p1 = Planet::new(1, 3, -11);
    let mut p2 = Planet::new(17, -10, -8);
    let mut p3 = Planet::new(-1, -15, 2);
    let mut p4 = Planet::new(12, -4, -4);

    simulate(1000, &mut p1, &mut p2, &mut p3, &mut p4);
    let energy = calculate_energy(&p1, &p2, &p3, &p4);
    println!("Total energy: {}", energy);
}

#[test]
fn test() {
    let mut p1 = Planet::new(-1, 0, 2);
    let mut p2 = Planet::new(2, -10, -7);
    let mut p3 = Planet::new(4, -8, 8);
    let mut p4 = Planet::new(3, 5, -1);

    simulate(10, &mut p1, &mut p2, &mut p3, &mut p4);
    assert_eq!(179, calculate_energy(&p1, &p2, &p3, &p4));
}
