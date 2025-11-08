use std::ops::{Add, Div, Mul};

ec::solution!(2);

#[derive(Debug, Clone, Copy, Default)]
struct Complex {
    real: i64,
    imag: i64
}

impl Complex {
    fn new(real: i64, imag:i64) -> Self {
        Complex { real, imag }
    }
}

impl From<&str> for Complex {
    fn from(value: &str) -> Self {
        // expects [X,Y] format
        let (real,imag) = value.split_once(',').unwrap();
        let real: i64 = real.strip_prefix("[").and_then(|v| v.parse().ok()).unwrap();
        let imag: i64 = imag.strip_suffix("]").and_then(|v| v.parse().ok()).unwrap();
        Complex { real, imag }
    }   
}

impl From<Complex> for String {
    fn from(value: Complex) -> Self {
        format!("[{},{}]", value.real, value.imag)
    }
}

// [X1,Y1] + [X2,Y2] = [X1 + X2, Y1 + Y2]
impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex { 
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag
         }
    }
}

// [X1,Y1] * [X2,Y2] = [X1 * X2 - Y1 * Y2, X1 * Y2 + Y1 * X2]
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

// [X1,Y1] / [X2,Y2] = [X1 / X2, Y1 / Y2]
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real / rhs.real,
            imag: self.imag / rhs.imag,
        }
    }
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut r = Complex::from("[0,0]");
    let a = Complex::from(notes.split_once('=').unwrap().1);
    for _ in 0..3 {
        r = r * r;
        r = r / Complex::new(10,10);
        r = r + a;
    }
    Some(r.into())

}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let start = Complex::from(notes.split_once('=').unwrap().1);
    let mut count = 0;
    for x in 0..101 {
        for y in 0..101 {
            let p = Complex::new(start.real + (x * 10), start.imag + (y * 10));
            let mut r = Complex::new(0,0);
            let mut engrave = true;
            for _ in 0..100 {
                r = r * r;
                r = r / Complex::new(100000,100000);
                r = r + p;
                if r.real > 1000000 || r.real < -1000000 || r.imag > 1000000 || r.imag < -1000000 {
                    engrave = false;
                    break;
                }
            }
            if engrave { count += 1; }
        }
    }
    Some(format!("{count}"))
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let start = Complex::from(notes.split_once('=').unwrap().1);
    let mut count = 0;
    for x in 0..1001 {
        for y in 0..1001 {
            let p = Complex::new(start.real + x, start.imag + y);
            let mut r = Complex::new(0,0);
            let mut engrave = true;
            for _ in 0..100 {
                r = r * r;
                r = r / Complex::new(100000,100000);
                r = r + p;
                if r.real > 1000000 || r.real < -1000000 || r.imag > 1000000 || r.imag < -1000000 {
                    engrave = false;
                    break;
                }
            }
            if engrave { count += 1; }
        }
    }
    Some(format!("{count}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(2, 1));
        assert_eq!(result, Some("[357,862]".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(2, 2));
        assert_eq!(result, Some("4076".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(2, 3));
        assert_eq!(result, Some("406954".to_string()));
    }
}
