use super::deck::{MatrixSpec, num_text};

/// A cable net: nodes, cables, force densities and loads.
#[derive(Clone, Debug, Default)]
pub struct Net {
    pub names: Vec<String>,
    pub at: Vec<[f64; 3]>,
    pub fixed: Vec<bool>,
    pub cables: Vec<(String, usize, usize)>,
    pub q: Vec<f64>,
    pub load: Vec<[f64; 3]>,
}

/// Solve `a x = b` by Gaussian elimination; `a` becomes upper triangular.
fn eliminate(a: &mut [Vec<f64>], b: &mut [[f64; 3]]) {
    let n = a.len();
    for k in 0..n {
        for r in k + 1..n {
            let f = a[r][k] / a[k][k];
            if f == 0.0 {
                continue;
            }
            for c in k..n {
                a[r][c] -= f * a[k][c];
            }
            for d in 0..3 {
                b[r][d] -= f * b[k][d];
            }
        }
    }
    for k in (0..n).rev() {
        for d in 0..3 {
            let mut s = b[k][d];
            for c in k + 1..n {
                s -= a[k][c] * b[c][d];
            }
            b[k][d] = s / a[k][k];
        }
    }
}

impl Net {
    /// A square grid of `n` by `m` nodes, one unit apart, named `n<row>_<col>`.
    pub fn grid(n: usize, m: usize) -> Net {
        let mut net = Net::default();
        for r in 0..n {
            for c in 0..m {
                net.node(&format!("n{r}_{c}"), [c as f64, r as f64, 0.0], false);
            }
        }
        for r in 0..n {
            for c in 0..m {
                let i = r * m + c;
                if c + 1 < m {
                    net.cable(&format!("h{r}_{c}"), i, i + 1, 1.0);
                }
                if r + 1 < n {
                    net.cable(&format!("v{r}_{c}"), i, i + m, 1.0);
                }
            }
        }
        net
    }

    /// Append a node with zero load.
    pub fn node(&mut self, name: &str, at: [f64; 3], fixed: bool) {
        self.names.push(name.into());
        self.at.push(at);
        self.fixed.push(fixed);
        self.load.push([0.0; 3]);
    }

    /// Append a cable from node `a` to node `b` with force density `q`.
    pub fn cable(&mut self, name: &str, a: usize, b: usize, q: f64) {
        self.cables.push((name.into(), a, b));
        self.q.push(q);
    }

    /// The index of the node called `name`.
    pub fn find(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }

    /// Indices of the free nodes.
    pub fn free(&self) -> Vec<usize> {
        (0..self.names.len()).filter(|&i| !self.fixed[i]).collect()
    }

    /// Indices of the fixed nodes.
    pub fn anchors(&self) -> Vec<usize> {
        (0..self.names.len()).filter(|&i| self.fixed[i]).collect()
    }

    /// How many cables meet at a node.
    pub fn degree(&self, i: usize) -> usize {
        self.cables.iter().filter(|c| c.1 == i || c.2 == i).count()
    }

    /// The branch-node matrix restricted to `nodes`: +1 where a cable starts, -1 where it ends.
    fn connectivity(&self, nodes: &[usize]) -> Vec<Vec<f64>> {
        let mut m = vec![vec![0.0; nodes.len()]; self.cables.len()];
        for (r, (_, a, b)) in self.cables.iter().enumerate() {
            for (c, &i) in nodes.iter().enumerate() {
                if i == *a {
                    m[r][c] = 1.0;
                }
                if i == *b {
                    m[r][c] = -1.0;
                }
            }
        }
        m
    }

    // --8<-- [start:stage-net-equilibrium]
    /// `C_rows^T Q C_cols`.
    fn density(&self, rows: &[usize], cols: &[usize]) -> Vec<Vec<f64>> {
        let (cr, cc) = (self.connectivity(rows), self.connectivity(cols));
        let mut m = vec![vec![0.0; cols.len()]; rows.len()];
        for k in 0..self.cables.len() {
            for i in 0..rows.len() {
                if cr[k][i] == 0.0 {
                    continue;
                }
                for j in 0..cols.len() {
                    m[i][j] += cr[k][i] * self.q[k] * cc[k][j];
                }
            }
        }
        m
    }

    /// The right-hand side `p - D_F x_F`, one row per free node.
    fn rhs(&self) -> Vec<[f64; 3]> {
        let (free, fixed) = (self.free(), self.anchors());
        let df = self.density(&free, &fixed);
        let mut b: Vec<[f64; 3]> = free.iter().map(|&i| self.load[i]).collect();
        for i in 0..free.len() {
            for (j, &f) in fixed.iter().enumerate() {
                for d in 0..3 {
                    b[i][d] -= df[i][j] * self.at[f][d];
                }
            }
        }
        b
    }

    /// Positions of all nodes at equilibrium: `D_N x_N = p - D_F x_F` for x, y and z.
    pub fn solve(&self) -> Vec<[f64; 3]> {
        let free = self.free();
        let mut a = self.density(&free, &free);
        let mut b = self.rhs();
        eliminate(&mut a, &mut b);
        let mut at = self.at.clone();
        for (k, &i) in free.iter().enumerate() {
            at[i] = b[k];
        }
        at
    }

    // --8<-- [end:stage-net-equilibrium]
    /// A matrix of the method by its name, with the solved shape where lengths are needed.
    pub fn matrix(&self, name: &str) -> Option<MatrixSpec> {
        let (free, fixed) = (self.free(), self.anchors());
        let all: Vec<usize> = free.iter().chain(&fixed).copied().collect();
        let solved = self.solve();
        let len = |k: usize| {
            let (_, a, b) = &self.cables[k];
            (0..3)
                .map(|d| (solved[*a][d] - solved[*b][d]).powi(2))
                .sum::<f64>()
                .sqrt()
        };
        let diag = |v: Vec<f64>| -> Vec<Vec<f64>> {
            let n = v.len();
            (0..n)
                .map(|r| (0..n).map(|c| if r == c { v[r] } else { 0.0 }).collect())
                .collect()
        };
        let coords = |nodes: &[usize], at: &[[f64; 3]]| -> Vec<Vec<f64>> {
            nodes.iter().map(|&i| at[i].to_vec()).collect()
        };
        let cables: Vec<String> = self.cables.iter().map(|c| tex_name(&c.0)).collect();
        let node = |nodes: &[usize]| -> Vec<String> {
            nodes.iter().map(|&i| tex_name(&self.names[i])).collect()
        };
        let xyz = || vec!["x".to_string(), "y".into(), "z".into()];
        let (rows, row_labels, col_labels, title): (Vec<Vec<f64>>, Vec<String>, Vec<String>, &str) =
            match name {
                "C" => (self.connectivity(&all), cables, node(&all), r"\mathbf{C}"),
                "CN" => (
                    self.connectivity(&free),
                    cables,
                    node(&free),
                    r"\mathbf{C}_N",
                ),
                "CF" => (
                    self.connectivity(&fixed),
                    cables,
                    node(&fixed),
                    r"\mathbf{C}_F",
                ),
                "Q" => (diag(self.q.clone()), cables, vec![], r"\mathbf{Q}"),
                "q" => (
                    self.q.iter().map(|q| vec![*q]).collect(),
                    cables,
                    vec![],
                    r"\mathbf{q}",
                ),
                "L" => (
                    diag((0..self.q.len()).map(len).collect()),
                    cables,
                    vec![],
                    r"\mathbf{L}",
                ),
                "l" => (
                    (0..self.q.len()).map(|k| vec![len(k)]).collect(),
                    cables,
                    vec![],
                    r"\mathbf{l}",
                ),
                "f" => (
                    (0..self.q.len())
                        .map(|k| vec![len(k) * self.q[k]])
                        .collect(),
                    cables,
                    vec![],
                    r"\mathbf{f}",
                ),
                "DN" => (
                    self.density(&free, &free),
                    node(&free),
                    node(&free),
                    r"\mathbf{D}_N",
                ),
                "DF" => (
                    self.density(&free, &fixed),
                    node(&free),
                    node(&fixed),
                    r"\mathbf{D}_F",
                ),
                "U" => {
                    let mut a = self.density(&free, &free);
                    eliminate(&mut a, &mut self.rhs());
                    (a, node(&free), node(&free), r"\mathbf{U}")
                }
                "xF" => (
                    coords(&fixed, &self.at),
                    node(&fixed),
                    xyz(),
                    r"\mathbf{x}_F",
                ),
                "xN" => (coords(&free, &solved), node(&free), xyz(), r"\mathbf{x}_N"),
                "p" => (
                    free.iter().map(|&i| self.load[i].to_vec()).collect(),
                    node(&free),
                    xyz(),
                    r"\mathbf{p}",
                ),
                "b" => (
                    self.rhs().iter().map(|r| r.to_vec()).collect(),
                    node(&free),
                    xyz(),
                    r"\mathbf{p} - \mathbf{D}_F\mathbf{x}_F",
                ),
                _ => return None,
            };
        let big = rows.len() > 14 || rows.first().is_some_and(|r| r.len() > 14);
        Some(MatrixSpec {
            rows: rows
                .iter()
                .map(|r| r.iter().map(|v| num_text(*v)).collect())
                .collect(),
            name: Some(title.into()),
            rowlabels: if big { vec![] } else { row_labels },
            collabels: if big { vec![] } else { col_labels },
            signs: true,
            pattern: big,
            ..MatrixSpec::default()
        })
    }
}

/// `P0` as TeX `P_0`, `n3_4` as `n_{3,4}`.
pub fn tex_name(name: &str) -> String {
    let split = name
        .find(|c: char| c.is_ascii_digit())
        .unwrap_or(name.len());
    let (head, tail) = name.split_at(split);
    if tail.is_empty() || head.is_empty() {
        return name.into();
    }
    format!("{head}_{{{}}}", tail.replace('_', ","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_nodes_rest_at_the_weighted_average() {
        let mut net = Net::default();
        net.node("P0", [3.6, 3.4, 1.9], false);
        for (name, at) in [
            ("P1", [0.0, 0.0, 0.0]),
            ("P2", [5.0, 0.0, 3.0]),
            ("P3", [0.0, 7.0, 3.0]),
            ("P4", [7.0, 5.0, 0.0]),
        ] {
            net.node(name, at, true);
        }
        for (k, name) in ["a", "b", "c", "d"].iter().enumerate() {
            net.cable(name, 0, k + 1, 1.0);
        }
        net.load[0] = [0.0, 0.0, -5.0];
        let at = net.solve()[0];
        assert!(
            (at[0] - 3.0).abs() < 1e-12
                && (at[1] - 3.0).abs() < 1e-12
                && (at[2] - 0.25).abs() < 1e-12
        );
        assert_eq!(net.matrix("DN").unwrap().rows, vec![vec!["4".to_string()]]);
        assert_eq!(net.matrix("f").unwrap().rows[0][0], "4.25");
    }

    #[test]
    fn a_grid_has_the_cables_of_its_rows_and_columns() {
        let net = Net::grid(10, 10);
        assert_eq!(net.names.len(), 100);
        assert_eq!(net.cables.len(), 180);
        assert!(net.matrix("C").unwrap().pattern);
    }
}
