use aoc_2023::commons::io::Input;
use aoc_2023::commons::math::LcmExt;
use std::error::Error;

#[inline]
fn path_length<F: Fn(u32) -> bool>(
    nodes: &[(u32, u32)],
    path: &str,
    start_node: u32,
    end: F,
) -> usize {
    let mut current_node = start_node;
    for (i, dir) in path.chars().cycle().enumerate() {
        let options = nodes[current_node as usize];
        current_node = match dir {
            'L' => options.0,
            'R' => options.1,
            _ => panic!("AHHH!"),
        };

        if end(current_node) {
            return i + 1;
        }
    }

    usize::MAX
}

#[inline]
fn node_as_int(s: &str) -> u32 {
    let mut bytes = s.bytes();
    ((bytes.next().unwrap() - b'A') as u32) << 10
        | ((bytes.next().unwrap() - b'A') as u32) << 5
        | ((bytes.next().unwrap() - b'A') as u32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;
    let mut lines = input.as_str().lines();

    let path = lines.next().unwrap();
    lines.next();

    #[allow(invalid_value)]
    let mut nodes: [(u32, u32); 32768] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    let mut ending_in_a = Vec::with_capacity(1_000);
    for l in lines {
        let node_name = &l[0..3];
        let node = node_as_int(node_name);
        let left_path = node_as_int(&l[7..10]);
        let right_path = node_as_int(&l[12..15]);

        nodes[node as usize] = (left_path, right_path);
        if node_name.ends_with('A') {
            ending_in_a.push(node);
        }
    }

    let target_node = node_as_int("ZZZ");
    let part1 = path_length(&nodes, path, 0, |node| node == target_node);

    let part2 = ending_in_a
        .iter()
        .map(|node_idx| path_length(&nodes, path, *node_idx, |node| node & 0b11111 == 0b11001))
        .lcm()
        .unwrap();

    println!("{}\n{}", part1, part2);

    Ok(())
}
