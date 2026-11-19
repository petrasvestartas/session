use super::deck::{Pos, Scalar};

/// A scalar or a 3D vector.
#[derive(Clone, Copy, Debug)]
pub enum Val {
    S(f64),
    V([f64; 3]),
}

/// Named points by value.
pub trait Points {
    fn point(&self, name: &str) -> Option<[f64; 3]>;
}

/// A recursive-descent reader of one expression over named points.
struct Parser<'a, T: Points> {
    chars: Vec<char>,
    at: usize,
    pts: &'a T,
}

/// `a + sign * b` for two numbers or two points.
fn add(a: Val, b: Val, sign: f64) -> Result<Val, String> {
    match (a, b) {
        (Val::S(a), Val::S(b)) => Ok(Val::S(a + sign * b)),
        (Val::V(a), Val::V(b)) => Ok(Val::V([
            a[0] + sign * b[0],
            a[1] + sign * b[1],
            a[2] + sign * b[2],
        ])),
        _ => Err("cannot add a number and a point".into()),
    }
}

/// `a * b`, or `a / b` when `div`, for numbers and a point scaled by a number.
fn mul(a: Val, b: Val, div: bool) -> Result<Val, String> {
    let f = |x: f64, y: f64| if div { x / y } else { x * y };
    match (a, b) {
        (Val::S(a), Val::S(b)) => Ok(Val::S(f(a, b))),
        (Val::V(v), Val::S(s)) => Ok(Val::V([f(v[0], s), f(v[1], s), f(v[2], s)])),
        (Val::S(s), Val::V(v)) if !div => Ok(Val::V([s * v[0], s * v[1], s * v[2]])),
        _ => Err("cannot multiply two points".into()),
    }
}

impl<T: Points> Parser<'_, T> {
    fn skip_space(&mut self) {
        while self.at < self.chars.len() && self.chars[self.at].is_whitespace() {
            self.at += 1;
        }
    }

    fn eat(&mut self, c: char) -> bool {
        self.skip_space();
        if self.chars.get(self.at) == Some(&c) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    fn expr(&mut self) -> Result<Val, String> {
        let mut v = self.term()?;
        loop {
            if self.eat('+') {
                v = add(v, self.term()?, 1.0)?;
            } else if self.eat('-') {
                v = add(v, self.term()?, -1.0)?;
            } else {
                return Ok(v);
            }
        }
    }

    fn term(&mut self) -> Result<Val, String> {
        let mut v = self.factor()?;
        loop {
            if self.eat('*') {
                v = mul(v, self.factor()?, false)?;
            } else if self.eat('/') {
                v = mul(v, self.factor()?, true)?;
            } else {
                return Ok(v);
            }
        }
    }

    fn scalar(&mut self) -> Result<f64, String> {
        match self.expr()? {
            Val::S(s) => Ok(s),
            Val::V(_) => Err("expected a number".into()),
        }
    }

    fn factor(&mut self) -> Result<Val, String> {
        if self.eat('-') {
            return mul(Val::S(-1.0), self.factor()?, false);
        }
        if self.eat('(') {
            let v = self.expr()?;
            self.eat(')');
            return Ok(v);
        }
        if self.eat('[') {
            let x = self.scalar()?;
            self.eat(',');
            let y = self.scalar()?;
            self.eat(',');
            let z = self.scalar()?;
            self.eat(']');
            return Ok(Val::V([x, y, z]));
        }
        let start = self.at;
        let c = *self.chars.get(self.at).ok_or("unexpected end")?;
        if c.is_ascii_digit() || c == '.' {
            while self.at < self.chars.len()
                && (self.chars[self.at].is_ascii_digit()
                    || self.chars[self.at] == '.'
                    || self.chars[self.at] == 'e')
            {
                self.at += 1;
            }
            let t: String = self.chars[start..self.at].iter().collect();
            return t.parse().map(Val::S).map_err(|_| format!("bad number {t}"));
        }
        while self.at < self.chars.len()
            && (self.chars[self.at].is_alphanumeric() || self.chars[self.at] == '_')
        {
            self.at += 1;
        }
        let name: String = self.chars[start..self.at].iter().collect();
        if name.is_empty() {
            return Err(format!("unexpected `{c}`"));
        }
        if self.eat('(') {
            let mut args = Vec::new();
            if !self.eat(')') {
                loop {
                    args.push(self.expr()?);
                    if self.eat(')') {
                        break;
                    }
                    if !self.eat(',') {
                        return Err(format!("{name}(...) is not closed"));
                    }
                }
            }
            let v = |i: usize| match args.get(i) {
                Some(Val::V(v)) => Ok(*v),
                _ => Err(format!("{name} expects a point as argument {}", i + 1)),
            };
            let s = |i: usize| match args.get(i) {
                Some(Val::S(s)) => Ok(*s),
                _ => Err(format!("{name} expects a number as argument {}", i + 1)),
            };
            let d = |a: [f64; 3], b: [f64; 3]| {
                ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2) + (b[2] - a[2]).powi(2)).sqrt()
            };
            return Ok(match name.as_str() {
                "x" => Val::S(v(0)?[0]),
                "y" => Val::S(v(0)?[1]),
                "z" => Val::S(v(0)?[2]),
                "len" | "dist" => Val::S(d(v(0)?, v(1)?)),
                "mid" => {
                    let (a, b) = (v(0)?, v(1)?);
                    Val::V([
                        (a[0] + b[0]) / 2.0,
                        (a[1] + b[1]) / 2.0,
                        (a[2] + b[2]) / 2.0,
                    ])
                }
                "lerp" => {
                    let (a, b, t) = (v(0)?, v(1)?, s(2)?);
                    Val::V([
                        a[0] + (b[0] - a[0]) * t,
                        a[1] + (b[1] - a[1]) * t,
                        a[2] + (b[2] - a[2]) * t,
                    ])
                }
                "unit" => {
                    let a = v(0)?;
                    let l = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt().max(1e-12);
                    Val::V([a[0] / l, a[1] / l, a[2] / l])
                }
                "norm" => {
                    let a = v(0)?;
                    Val::S((a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt())
                }
                "sqrt" => Val::S(s(0)?.sqrt()),
                "abs" => Val::S(s(0)?.abs()),
                _ => return Err(format!("unknown function {name}")),
            });
        }
        self.pts
            .point(&name)
            .map(Val::V)
            .ok_or_else(|| format!("unknown point {name}"))
    }
}

/// Evaluate an expression such as `mid(P0,P1) + [0,0,0.3]` or `len(P0,P1)`.
pub fn eval<T: Points>(src: &str, pts: &T) -> Result<Val, String> {
    let mut parser = Parser {
        chars: src.chars().collect(),
        at: 0,
        pts,
    };
    let v = parser.expr()?;
    parser.skip_space();
    if parser.at < parser.chars.len() {
        let rest = parser.chars[parser.at];
        return Err(format!("unexpected `{rest}` in `{src}`"));
    }
    Ok(v)
}

/// A position spec at the current point values.
pub fn position<T: Points>(pos: &Pos, pts: &T) -> Result<[f64; 3], String> {
    match pos {
        Pos::Name(n) => pts.point(n).ok_or_else(|| format!("unknown point {n}")),
        Pos::Abs(c) => {
            let mut out = [0.0; 3];
            for (i, s) in c.iter().enumerate() {
                out[i] = match s {
                    Scalar::N(x) => *x,
                    Scalar::E(e) => match eval(e, pts)? {
                        Val::S(x) => x,
                        Val::V(_) => return Err(format!("`{e}` is a point, not a coordinate")),
                    },
                };
            }
            Ok(out)
        }
        Pos::Expr(e) => match eval(e, pts)? {
            Val::V(v) => Ok(v),
            Val::S(_) => Err(format!("`{e}` is a number, not a point")),
        },
    }
}

/// Replace `{=expr}` and `{=expr:.3}` in a TeX string with values; minus signs stay TeX.
pub fn fill<T: Points>(tex: &str, pts: &T) -> String {
    if !tex.contains("{=") {
        return tex.to_string();
    }
    let mut out = String::new();
    let mut rest = tex;
    while let Some(start) = rest.find("{=") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find('}') else {
            out.push_str(&rest[start..]);
            return out;
        };
        let inner = &after[..end];
        let (expr, digits) = match inner.rsplit_once(":.") {
            Some((e, d)) => (e, d.parse::<usize>().unwrap_or(3)),
            None => (inner, 3),
        };
        let text = match eval(expr, pts) {
            Ok(Val::S(x)) => {
                let x = if x.abs() < 0.5 * 10f64.powi(-(digits as i32)) {
                    0.0
                } else {
                    x
                };
                format!("{x:.digits$}")
            }
            Ok(Val::V(v)) => format!("({:.digits$}, {:.digits$}, {:.digits$})", v[0], v[1], v[2]),
            Err(_) => "?".into(),
        };
        out.push_str(&text);
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}
