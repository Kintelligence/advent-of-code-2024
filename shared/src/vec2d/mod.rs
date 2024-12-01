use core::fmt;

pub struct Vec2d<T> {
    pub vec: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Vec2d<T> {
    pub fn from_vec(vec: Vec<T>, height: usize) -> Self {
        Self {
            width: vec.len() / height,
            vec,
            height,
        }
    }

    pub fn new(vec: Vec<T>, width: usize, height: usize) -> Self {
        Self { vec, width, height }
    }

    pub fn row(&self, y: usize) -> &[T] {
        &self.vec[self.width * y..(self.width * y + self.width)]
    }

    pub fn index(&self, x: usize, y: usize) -> &T {
        &self.vec[self.width * y + x]
    }

    pub fn index_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.vec[self.width * y + x]
    }

    pub fn up(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if y == 0 {
            return None;
        }

        Some((x, y - 1))
    }

    pub fn down(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if y < self.height - 1 {
            return None;
        }

        Some((x, y + 1))
    }

    pub fn right(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x < self.width - 1 {
            return None;
        }

        Some((x + 1, y))
    }

    pub fn left(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x > 0 {
            return None;
        }

        Some((x - 1, y))
    }

    pub fn adjacent(&self, x: usize, y: usize) -> Adjecent {
        Adjecent {
            x,
            y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn neighbours(&self, x: usize, y: usize) -> Neighbours {
        Neighbours {
            x,
            y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn diagonals(&self, x: usize, y: usize) -> Diagonals {
        Diagonals {
            x,
            y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Vec2d<T> {
    fn index(&self, index: (usize, usize)) -> &T {
        self.index(index.0, index.1)
    }

    type Output = T;
}

impl<T> std::ops::IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        self.index_mut(index.0, index.1)
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.height {
            str.push_str(&format!("{:?}", &self.row(i)));
            if i != self.height - 1 {
                str.push_str(", ");
            }
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}

pub const ADJ_EIGHT: [(isize, isize); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
];

pub struct Adjecent {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for Adjecent {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 7 {
                return None;
            }

            let nx = self.x as isize + ADJ_EIGHT[self.current].0;
            let ny = self.y as isize + ADJ_EIGHT[self.current].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some((nx as usize, ny as usize));
        }
    }
}

pub const ADJ_FOUR: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

pub struct Neighbours {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for Neighbours {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 3 {
                return None;
            }

            let nx = self.x as isize + ADJ_FOUR[self.current].0;
            let ny = self.y as isize + ADJ_FOUR[self.current].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some((nx as usize, ny as usize));
        }
    }
}

pub const DIAGONAL: [(isize, isize); 4] = [(-1, 1), (1, -1), (1, 1), (-1, -1)];

pub struct Diagonals {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for Diagonals {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 3 {
                return None;
            }

            let nx = self.x as isize + DIAGONAL[self.current].0;
            let ny = self.y as isize + DIAGONAL[self.current].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some((nx as usize, ny as usize));
        }
    }
}
