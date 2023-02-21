pub struct C{
    r: isize,
    i: isize
}

impl C {
    pub fn new(r: isize, i: isize) -> Self{
        Self{ r, i }
    }
}

impl std::ops::Add for C{
    type Output = Self;
    fn add(self, x: Self) -> Self::Output {
        Self{
            r: self.r + x.r, 
            i: self.i + x.i
        }
    }
}

impl std::ops::Sub for C{
    type Output = Self;
    fn sub(self, x: Self) -> Self::Output {
        Self{
            r: self.r - x.r, 
            i: self.i - x.i
        }
    }
}

impl std::ops::AddAssign for C{
    fn add_assign(&mut self, x: Self) {
        self.r += x.r; 
        self.i += x.i;
    }
}

impl std::ops::SubAssign for C{
    fn sub_assign(&mut self, x: Self) {
        self.r -= x.r; 
        self.i -= x.i;
    }
}

impl std::ops::MulAssign for C{
    fn mul_assign(&mut self, x: Self) {
        let (r, i) = (self.r, self.i);
        self.r = r * x.r;
        self.r -= i * x.i;

        self.i = r * x.i;
        self.i += i * x.r;
    }
}

impl std::fmt::Display for C{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (mut r, mut i, mut ops) = ("".to_string(), "".to_string(), "".to_string());
        if self.i > 0{
            ops = " + ".to_string();
            if self.i == 0{
                i = i
            }else{
                i = format!("{}i", self.i.abs());
            }

        }else if self.i < 0{
            ops = " - ".to_string();
            i = format!("{}i", self.i.abs());
        }
        
        r = if self.r != 0{
            self.r.to_string()
        }else{
            ops = format!("{} ", ops.trim());
            r
        };

        write!(f, "{r}{ops}{i}")
    }
}