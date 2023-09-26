use std::ops::*;

#[derive(Clone, Copy)]
pub struct Modnum<const M: i64> {
    x: i64,
}
impl<const M: i64> Modnum<M> {
    pub fn new(x: i64) -> Modnum<M> {
        Modnum { x: (x % M + M) % M }
    }
}

impl<const M: i64> From<i64> for Modnum<M> {
    fn from(x: i64) -> Self {
        Modnum { x: x % M }
    }
}

impl<const M: i64> Add<Modnum<M>> for Modnum<M> {
    type Output = Self;
    fn add(self, rhs: Modnum<M>) -> Self::Output {
        Modnum {
            x: (self.x + rhs.x) % M,
        }
    }
}
impl<const M: i64> Add<i64> for Modnum<M> {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output {
        Modnum {
            x: (self.x + rhs % M) % M,
        }
    }
}
impl<const M: i64> Sub<Modnum<M>> for Modnum<M> {
    type Output = Self;
    fn sub(self, rhs: Modnum<M>) -> Self::Output {
        Modnum {
            x: (self.x - rhs.x + M) % M,
        }
    }
}
impl<const M: i64> Sub<i64> for Modnum<M> {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self::Output {
        Modnum {
            x: (self.x - rhs % M + M) % M,
        }
    }
}
impl<const M: i64> Mul<Modnum<M>> for Modnum<M> {
    type Output = Self;
    fn mul(self, rhs: Modnum<M>) -> Self::Output {
        Modnum {
            x: (self.x * rhs.x) % M,
        }
    }
}
impl<const M: i64> Mul<i64> for Modnum<M> {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Modnum {
            x: (self.x * (rhs % M)) % M,
        }
    }
}

impl<const M: i64> AddAssign<Modnum<M>> for Modnum<M> {
    fn add_assign(&mut self, rhs: Modnum<M>) {
        self.x = (self.x + rhs.x) % M;
    }
}
impl<const M: i64> AddAssign<i64> for Modnum<M> {
    fn add_assign(&mut self, rhs: i64) {
        self.x = (self.x + rhs % M) % M;
    }
}
impl<const M: i64> SubAssign<Modnum<M>> for Modnum<M> {
    fn sub_assign(&mut self, rhs: Modnum<M>) {
        self.x = (self.x - rhs.x + M) % M;
    }
}
impl<const M: i64> SubAssign<i64> for Modnum<M> {
    fn sub_assign(&mut self, rhs: i64) {
        self.x = (self.x - rhs % M + M) % M;
    }
}
impl<const M: i64> MulAssign<Modnum<M>> for Modnum<M> {
    fn mul_assign(&mut self, rhs: Modnum<M>) {
        self.x = (self.x * rhs.x) % M;
    }
}
impl<const M: i64> MulAssign<i64> for Modnum<M> {
    fn mul_assign(&mut self, rhs: i64) {
        self.x = (self.x * (rhs % M)) % M;
    }
}
