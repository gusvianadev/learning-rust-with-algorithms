use std::rc::Rc;

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

type Maze<'a> = &'a [&'a str];
type Seen = Vec<Vec<bool>>;
type Path = Vec<Rc<Point>>;

fn walk(
    maze: Maze,
    wall: char,
    curr: Rc<Point>,
    end: Rc<Point>,
    seen: &mut Seen,
    path: &mut Path,
) -> bool {
    let x_pos = curr.x as usize;
    let y_pos = curr.y as usize;
    // 1. Base Case

    // off the map
    if curr.x < 0 || x_pos >= maze[0].len() || curr.y < 0 || y_pos >= maze.len() {
        return false;
    }

    // on a wall
    let maze_curr = &maze[y_pos];
    if maze_curr.chars().nth(x_pos).unwrap() == wall {
        return false;
    }

    // the end
    if curr.x == end.x && curr.y == end.y {
        path.push(end);
        return true;
    }

    // seen
    if seen[y_pos][x_pos] {
        return false;
    }

    // 3 recurse
    // pre
    seen[y_pos][x_pos] = true;
    path.push(Rc::clone(&curr));

    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    // recurse
    for i in 0..dir.len() {
        let (x, y) = dir[i];
        let new_curr = Rc::new(Point {
            x: curr.x + x,
            y: curr.y + y,
        });

        if walk(&maze, wall, new_curr, Rc::clone(&end), seen, path) {
            return true;
        }
    }

    // post
    path.pop();

    false
}

pub fn maze_solver(maze: &Maze, wall: char, start: Rc<Point>, end: Rc<Point>) -> Path {
    let mut seen: Seen = vec![];
    let mut path: Path = vec![];

    for i in 0..maze.len() {
        seen.push(vec![false; maze[0].len()]);
    }

    walk(&maze, wall, start, end, &mut seen, &mut path);

    return path;
}

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;
    pub fn draw_path(data: Maze, path: Path) -> Vec<String> {
        let mut data2: Vec<Vec<char>> = data.iter().map(|row| row.chars().collect()).collect();
        for p in path {
            if let Some(row) = data2.get_mut(p.y as usize) {
                if let Some(cell) = row.get_mut(p.x as usize) {
                    *cell = '*';
                }
            }
        }
        data2.iter().map(|row| row.iter().collect()).collect()
    }
    #[test]
    fn test_maze_solver() {
        let maze: Maze = &[
            "xxxxxxxxxx x",
            "x        x x",
            "x        x x",
            "x xxxxxxxx x",
            "x          x",
            "x xxxxxxxxxx",
        ];

        let maze_result: Path = Vec::from([
            Rc::new(Point { x: 10, y: 0 }),
            Rc::new(Point { x: 10, y: 1 }),
            Rc::new(Point { x: 10, y: 2 }),
            Rc::new(Point { x: 10, y: 3 }),
            Rc::new(Point { x: 10, y: 4 }),
            Rc::new(Point { x: 9, y: 4 }),
            Rc::new(Point { x: 8, y: 4 }),
            Rc::new(Point { x: 7, y: 4 }),
            Rc::new(Point { x: 6, y: 4 }),
            Rc::new(Point { x: 5, y: 4 }),
            Rc::new(Point { x: 4, y: 4 }),
            Rc::new(Point { x: 3, y: 4 }),
            Rc::new(Point { x: 2, y: 4 }),
            Rc::new(Point { x: 1, y: 4 }),
            Rc::new(Point { x: 1, y: 5 }),
        ]);

        let start = Rc::new(Point { x: 10, y: 0 });
        let end = Rc::new(Point { x: 1, y: 5 });

        let result = maze_solver(&maze, 'x', start, end);
        dbg!(&result);

        assert_eq!(draw_path(maze, result), draw_path(maze, maze_result))
    }
}
