extern crate num;
use num::Complex;

struct Grid {
    matrix: Vec<Vec<u8>>,
    pos: Complex<isize>,
    dir: Complex<isize>,
}

impl Grid {
    fn new(tiles: &[u8]) -> Self {
        let matrix: Vec<_> = tiles.split(|&x| x == b'\n')
            .map(|row| row.to_vec())
            .collect();
        let start = matrix[0].iter().position(|&x| x == b'|').unwrap() as isize;
        let pos: Complex<isize> = Complex::new(start, 0);
        let dir: Complex<isize> = Complex::new(0, 1);
        Grid { matrix, pos, dir }
    }

    fn peek(&mut self, x: isize, y: isize) -> Option<u8> {
        let direction = self.dir * Complex::new(x, y);
        let pos = self.pos + direction;
        let token = self.token(&pos);
        if token == b' ' { None } else { Some(token) }
    }

    fn token(&self, pos: &Complex<isize>) -> u8 {
        self.matrix.get(pos.im as usize)
            .and_then(|row| row.get(pos.re as usize))
            .cloned()
            .unwrap_or(b' ')
    }

    fn walk(&mut self) -> Vec<u8> {
        let mut visited: Vec<u8> = Vec::new();
        // Walk forward
        while let Some(token) = self.peek(1, 0) {
            // Step onto new tile
            self.pos = self.pos + self.dir;

            if token == b'+' {
                // We need to rotate so let's look either side
                if self.peek(0, 1).is_some() {
                    self.dir *= Complex::new(0, 1);
                    continue;
                }
                if self.peek(0, -1).is_some() {
                    self.dir *= Complex::new(0, -1);
                    continue;
                }
            }

            if (token as char).is_alphabetic() {
                visited.push(token);
            }
        }
        visited
    }
}

pub fn path(input: &[u8]) -> Vec<u8> {
    let mut grid = Grid::new(input);
    grid.walk()
}

#[cfg(test)]
mod tests {
    use super::path;

    #[test]
    fn it_works() {
        let input = b"     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
                ";

        assert_eq!(path(input), b"ABCDEF");
    }
}
