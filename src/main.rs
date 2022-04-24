use std::clone::Clone;

use graphics::math::Vec2d;

use piston_window::*;

struct World {
    width: u32,
    height: u32,
    alive: Vec<Vec2d<u32>>,
    matrix: Vec<bool>,
}

impl Clone for World {
    fn clone(&self) -> World {
        let mut new_world = World::new(self.width, self.height);
        for a in &mut self.alive.iter() {
            new_world.alive.push(*a);
            let idx = new_world.convert_index(a[0] as i32, a[1] as i32);
            new_world.matrix[idx] = true;
        }
        new_world
    }
}

impl World {
    fn new(width: u32, height: u32) -> World {
        let mut world = World {
            width: width,
            height: height,
            alive: Vec::<Vec2d<u32>>::new(),
            matrix: Vec::with_capacity(width as usize * height as usize),
        };
        world.matrix.resize(width as usize * height as usize, false);
        world
    }

    fn update(&mut self) -> World {
        let mut new_world = World::new(self.width, self.height);
        for i in 0..self.width as i32{
            for j in 0..self.height as i32{
                let alive_neighbors = self.count_neighbors(i, j);
                match alive_neighbors {
                    0 | 1 => new_world.set_index(i, j, false),
                    2 => new_world.set_index(i, j, self.get_index(i, j)),
                    3 => new_world.set_index(i, j, true),
                    4_u32..=u32::MAX => new_world.set_index(i, j, false),
                }
            }
        }
        new_world
    }

    fn count_neighbors(&self, x: i32, y: i32) -> u32 {
        let mut neighbors = 0u32;
        for i in -1i32..=1 {
            let posx = x + i;
            if posx < 0 || posx >= self.width as i32 {
                continue
            }
            for j in -1i32..=1 {
                let posy = y + j;
                if posy < 0 || posy >= self.height as i32 {
                    continue
                }
                if self.get_index(posx, posy) && !(posx == x && posy == y) {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn set_index(&mut self, i: i32, j: i32, val: bool) {
        let idx = self.convert_index(i, j);
        self.matrix[idx] = val;
        if val {
            self.alive.push([i as u32, j as u32].into());
        }
    }

    fn get_index(&self, i: i32, j: i32) -> bool {
        self.matrix[self.convert_index(i, j)]
    }

    fn convert_index(&self, i: i32, j: i32) -> usize {
        (i * self.width as i32 + j) as usize
    }
}

fn print_alive(w: &World) {
    for a in &mut w.alive.iter() {
        println!("a[0]={}, a[1]={}, live_n={}", a[0], a[1], w.count_neighbors(a[0] as i32, a[1] as i32));
    }
}

fn main() {
    let (width, height) = (800, 800);
    let mut window: PistonWindow = WindowSettings::new("stones", [width, height])
    .exit_on_esc(true)
    .build()
    .expect("Could not create a window.");

    let mut old_world = World::new(width, height);
    let mut new_world: World;

    old_world.set_index(49, 50, true);
    old_world.set_index(50, 51, true);
    old_world.set_index(51, 50, true);
    old_world.set_index(51, 51, true);
    old_world.set_index(51, 49, true);

    print_alive(&old_world);

    let mut i: u32 = 0;
    while let Some(event) = window.next() {
        println!("Starting turn {}", i);
        println!("Old world");
        print_alive(&old_world);
        new_world = old_world.update();
        println!("New world");
        print_alive(&new_world);
        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut new_world.alive.iter() {
                let size = [s[0] as f64 * 4.0, s[1] as f64 * 4.0, 4.0, 4.0];
                rectangle([1.0, 1.0, 1.0, 0.99], size, ctx.transform, renderer);
            }
        });
        old_world = new_world.clone();
        i += 1;
    }
}
