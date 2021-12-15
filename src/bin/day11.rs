use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::io;
use advent_code_lib::{advent_main, Position, RowMajorPositionIterator, search, nums2map, map_width_height, ContinueSearch};
use bare_metal_modulo::{MNum, ModNumC};

fn main() -> io::Result<()> {
    advent_main(&[], &["-show:num_steps"], |args| {
        let octopi = DumboOctopi::new(args[1].as_str())?;
        match args.iter().find(|arg| arg.starts_with("-show")) {
            None => {
                println!("Part 1 score: {}", part_1(octopi.clone()));
                println!("Part 2 score: {}", part_2(octopi));
            }
            Some(show_step) => {
                let steps: usize = show_step.split(':').nth(1).unwrap().parse().unwrap();
                show_steps(octopi, steps);
            }
        }
        Ok(())
    })
}

const PART_1_ITERATIONS: usize = 100;

fn part_1(octopi: DumboOctopi) -> usize {
    octopi.take(PART_1_ITERATIONS).sum()
}

fn part_2(octopi: DumboOctopi) -> usize {
    let target_flashes = octopi.len();
    1 + octopi.enumerate().find(|(_, flashes)| *flashes == target_flashes)
        .map(|(step, _)| step).unwrap()
}

fn show_steps(mut octopi: DumboOctopi, steps: usize) {
    println!("Before any steps:");
    println!("{}", octopi);
    println!();

    for step in 1..=steps {
        let flashes = octopi.next().unwrap();
        println!("After step {} ({} flashes):", step, flashes);
        println!("{}", octopi);
        println!();
    }
}

#[derive(Clone, Debug)]
struct DumboOctopi {
    energies: HashMap<Position, ModNumC<u32, 10> >,
    width: usize,
    height: usize
}

impl DumboOctopi {
    fn new(filename: &str) -> io::Result<DumboOctopi> {
        let energies = nums2map(filename)?;
        let (width, height) = map_width_height(&energies);
        Ok(DumboOctopi {energies, width, height})
    }

    fn just_flashed(&self) -> impl Iterator<Item=Position> + '_ {
        self.energies.iter()
            .filter(|(_, energy)| **energy == 0)
            .map(|(p, _)| *p)
    }

    fn enqueue_flashed_neighbors(&mut self, flasher: Position, queue: &mut VecDeque<Position>) {
        for neighbor in flasher.neighbors() {
            if let Some(neighbor_energy) = self.energies.get_mut(&neighbor) {
                if *neighbor_energy > 0 {
                    *neighbor_energy += 1;
                    if *neighbor_energy == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.energies.len()
    }
}

impl Iterator for DumboOctopi {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for (_, energy) in self.energies.iter_mut() {
            *energy += 1;
        }

        let result = search(self.just_flashed().collect(),
                            |flasher, q| {
                                self.enqueue_flashed_neighbors(*flasher, q);
                                ContinueSearch::Yes});
        Some(*result.dequeued())
    }
}

impl Display for DumboOctopi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for p in RowMajorPositionIterator::new(self.width, self.height) {
            if p.col == 0 && p.row > 0 {writeln!(f)?;}
            write!(f, "{}", self.energies.get(&p).unwrap().a())?
        }
        Ok(())
    }
}