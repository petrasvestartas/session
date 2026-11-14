// --8<-- [start:implementation]
use super::font::{BOLD, Construction, FontId, Fonts, MATH, Part, TEXT};
use std::collections::HashMap;

/// A colour as red, green, blue and alpha in 0..1.
pub type Rgba = [f32; 4];

/// A drawable piece of a formula, em units, y down from the baseline.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Glyph {
        font: FontId,
        gid: u16,
        size: f32,
    },
    Rect {
        w: f32,
        h: f32,
        radius: f32,
        fill: f32,
        stroke: f32,
    },
    Stroke {
        dx: f32,
        dy: f32,
        width: f32,
    },
    Image {
        index: u16,
        w: f32,
        h: f32,
    },
}

/// A shape at its position; `x, y` is the glyph pen point or the rect's top-left corner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Prim {
    pub shape: Shape,
    pub x: f32,
    pub y: f32,
    pub color: Option<Rgba>,
    pub part: Option<u16>,
}

/// A laid-out formula or paragraph, em units.
#[derive(Clone, Debug, Default)]
pub struct Expr {
    pub prims: Vec<Prim>,
    pub parts: Vec<String>,
    pub w: f32,
    pub h: f32,
    pub d: f32,
    pub align: Option<f32>,
    pub unknown: Vec<String>,
}

impl Shape {
    /// What morph matching compares: equal glyphs, or two rules.
    pub fn ident(&self) -> u64 {
        match *self {
            Shape::Glyph { font, gid, .. } => ((font as u64) << 16) | gid as u64,
            Shape::Rect { .. } => 1 << 40,
            Shape::Stroke { .. } => 2 << 40,
            Shape::Image { index, .. } => (3 << 40) | index as u64,
        }
    }

    /// The shape `k` times larger, keeping a rect's corner radius and outline width.
    pub fn resized(self, k: f32) -> Shape {
        match self {
            Shape::Glyph { font, gid, size } => Shape::Glyph {
                font,
                gid,
                size: size * k,
            },
            Shape::Rect {
                w,
                h,
                radius,
                fill,
                stroke,
            } => Shape::Rect {
                w: w * k,
                h: h * k,
                radius,
                fill,
                stroke,
            },
            Shape::Stroke { dx, dy, width } => Shape::Stroke {
                dx: dx * k,
                dy: dy * k,
                width: width * k,
            },
            Shape::Image { index, w, h } => Shape::Image {
                index,
                w: w * k,
                h: h * k,
            },
        }
    }
}

impl Expr {
    /// Prims tagged with the part whose key is `key`.
    pub fn part(&self, key: &str) -> Vec<usize> {
        let ids: Vec<u16> = self
            .parts
            .iter()
            .enumerate()
            .filter(|(_, k)| k.as_str() == key)
            .map(|(i, _)| i as u16)
            .collect();
        self.prims
            .iter()
            .enumerate()
            .filter(|(_, p)| p.part.is_some_and(|id| ids.contains(&id)))
            .map(|(i, _)| i)
            .collect()
    }
}

/// TeX's atom classes, which decide the space between neighbours.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Class {
    Ord,
    Op,
    Bin,
    Rel,
    Open,
    Close,
    Punct,
    Inner,
}

/// The math alphabet a letter is drawn from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Variant {
    Italic,
    Upright,
    Bold,
    BoldItalic,
}

/// TeX's math style.
#[derive(Clone, Copy, Debug)]
struct Style {
    level: u8,     // 0 display, 1 text, 2 script, 3 scriptscript.
    cramped: bool, // Superscripts sit lower.
}

impl Style {
    const DISPLAY: Style = Style {
        level: 0,
        cramped: false,
    };
    const TEXT: Style = Style {
        level: 1,
        cramped: false,
    };
    fn scale(self, f: &Fonts) -> f32 {
        match self.level {
            0 | 1 => 1.0,
            2 => f.c.script,
            _ => f.c.script_script,
        }
    }
    fn sup(self) -> Style {
        Style {
            level: if self.level < 2 { 2 } else { 3 },
            cramped: self.cramped,
        }
    }
    fn sub(self) -> Style {
        Style {
            cramped: true,
            ..self.sup()
        }
    }
    fn num(self) -> Style {
        Style {
            level: (self.level + 1).clamp(1, 3),
            cramped: self.cramped,
        }
    }
    fn den(self) -> Style {
        Style {
            cramped: true,
            ..self.num()
        }
    }
    fn cramp(self) -> Style {
        Style {
            cramped: true,
            ..self
        }
    }
    fn script(self) -> bool {
        self.level >= 2
    }
}

/// A parsed piece of a formula.
#[derive(Clone, Debug)]
enum Node {
    Char(char, Variant),
    Group(Vec<Node>),
    Scripts {
        base: Box<Node>,
        sup: Option<Box<Node>>,
        sub: Option<Box<Node>>,
    },
    Frac {
        num: Vec<Node>,
        den: Vec<Node>,
        style: Option<Style>,
    },
    Sqrt(Vec<Node>),
    Delim {
        left: Option<char>,
        right: Option<char>,
        body: Vec<Node>,
    },
    Big(char, f32, Class),
    Text(String, bool),
    Space(f32),
    OpGlyph(char, bool),
    OpWord(String, bool),
    Accent(char, Vec<Node>, bool),
    Overline(Vec<Node>),
    Underline(Vec<Node>),
    Brace {
        body: Vec<Node>,
        label: Option<Vec<Node>>,
        under: bool,
    },
    Cancel(Vec<Node>),
    XArrow(Vec<Node>, char),
    Phantom(Vec<Node>),
    Boxed(Vec<Node>),
    StyleSet(Style),
    ColorPush(Rgba),
    ColorPop,
    PartPush(u16),
    PartPop,
    Align,
}

/// A lexed TeX token.
#[derive(Clone, Debug, PartialEq)]
enum Tok {
    Cmd(String),
    Ch(char),
    Open,
    Close,
    Sup,
    Sub,
    PartOpen,
    PartClose,
    Amp,
    Raw(String),
}

/// Commands whose braced argument is kept as raw text.
const RAW: [&str; 5] = ["text", "textrm", "textbf", "mbox", "operatorname"];

// --8<-- [start:lexer]
/// Split TeX source into tokens.
fn lex(src: &str) -> Vec<Tok> {
    let chars: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '\\' => {
                i += 1;
                if i >= chars.len() {
                    break;
                }
                if chars[i].is_ascii_alphabetic() {
                    let start = i;
                    while i < chars.len() && chars[i].is_ascii_alphabetic() {
                        i += 1;
                    }
                    let name: String = chars[start..i].iter().collect();
                    let raw = RAW.contains(&name.as_str());
                    out.push(Tok::Cmd(name));
                    if raw {
                        let mut j = i;
                        while j < chars.len() && chars[j] == ' ' {
                            j += 1;
                        }
                        if chars.get(j) == Some(&'{') {
                            let mut depth = 0;
                            let mut k = j;
                            while k < chars.len() {
                                if chars[k] == '{' {
                                    depth += 1;
                                } else if chars[k] == '}' {
                                    depth -= 1;
                                    if depth == 0 {
                                        break;
                                    }
                                }
                                k += 1;
                            }
                            out.push(Tok::Raw(chars[j + 1..k.min(chars.len())].iter().collect()));
                            i = k + 1;
                        }
                    }
                    continue;
                }
                out.push(Tok::Cmd(chars[i].to_string()));
            }
            '{' if chars.get(i + 1) == Some(&'{') => {
                out.push(Tok::PartOpen);
                i += 1;
            }
            '}' if chars.get(i + 1) == Some(&'}') && part_depth(&out) > 0 => {
                out.push(Tok::PartClose);
                i += 1;
            }
            '{' => out.push(Tok::Open),
            '}' => out.push(Tok::Close),
            '^' => out.push(Tok::Sup),
            '_' => out.push(Tok::Sub),
            '&' => out.push(Tok::Amp),
            '%' => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            c if c.is_whitespace() => {}
            c => out.push(Tok::Ch(c)),
        }
        i += 1;
    }
    out
}

// --8<-- [end:lexer]
/// Open `{{` parts not yet closed, so `}}` only closes a part when one is open.
fn part_depth(toks: &[Tok]) -> i32 {
    let mut depth = 0;
    let mut braces: Vec<bool> = Vec::new();
    for t in toks {
        match t {
            Tok::PartOpen => {
                depth += 1;
                braces.push(true);
            }
            Tok::PartClose => {
                depth -= 1;
                braces.pop();
            }
            Tok::Open => braces.push(false),
            Tok::Close => {
                braces.pop();
            }
            _ => {}
        }
    }
    // a `}}` right after an ordinary open brace inside the part closes that brace first
    if braces.last() == Some(&false) {
        0
    } else {
        depth
    }
}

/// Reads tokens into nodes, collecting part keys and unknown commands.
struct Parser<'a> {
    toks: Vec<Tok>,
    pos: usize,
    palette: &'a HashMap<String, Rgba>,
    parts: Vec<String>,
    unknown: Vec<String>,
}

/// The character a symbol command stands for.
fn symbol(name: &str) -> Option<char> {
    Some(match name {
        "alpha" => 'α',
        "beta" => 'β',
        "gamma" => 'γ',
        "delta" => 'δ',
        "epsilon" => 'ϵ',
        "varepsilon" => 'ε',
        "zeta" => 'ζ',
        "eta" => 'η',
        "theta" => 'θ',
        "vartheta" => 'ϑ',
        "iota" => 'ι',
        "kappa" => 'κ',
        "lambda" => 'λ',
        "mu" => 'μ',
        "nu" => 'ν',
        "xi" => 'ξ',
        "pi" => 'π',
        "varpi" => 'ϖ',
        "rho" => 'ρ',
        "varrho" => 'ϱ',
        "sigma" => 'σ',
        "varsigma" => 'ς',
        "tau" => 'τ',
        "upsilon" => 'υ',
        "phi" => 'ϕ',
        "varphi" => 'φ',
        "chi" => 'χ',
        "psi" => 'ψ',
        "omega" => 'ω',
        "Gamma" => 'Γ',
        "Delta" => 'Δ',
        "Theta" => 'Θ',
        "Lambda" => 'Λ',
        "Xi" => 'Ξ',
        "Pi" => 'Π',
        "Sigma" => 'Σ',
        "Upsilon" => 'Υ',
        "Phi" => 'Φ',
        "Psi" => 'Ψ',
        "Omega" => 'Ω',
        "partial" => '∂',
        "nabla" => '∇',
        "infty" => '∞',
        "cdot" => '⋅',
        "times" => '×',
        "div" => '÷',
        "pm" => '±',
        "mp" => '∓',
        "approx" => '≈',
        "neq" | "ne" => '≠',
        "le" | "leq" => '≤',
        "ge" | "geq" => '≥',
        "to" | "rightarrow" => '→',
        "leftarrow" | "gets" => '←',
        "Rightarrow" => '⇒',
        "Leftarrow" => '⇐',
        "Leftrightarrow" => '⇔',
        "implies" | "Longrightarrow" => '⟹',
        "iff" | "Longleftrightarrow" => '⟺',
        "longrightarrow" => '⟶',
        "mapsto" => '↦',
        "equiv" => '≡',
        "sim" => '∼',
        "simeq" => '≃',
        "propto" => '∝',
        "in" => '∈',
        "ldots" | "dots" => '…',
        "cdots" => '⋯',
        "vdots" => '⋮',
        "ddots" => '⋱',
        "prime" => '′',
        "circ" => '∘',
        "ast" => '∗',
        "star" => '⋆',
        "bullet" => '∙',
        "mid" => '∣',
        "|" => '‖',
        "{" | "lbrace" => '{',
        "}" | "rbrace" => '}',
        "langle" => '⟨',
        "rangle" => '⟩',
        "lvert" | "rvert" | "vert" => '|',
        "ell" => 'ℓ',
        "forall" => '∀',
        "exists" => '∃',
        "emptyset" => '∅',
        "%" => '%',
        "#" => '#',
        "&" => '&',
        "_" => '_',
        "perp" => '⊥',
        "parallel" => '∥',
        "angle" => '∠',
        "triangle" => '△',
        "degree" => '°',
        _ => return None,
    })
}

/// The width in em of a spacing command.
fn space(name: &str) -> Option<f32> {
    Some(match name {
        "," | "thinspace" => 3.0 / 18.0,
        ":" | ">" | "medspace" => 4.0 / 18.0,
        ";" | "thickspace" => 5.0 / 18.0,
        "!" => -3.0 / 18.0,
        " " => 0.333,
        "quad" => 1.0,
        "qquad" => 2.0,
        _ => return None,
    })
}

/// The size and class of a `\big`-family command.
fn big(name: &str) -> Option<(f32, Class)> {
    let (size, rest) = if let Some(r) = name.strip_prefix("Bigg") {
        (3.0, r)
    } else if let Some(r) = name.strip_prefix("bigg") {
        (2.4, r)
    } else if let Some(r) = name.strip_prefix("Big") {
        (1.8, r)
    } else if let Some(r) = name.strip_prefix("big") {
        (1.2, r)
    } else {
        return None;
    };
    let class = match rest {
        "" => Class::Inner,
        "l" => Class::Open,
        "r" => Class::Close,
        "m" => Class::Rel,
        _ => return None,
    };
    Some((size, class))
}

/// Parse a hex color `#rrggbb[aa]`.
pub fn hex(text: &str) -> Option<Rgba> {
    let t = text.trim().trim_start_matches('#');
    let byte = |i: usize| {
        u8::from_str_radix(t.get(i..i + 2)?, 16)
            .ok()
            .map(|b| b as f32 / 255.0)
    };
    match t.len() {
        6 => Some([byte(0)?, byte(2)?, byte(4)?, 1.0]),
        8 => Some([byte(0)?, byte(2)?, byte(4)?, byte(6)?]),
        _ => None,
    }
}

impl Parser<'_> {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn next(&mut self) -> Option<Tok> {
        let t = self.toks.get(self.pos).cloned();
        self.pos += 1;
        t
    }

    /// Nodes until a closing brace, `}}` or the end.
    fn list(&mut self, variant: Variant) -> Vec<Node> {
        let mut out: Vec<Node> = Vec::new();
        let mut colored = 0;
        while let Some(t) = self.peek().cloned() {
            match t {
                Tok::Close | Tok::PartClose => break,
                Tok::Cmd(ref c) if c == "right" => break,
                Tok::Sup | Tok::Sub => {
                    self.pos += 1;
                    let arg = self.arg(variant);
                    let base_at = out.iter().rposition(|n| !marker(n));
                    let base = match base_at {
                        Some(i) => out.remove(i),
                        None => Node::Group(Vec::new()),
                    };
                    let at = base_at.unwrap_or(out.len());
                    let node = match base {
                        Node::Brace {
                            body,
                            label: None,
                            under,
                        } if (t == Tok::Sub) == under => Node::Brace {
                            body,
                            label: Some(vec![arg]),
                            under,
                        },
                        Node::Scripts { base, sup, sub } => {
                            let (mut sup, mut sub) = (sup, sub);
                            if t == Tok::Sup && sup.is_none() {
                                sup = Some(Box::new(arg));
                            } else if t == Tok::Sub && sub.is_none() {
                                sub = Some(Box::new(arg));
                            } else {
                                out.insert(at, Node::Scripts { base, sup, sub });
                                let empty = Box::new(Node::Group(Vec::new()));
                                let (sup, sub) = script_slots(&t, arg);
                                out.insert(
                                    at + 1,
                                    Node::Scripts {
                                        base: empty,
                                        sup,
                                        sub,
                                    },
                                );
                                continue;
                            }
                            Node::Scripts { base, sup, sub }
                        }
                        base => {
                            let (sup, sub) = script_slots(&t, arg);
                            Node::Scripts {
                                base: Box::new(base),
                                sup,
                                sub,
                            }
                        }
                    };
                    out.insert(at, node);
                }
                Tok::Ch('\'') => {
                    self.pos += 1;
                    let mut primes = String::from('′');
                    while self.peek() == Some(&Tok::Ch('\'')) {
                        self.pos += 1;
                        primes.push('′');
                    }
                    let sup = Node::Group(
                        primes
                            .chars()
                            .map(|c| Node::Char(c, Variant::Upright))
                            .collect(),
                    );
                    let base = out.pop().unwrap_or(Node::Group(Vec::new()));
                    out.push(Node::Scripts {
                        base: Box::new(base),
                        sup: Some(Box::new(sup)),
                        sub: None,
                    });
                }
                Tok::Cmd(ref c) if c == "color" => {
                    self.pos += 1;
                    let rgba = self.color_arg();
                    out.push(Node::ColorPush(rgba));
                    colored += 1;
                }
                _ => {
                    if let Some(node) = self.atom(variant) {
                        out.push(node);
                    }
                }
            }
        }
        for _ in 0..colored {
            out.push(Node::ColorPop);
        }
        out
    }

    /// A colour argument: a palette name or a hex colour, white when neither.
    fn color_arg(&mut self) -> Rgba {
        let name = self.raw_arg();
        self.palette
            .get(name.trim())
            .copied()
            .or_else(|| hex(&name))
            .unwrap_or([1.0, 1.0, 1.0, 1.0])
    }

    /// The source text of a braced argument, or one token.
    fn raw_arg(&mut self) -> String {
        match self.next() {
            Some(Tok::Raw(s)) => s,
            Some(Tok::Open) => {
                let mut depth = 1;
                let mut s = String::new();
                while let Some(t) = self.next() {
                    match t {
                        Tok::Open => {
                            depth += 1;
                            s.push('{');
                        }
                        Tok::Close => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                            s.push('}');
                        }
                        Tok::Ch(c) => s.push(c),
                        Tok::Cmd(c) => {
                            s.push('\\');
                            s.push_str(&c);
                        }
                        Tok::Sup => s.push('^'),
                        Tok::Sub => s.push('_'),
                        Tok::Amp => s.push('&'),
                        Tok::PartOpen => s.push_str("{{"),
                        Tok::PartClose => s.push_str("}}"),
                        Tok::Raw(r) => {
                            s.push('{');
                            s.push_str(&r);
                            s.push('}');
                        }
                    }
                }
                s
            }
            Some(Tok::Ch(c)) => c.to_string(),
            Some(Tok::Cmd(c)) => c,
            _ => String::new(),
        }
    }

    /// A braced group or a single atom as one node.
    fn arg(&mut self, variant: Variant) -> Node {
        match self.peek() {
            Some(Tok::Open) => {
                self.pos += 1;
                let list = self.list(variant);
                if self.peek() == Some(&Tok::Close) {
                    self.pos += 1;
                }
                Node::Group(list)
            }
            _ => self.atom(variant).unwrap_or(Node::Group(Vec::new())),
        }
    }

    /// An argument as a node list.
    fn arg_list(&mut self, variant: Variant) -> Vec<Node> {
        match self.arg(variant) {
            Node::Group(list) => list,
            other => vec![other],
        }
    }

    /// A delimiter character, None for `.`.
    fn delim(&mut self) -> Option<char> {
        match self.next() {
            Some(Tok::Ch('.')) => None,
            Some(Tok::Ch(c)) => Some(c),
            Some(Tok::Cmd(c)) => symbol(&c),
            _ => None,
        }
    }

    /// One node, None for a token that ends a list.
    fn atom(&mut self, variant: Variant) -> Option<Node> {
        let t = self.next()?;
        Some(match t {
            Tok::Ch(c) => Node::Char(c, variant),
            Tok::Amp => Node::Align,
            Tok::Open => {
                let list = self.list(variant);
                if self.peek() == Some(&Tok::Close) {
                    self.pos += 1;
                }
                Node::Group(list)
            }
            Tok::PartOpen => {
                let start = self.pos;
                let id = self.parts.len() as u16;
                self.parts.push(String::new());
                let list = self.list(variant);
                let key = key_of(&self.toks[start..self.pos]);
                self.parts[id as usize] = key;
                if self.peek() == Some(&Tok::PartClose) {
                    self.pos += 1;
                }
                let mut nodes = vec![Node::PartPush(id)];
                nodes.extend(list);
                nodes.push(Node::PartPop);
                Node::Group(nodes)
            }
            Tok::Close | Tok::PartClose | Tok::Sup | Tok::Sub | Tok::Raw(_) => return None,
            Tok::Cmd(name) => return self.command(&name, variant),
        })
    }

    /// The node of a `\name` command; unknown commands are reported and shown as text.
    fn command(&mut self, name: &str, variant: Variant) -> Option<Node> {
        if let Some(c) = symbol(name) {
            let v = if c.is_ascii() {
                Variant::Upright
            } else {
                variant
            };
            return Some(Node::Char(c, v));
        }
        if let Some(s) = space(name) {
            return Some(Node::Space(s));
        }
        if let Some((size, class)) = big(name) {
            let c = self.delim()?;
            return Some(Node::Big(c, size, class));
        }
        Some(match name {
            "frac" | "dfrac" | "tfrac" => {
                let num = self.arg_list(variant);
                let den = self.arg_list(variant);
                let style = match name {
                    "dfrac" => Some(Style::DISPLAY),
                    "tfrac" => Some(Style::TEXT),
                    _ => None,
                };
                Node::Frac { num, den, style }
            }
            "sqrt" => Node::Sqrt(self.arg_list(variant)),
            "left" => {
                let left = self.delim();
                let body = self.list(variant);
                let right = if self.peek() == Some(&Tok::Cmd("right".into())) {
                    self.pos += 1;
                    self.delim()
                } else {
                    None
                };
                Node::Delim { left, right, body }
            }
            "mathbf" => Node::Group(self.arg_list(Variant::Bold)),
            "boldsymbol" | "bm" => Node::Group(self.arg_list(Variant::BoldItalic)),
            "mathrm" | "mathup" => Node::Group(self.arg_list(Variant::Upright)),
            "mathit" => Node::Group(self.arg_list(Variant::Italic)),
            "text" | "textrm" | "mbox" => Node::Text(self.raw_arg(), false),
            "textbf" => Node::Text(self.raw_arg(), true),
            "operatorname" => Node::OpWord(self.raw_arg(), false),
            "sum" => Node::OpGlyph('∑', true),
            "prod" => Node::OpGlyph('∏', true),
            "int" => Node::OpGlyph('∫', false),
            "lim" | "max" | "min" | "sup" | "inf" | "det" => Node::OpWord(name.into(), true),
            "cos" | "sin" | "tan" | "ln" | "log" | "exp" | "arccos" | "arcsin" | "arctan"
            | "diag" => Node::OpWord(name.into(), false),
            "hat" => Node::Accent('\u{302}', self.arg_list(variant), false),
            "widehat" => Node::Accent('\u{302}', self.arg_list(variant), true),
            "tilde" => Node::Accent('\u{303}', self.arg_list(variant), false),
            "bar" => Node::Accent('\u{304}', self.arg_list(variant), false),
            "dot" => Node::Accent('\u{307}', self.arg_list(variant), false),
            "ddot" => Node::Accent('\u{308}', self.arg_list(variant), false),
            "vec" => Node::Accent('\u{20D7}', self.arg_list(variant), false),
            "overrightarrow" => Node::Accent('\u{20D7}', self.arg_list(variant), true),
            "overline" => Node::Overline(self.arg_list(variant)),
            "underline" => Node::Underline(self.arg_list(variant)),
            "underbrace" => Node::Brace {
                body: self.arg_list(variant),
                label: None,
                under: true,
            },
            "overbrace" => Node::Brace {
                body: self.arg_list(variant),
                label: None,
                under: false,
            },
            "cancel" | "bcancel" | "xcancel" => Node::Cancel(self.arg_list(variant)),
            "xrightarrow" => Node::XArrow(self.arg_list(variant), '→'),
            "xleftarrow" => Node::XArrow(self.arg_list(variant), '←'),
            "phantom" => Node::Phantom(self.arg_list(variant)),
            "boxed" => Node::Boxed(self.arg_list(variant)),
            "displaystyle" => Node::StyleSet(Style::DISPLAY),
            "textstyle" => Node::StyleSet(Style::TEXT),
            "scriptstyle" => Node::StyleSet(Style {
                level: 2,
                cramped: false,
            }),
            "textcolor" => {
                let rgba = self.color_arg();
                let mut nodes = vec![Node::ColorPush(rgba)];
                nodes.extend(self.arg_list(variant));
                nodes.push(Node::ColorPop);
                Node::Group(nodes)
            }
            "limits" | "nolimits" | "notag" => Node::Space(0.0),
            "\\" => Node::Space(1.0),
            _ => {
                self.unknown.push(format!("\\{name}"));
                Node::Text(format!("\\{name}"), false)
            }
        })
    }
}

/// The sup and sub slots for a script token and its argument.
fn script_slots(t: &Tok, arg: Node) -> (Option<Box<Node>>, Option<Box<Node>>) {
    let script = Some(Box::new(arg));
    if *t == Tok::Sup {
        (script, None)
    } else {
        (None, script)
    }
}

/// Nodes that change state instead of drawing.
fn marker(n: &Node) -> bool {
    matches!(
        n,
        Node::ColorPush(_)
            | Node::ColorPop
            | Node::PartPush(_)
            | Node::PartPop
            | Node::Align
            | Node::StyleSet(_)
    )
}

/// The normalized source of a part: its tokens written back without spaces.
fn key_of(toks: &[Tok]) -> String {
    let mut s = String::new();
    for t in toks {
        match t {
            Tok::Cmd(c) => {
                s.push('\\');
                s.push_str(c);
                if c.chars().all(|c| c.is_ascii_alphabetic()) {
                    s.push(' ');
                }
            }
            Tok::Ch(c) => s.push(*c),
            Tok::Open => s.push('{'),
            Tok::Close => s.push('}'),
            Tok::Sup => s.push('^'),
            Tok::Sub => s.push('_'),
            Tok::PartOpen => s.push_str("{{"),
            Tok::PartClose => s.push_str("}}"),
            Tok::Amp => s.push('&'),
            Tok::Raw(r) => {
                s.push('{');
                s.push_str(r);
                s.push('}');
            }
        }
    }
    s.trim().to_string()
}

/// The normalized form `key_of` would give `src`, so authors can name parts by their TeX.
pub fn normalize_key(src: &str) -> String {
    key_of(&lex(src))
}

// ═══════════════════════════════════════════════════════════════════════════
// Layout
// ═══════════════════════════════════════════════════════════════════════════

/// A laid-out box, em units, y down from the baseline.
#[derive(Clone, Debug, Default)]
struct MBox {
    w: f32,               // Advance width.
    h: f32,               // Height above the baseline.
    d: f32,               // Depth below the baseline.
    items: Vec<Prim>,     // Shapes, relative to the box origin.
    italic: f32,          // Italic correction of the last glyph.
    single: Option<u16>,  // The math glyph when the box is one glyph.
    class: Option<Class>, // Atom class for spacing.
    align: Option<f32>,   // x of the first `&`.
    rel: Option<f32>,     // x of the first relation.
}

impl MBox {
    fn shift(&mut self, dx: f32, dy: f32) {
        for p in &mut self.items {
            p.x += dx;
            p.y += dy;
        }
    }

    /// Append `other` with its origin at (x, y-up offset `up`).
    fn put(&mut self, mut other: MBox, x: f32, up: f32) {
        other.shift(x, -up);
        self.items.extend(other.items);
    }

    fn tag(&mut self, color: Option<Rgba>, part: Option<u16>) {
        for p in &mut self.items {
            if p.color.is_none() {
                p.color = color;
            }
            if p.part.is_none() {
                p.part = part;
            }
        }
    }
}

/// The fonts layout reads from.
struct Ctx<'a> {
    f: &'a Fonts,
}

/// A filled rule with its top `top_up` above the baseline.
fn rect(x: f32, top_up: f32, w: f32, h: f32) -> Prim {
    Prim {
        shape: Shape::Rect {
            w,
            h,
            radius: 0.0,
            fill: 1.0,
            stroke: 0.0,
        },
        x,
        y: -top_up,
        color: None,
        part: None,
    }
}

/// The math-alphabet character of `c` in variant `v`.
fn map_char(c: char, v: Variant) -> char {
    let off = |base: u32, from: char| char::from_u32(base + (c as u32 - from as u32)).unwrap_or(c);
    match v {
        Variant::Italic => match c {
            'h' => 'ℎ',
            'a'..='z' => off(0x1D44E, 'a'),
            'A'..='Z' => off(0x1D434, 'A'),
            'α'..='ω' => off(0x1D6FC, 'α'),
            'ϵ' => '𝜖',
            'ϑ' => '𝜗',
            'ϕ' => '𝜙',
            'ϖ' => '𝜛',
            'ϱ' => '𝜚',
            '∂' => '𝜕',
            _ => c,
        },
        Variant::Bold => match c {
            'a'..='z' => off(0x1D41A, 'a'),
            'A'..='Z' => off(0x1D400, 'A'),
            '0'..='9' => off(0x1D7CE, '0'),
            'Α'..='Ω' => off(0x1D6A8, 'Α'),
            'α'..='ω' => off(0x1D6C2, 'α'),
            _ => c,
        },
        Variant::BoldItalic => match c {
            'a'..='z' => off(0x1D482, 'a'),
            'A'..='Z' => off(0x1D468, 'A'),
            'α'..='ω' => off(0x1D736, 'α'),
            _ => c,
        },
        Variant::Upright => c,
    }
}

/// The atom class of a character.
fn class_of(c: char) -> Class {
    match c {
        '+' | '−' | '×' | '⋅' | '±' | '∓' | '∗' | '∘' | '∧' | '∨' | '∩' | '∪' | '⊗' | '⊕' | '÷'
        | '∙' | '⋆' => Class::Bin,
        '=' | '<' | '>' | '≈' | '≠' | '≤' | '≥' | '→' | '←' | '↦' | '⇒' | '⇐' | '⇔' | '⟹' | '⟺'
        | '⟶' | '≡' | '∼' | '≃' | '∝' | ':' | '∈' | '∣' | '⊥' | '∥' => Class::Rel,
        '(' | '[' | '{' | '⟨' => Class::Open,
        ')' | ']' | '}' | '⟩' | '!' | '?' => Class::Close,
        ',' | ';' => Class::Punct,
        '⋯' | '…' => Class::Inner,
        _ => Class::Ord,
    }
}

/// TeX's inter-atom space in mu: 0 none, 1 thin, 2 medium, 3 thick; negative only outside scripts.
fn spacing(a: Class, b: Class) -> i8 {
    use Class::*;
    let row: [i8; 8] = match a {
        Ord => [0, 1, -2, -3, 0, 0, 0, -1],
        Op => [1, 1, 0, -3, 0, 0, 0, -1],
        Bin => [-2, -2, 0, 0, -2, 0, 0, -2],
        Rel => [-3, -3, 0, 0, -3, 0, 0, -3],
        Open => [0; 8],
        Close => [0, 1, -2, -3, 0, 0, 0, -1],
        Punct => [-1, -1, 0, -1, -1, -1, -1, -1],
        Inner => [-1, 1, -2, -3, -1, 0, -1, -1],
    };
    let i = match b {
        Ord => 0,
        Op => 1,
        Bin => 2,
        Rel => 3,
        Open => 4,
        Close => 5,
        Punct => 6,
        Inner => 7,
    };
    row[i]
}

impl Ctx<'_> {
    fn glyph(&self, font: FontId, gid: u16, s: f32) -> MBox {
        let m = self.f.metrics(font, gid);
        MBox {
            w: m.advance * s,
            h: m.height.max(0.0) * s,
            d: m.depth.max(0.0) * s,
            items: vec![Prim {
                shape: Shape::Glyph { font, gid, size: s },
                x: 0.0,
                y: 0.0,
                color: None,
                part: None,
            }],
            italic: m.italic * s,
            single: if font == MATH { Some(gid) } else { None },
            class: None,
            align: None,
            rel: None,
        }
    }

    fn char_box(&self, c: char, v: Variant, st: Style) -> MBox {
        let s = st.scale(self.f);
        let c = if c == '-' {
            '−'
        } else if c == '*' {
            '∗'
        } else {
            c
        };
        let mapped = map_char(c, v);
        let found = self.f.glyph(MATH, mapped).or_else(|| self.f.glyph(MATH, c));
        let mut b = match found {
            Some((font, gid)) => self.glyph(font, gid, s),
            None => MBox {
                w: 0.5 * s,
                ..MBox::default()
            },
        };
        b.class = Some(class_of(c));
        b
    }

    fn text_box(&self, text: &str, bold: bool, st: Style) -> MBox {
        let s = st.scale(self.f);
        let font = if bold { BOLD } else { TEXT };
        let mut b = MBox {
            class: Some(Class::Ord),
            ..MBox::default()
        };
        let mut x = 0.0;
        for c in text.chars() {
            if c == ' ' {
                x += self
                    .f
                    .glyph(font, ' ')
                    .map_or(0.333, |(f, g)| self.f.metrics(f, g).advance)
                    * s;
                continue;
            }
            if let Some((f, g)) = self.f.glyph(font, c) {
                let gb = self.glyph(f, g, s);
                b.h = b.h.max(gb.h);
                b.d = b.d.max(gb.d);
                let w = gb.w;
                b.put(gb, x, 0.0);
                x += w;
            }
        }
        b.w = x;
        b
    }

    fn list(&self, nodes: &[Node], mut st: Style) -> MBox {
        let mut out = MBox::default();
        let mut x = 0.0f32;
        let mut prev: Option<Class> = None;
        let mut colors: Vec<Rgba> = Vec::new();
        let mut parts: Vec<u16> = Vec::new();
        // lay out every atom first so binary operators can be reclassified with their neighbours
        let mut atoms: Vec<(MBox, Option<Rgba>, Option<u16>)> = Vec::new();
        let mut aligns: Vec<usize> = Vec::new();
        for n in nodes {
            match n {
                Node::ColorPush(c) => colors.push(*c),
                Node::ColorPop => {
                    colors.pop();
                }
                Node::PartPush(p) => parts.push(*p),
                Node::PartPop => {
                    parts.pop();
                }
                Node::StyleSet(s) => st = *s,
                Node::Align => aligns.push(atoms.len()),
                Node::Group(inner)
                    if inner.iter().any(marker) && !inner.is_empty() && is_transparent(inner) =>
                {
                    // a part or colour group: lay out its atoms as members of this list
                    let mut flat = Vec::new();
                    flatten(inner, &mut flat);
                    let sub =
                        self.atoms(&flat, st, &mut colors, &mut parts, &mut aligns, atoms.len());
                    atoms.extend(sub);
                }
                n => {
                    let b = self.node(n, st);
                    atoms.push((b, colors.last().copied(), parts.last().copied()));
                }
            }
        }
        let n = atoms.len();
        for i in 0..n {
            if atoms[i].0.class == Some(Class::Bin) {
                let before = (0..i).rev().find_map(|j| atoms[j].0.class);
                let after = (i + 1..n).find_map(|j| atoms[j].0.class);
                let lone = matches!(
                    before,
                    None | Some(Class::Bin | Class::Op | Class::Rel | Class::Open | Class::Punct)
                ) || matches!(after, Some(Class::Rel | Class::Close | Class::Punct));
                if lone {
                    atoms[i].0.class = Some(Class::Ord);
                }
            }
        }
        let mu = st.scale(self.f) / 18.0;
        for (i, (mut b, color, part)) in atoms.into_iter().enumerate() {
            if let (Some(p), Some(c)) = (prev, b.class) {
                let sp = spacing(p, c);
                if sp > 0 || (sp < 0 && !st.script()) {
                    x += match sp.abs() {
                        1 => 3.0,
                        2 => 4.0,
                        _ => 5.0,
                    } * mu;
                }
            }
            if aligns.first() == Some(&i) && out.align.is_none() {
                out.align = Some(x);
            }
            if b.class == Some(Class::Rel) && out.rel.is_none() {
                out.rel = Some(x);
            }
            if b.class.is_some() {
                prev = b.class;
            }
            b.tag(color, part);
            out.h = out.h.max(b.h);
            out.d = out.d.max(b.d);
            let w = b.w;
            let italic = b.italic;
            out.put(b, x, 0.0);
            x += w;
            out.italic = italic;
        }
        if aligns.first().is_some_and(|&i| i >= n) && out.align.is_none() {
            out.align = Some(x);
        }
        out.w = x;
        if n == 1 && out.items.len() == 1 {
            out.single = match out.items[0].shape {
                Shape::Glyph {
                    font: MATH, gid, ..
                } => Some(gid),
                _ => None,
            };
        }
        out
    }

    fn atoms(
        &self,
        nodes: &[Node],
        mut st: Style,
        colors: &mut Vec<Rgba>,
        parts: &mut Vec<u16>,
        aligns: &mut Vec<usize>,
        base: usize,
    ) -> Vec<(MBox, Option<Rgba>, Option<u16>)> {
        let mut out = Vec::new();
        for n in nodes {
            match n {
                Node::ColorPush(c) => colors.push(*c),
                Node::ColorPop => {
                    colors.pop();
                }
                Node::PartPush(p) => parts.push(*p),
                Node::PartPop => {
                    parts.pop();
                }
                Node::StyleSet(s) => st = *s,
                Node::Align => aligns.push(base + out.len()),
                n => {
                    let b = self.node(n, st);
                    out.push((b, colors.last().copied(), parts.last().copied()));
                }
            }
        }
        out
    }

    fn node(&self, n: &Node, st: Style) -> MBox {
        let f = self.f;
        let c = &f.c;
        let s = st.scale(f);
        match n {
            Node::Char(ch, v) => self.char_box(*ch, *v, st),
            Node::Group(list) => {
                let mut b = self.list(list, st);
                b.class = Some(Class::Ord);
                b
            }
            Node::Text(t, bold) => self.text_box(t, *bold, st),
            Node::Space(em) => MBox {
                w: em * s,
                ..MBox::default()
            },
            Node::Scripts { base, sup, sub } => {
                self.scripts(base, sup.as_deref(), sub.as_deref(), st)
            }
            Node::Frac { num, den, style } => {
                let fs = style.unwrap_or(st);
                let display = fs.level == 0;
                let fsc = fs.scale(f);
                let nb = self.list(num, fs.num());
                let db = self.list(den, fs.den());
                let t = c.rule * fsc;
                let axis = c.axis * fsc;
                let mut u = if display { c.num_up_display } else { c.num_up } * fsc;
                let mut v = if display {
                    c.den_down_display
                } else {
                    c.den_down
                } * fsc;
                let gn = if display {
                    c.num_gap_display
                } else {
                    c.num_gap
                } * fsc;
                let gd = if display {
                    c.den_gap_display
                } else {
                    c.den_gap
                } * fsc;
                u = u.max(axis + t / 2.0 + gn + nb.d);
                v = v.max(db.h + gd - (axis - t / 2.0));
                let pad = 0.12 * fsc;
                let w = nb.w.max(db.w);
                let mut b = MBox {
                    w: w + 2.0 * pad,
                    class: Some(Class::Inner),
                    ..MBox::default()
                };
                b.h = u + nb.h;
                b.d = v + db.d;
                let (nw, dw) = (nb.w, db.w);
                b.put(nb, pad + (w - nw) / 2.0, u);
                b.put(db, pad + (w - dw) / 2.0, -v);
                b.items.push(rect(pad, axis + t / 2.0, w, t));
                b
            }
            Node::Sqrt(body) => {
                let display = st.level == 0;
                let bb = self.list(body, st.cramp());
                let gap = if display {
                    c.radical_gap_display
                } else {
                    c.radical_gap
                } * s;
                let t = c.radical_rule * s;
                let need = bb.h + bb.d + gap + t;
                let rad = self.stretch_v('√', need, s, false);
                let top = bb.h + gap + t;
                let (rh, rw) = (rad.h, rad.w);
                let mut b = MBox {
                    class: Some(Class::Ord),
                    ..MBox::default()
                };
                let lift = top - rh;
                b.d = bb.d.max(rad.d - lift);
                b.h = top + c.radical_extra * s;
                b.put(rad, 0.0, lift);
                let bw = bb.w;
                b.put(bb, rw, 0.0);
                b.items.push(rect(rw - 0.02 * s, top, bw + 0.06 * s, t));
                b.w = rw + bw + 0.06 * s;
                b
            }
            Node::Delim { left, right, body } => {
                let bb = self.list(body, st);
                let axis = c.axis * s;
                let delta = (bb.h - axis).max(bb.d + axis);
                let size = (2.0 * delta * 0.901)
                    .max(2.0 * delta - 0.5 * s)
                    .max(c.delimited_min * s);
                let mut b = MBox {
                    class: Some(Class::Inner),
                    ..MBox::default()
                };
                let mut x = 0.0;
                let put = |b: &mut MBox, d: MBox, x: &mut f32| {
                    b.h = b.h.max(d.h);
                    b.d = b.d.max(d.d);
                    let w = d.w;
                    b.put(d, *x, 0.0);
                    *x += w;
                };
                if let Some(l) = left {
                    put(&mut b, self.stretch_v(*l, size, s, true), &mut x);
                } else {
                    x += 0.12 * s;
                }
                b.h = b.h.max(bb.h);
                b.d = b.d.max(bb.d);
                let bw = bb.w;
                b.put(bb, x, 0.0);
                x += bw;
                if let Some(r) = right {
                    put(&mut b, self.stretch_v(*r, size, s, true), &mut x);
                } else {
                    x += 0.12 * s;
                }
                b.w = x;
                b
            }
            Node::Big(ch, size, class) => {
                let mut b = self.stretch_v(*ch, size * s, s, true);
                // a bare `\big` takes its class from its delimiter, so `\big(` opens like `(`
                b.class = Some(if *class == Class::Inner {
                    class_of(*ch)
                } else {
                    *class
                });
                b
            }
            Node::OpGlyph(ch, _) => {
                let mut b = if st.level == 0 {
                    self.stretch_v(*ch, c.display_op_min * s, s, true)
                } else {
                    let mut b = self.char_box(*ch, Variant::Upright, st);
                    let axis = c.axis * s;
                    let lift = axis - (b.h - b.d) / 2.0;
                    b.shift(0.0, -lift);
                    b.h += lift;
                    b.d -= lift;
                    b
                };
                b.class = Some(Class::Op);
                b.single = None;
                b
            }
            Node::OpWord(name, _) => {
                let mut b = MBox {
                    class: Some(Class::Op),
                    ..MBox::default()
                };
                let mut x = 0.0;
                for ch in name.chars() {
                    let g = self.char_box(ch, Variant::Upright, st);
                    b.h = b.h.max(g.h);
                    b.d = b.d.max(g.d);
                    let w = g.w;
                    b.put(g, x, 0.0);
                    x += w;
                }
                b.w = x;
                b
            }
            Node::Accent(acc, body, wide) => {
                let bb = self.list(body, st.cramp());
                let mut b = MBox {
                    class: Some(Class::Ord),
                    ..MBox::default()
                };
                let attach = match bb.single {
                    Some(g) => f.accent_attach(g).map_or(bb.w / 2.0, |a| a * s),
                    None => bb.w / 2.0,
                };
                let acc_box = if *wide {
                    self.stretch_h(*acc, bb.w, s)
                } else {
                    f.glyph(MATH, *acc)
                        .map(|(fo, g)| self.glyph(fo, g, s))
                        .unwrap_or_default()
                };
                let am = match acc_box.items.first().map(|p| p.shape) {
                    Some(Shape::Glyph { font, gid, .. }) if !*wide => {
                        let m = f.metrics(font, gid);
                        f.accent_attach(gid)
                            .map_or((m.left + m.right) / 2.0 * s, |a| a * s)
                    }
                    _ => acc_box.w / 2.0,
                };
                let lift = (bb.h - c.accent_base * s).max(0.0);
                let top = acc_box.h + lift;
                b.w = bb.w;
                b.d = bb.d;
                b.h = bb.h.max(top);
                b.italic = bb.italic;
                b.put(bb, 0.0, 0.0);
                b.put(acc_box, attach - am, lift);
                b
            }
            Node::Overline(body) => {
                let bb = self.list(body, st.cramp());
                let t = c.overbar_rule * s;
                let top = bb.h + c.overbar_gap * s + t;
                let mut b = MBox {
                    w: bb.w,
                    h: top + c.overbar_extra * s,
                    d: bb.d,
                    class: Some(Class::Ord),
                    ..MBox::default()
                };
                b.items.push(rect(0.0, top, bb.w, t));
                b.put(bb, 0.0, 0.0);
                b
            }
            Node::Underline(body) => {
                let bb = self.list(body, st);
                let t = c.underbar_rule * s;
                let bottom = bb.d + c.underbar_gap * s + t;
                let mut b = MBox {
                    w: bb.w,
                    h: bb.h,
                    d: bottom + c.underbar_extra * s,
                    class: Some(Class::Ord),
                    ..MBox::default()
                };
                b.items.push(rect(0.0, -bottom + t, bb.w, t));
                b.put(bb, 0.0, 0.0);
                b
            }
            Node::Brace { body, label, under } => {
                let bb = self.list(body, st);
                let brace = self.stretch_h(if *under { '⏟' } else { '⏞' }, bb.w, s);
                let gap = 0.12 * s;
                let mut b = MBox {
                    w: bb.w,
                    h: bb.h,
                    d: bb.d,
                    class: Some(Class::Ord),
                    ..MBox::default()
                };
                let bw = brace.w;
                if *under {
                    let base = -(bb.d + gap + brace.h);
                    let bottom = -base + brace.d;
                    b.d = bottom;
                    b.put(brace, (bb.w - bw) / 2.0, base);
                    if let Some(l) = label {
                        let lb = self.list(l, st.sub());
                        let y = -(bottom + gap + lb.h);
                        b.d = -y + lb.d;
                        let lw = lb.w;
                        b.put(lb, (bb.w - lw) / 2.0, y);
                        b.w = b.w.max(lw);
                    }
                } else {
                    let base = bb.h + gap + brace.d;
                    let top = base + brace.h;
                    b.h = top;
                    b.put(brace, (bb.w - bw) / 2.0, base);
                    if let Some(l) = label {
                        let lb = self.list(l, st.sup());
                        let y = top + gap + lb.d;
                        b.h = y + lb.h;
                        let lw = lb.w;
                        b.put(lb, (bb.w - lw) / 2.0, y);
                        b.w = b.w.max(lw);
                    }
                }
                b.put(bb, 0.0, 0.0);
                b
            }
            Node::Cancel(body) => {
                let mut b = self.list(body, st);
                let pad = 0.06 * s;
                b.items.push(Prim {
                    shape: Shape::Stroke {
                        dx: b.w + 2.0 * pad,
                        dy: -(b.h + b.d + 2.0 * pad),
                        width: 0.055 * s,
                    },
                    x: -pad,
                    y: b.d + pad,
                    color: None,
                    part: None,
                });
                b.class = Some(Class::Ord);
                b.single = None;
                b
            }
            Node::XArrow(label, ch) => {
                let lb = self.list(label, st.sup());
                let w = (lb.w + 0.7 * s).max(1.6 * s);
                let arrow = self.stretch_h(*ch, w, s);
                let axis = c.axis * s;
                let mut b = MBox {
                    w: arrow.w,
                    class: Some(Class::Rel),
                    ..MBox::default()
                };
                let lift = axis - (arrow.h - arrow.d) / 2.0;
                let ah = arrow.h + lift;
                let aw = arrow.w;
                b.put(arrow, 0.0, lift);
                let y = ah + 0.1 * s + lb.d;
                b.h = y + lb.h;
                b.d = 0.0;
                let lw = lb.w;
                b.put(lb, (aw - lw) / 2.0, y);
                b
            }
            Node::Phantom(body) => {
                let mut b = self.list(body, st);
                b.items.clear();
                b
            }
            Node::Boxed(body) => {
                let bb = self.list(body, st);
                let pad = 0.25 * s;
                let mut b = MBox {
                    w: bb.w + 2.0 * pad,
                    h: bb.h + pad,
                    d: bb.d + pad,
                    class: Some(Class::Ord),
                    ..MBox::default()
                };
                b.items.push(Prim {
                    shape: Shape::Rect {
                        w: b.w,
                        h: b.h + b.d,
                        radius: 0.15 * s,
                        fill: 0.0,
                        stroke: 0.04 * s,
                    },
                    x: 0.0,
                    y: -b.h,
                    color: None,
                    part: None,
                });
                b.put(bb, pad, 0.0);
                b
            }
            _ => MBox::default(),
        }
    }

    fn scripts(&self, base: &Node, sup: Option<&Node>, sub: Option<&Node>, st: Style) -> MBox {
        let f = self.f;
        let c = &f.c;
        let s = st.scale(f);
        let limits =
            st.level == 0 && matches!(base, Node::OpGlyph(_, true) | Node::OpWord(_, true));
        let bb = self.node(base, st);
        if limits {
            let mut b = MBox {
                class: Some(Class::Op),
                ..MBox::default()
            };
            let up = sup.map(|n| self.node(n, st.sup()));
            let dn = sub.map(|n| self.node(n, st.sub()));
            let w =
                bb.w.max(up.as_ref().map_or(0.0, |u| u.w))
                    .max(dn.as_ref().map_or(0.0, |d| d.w));
            b.h = bb.h;
            b.d = bb.d;
            if let Some(u) = up {
                let y = bb.h + (c.upper_gap * s + u.d).max(c.upper_rise * s);
                b.h = y + u.h;
                let uw = u.w;
                b.put(u, (w - uw) / 2.0, y);
            }
            if let Some(d) = dn {
                let y = bb.d + (c.lower_gap * s + d.h).max(c.lower_drop * s);
                b.d = y + d.d;
                let dw = d.w;
                b.put(d, (w - dw) / 2.0, -y);
            }
            let bw = bb.w;
            b.put(bb, (w - bw) / 2.0, 0.0);
            b.w = w;
            return b;
        }
        let up = sup.map(|n| self.node(n, st.sup()));
        let dn = sub.map(|n| self.node(n, st.sub()));
        let single = bb.single.is_some();
        let (mut u, mut v) = if single {
            (0.0, 0.0)
        } else {
            let ss = st.sup().scale(f);
            (bb.h - c.sup_drop_max * ss, bb.d + c.sub_drop_min * ss)
        };
        let italic = bb.italic;
        let mut b = MBox {
            class: bb.class,
            ..MBox::default()
        };
        b.h = bb.h;
        b.d = bb.d;
        let bw = bb.w;
        b.put(bb, 0.0, 0.0);
        let after = c.space_after_script * s;
        match (up, dn) {
            (Some(p), None) => {
                let shift = if st.cramped {
                    c.sup_up_cramped
                } else {
                    c.sup_up
                } * s;
                u = u.max(shift).max(p.d + c.sup_bottom_min * s);
                b.h = b.h.max(u + p.h);
                let pw = p.w;
                b.put(p, bw + italic, u);
                b.w = bw + italic + pw + after;
            }
            (None, Some(q)) => {
                v = v.max(c.sub_down * s).max(q.h - c.sub_top_max * s);
                b.d = b.d.max(v + q.d);
                let qw = q.w;
                b.put(q, bw, -v);
                b.w = bw + qw + after;
            }
            (Some(p), Some(q)) => {
                let shift = if st.cramped {
                    c.sup_up_cramped
                } else {
                    c.sup_up
                } * s;
                u = u.max(shift).max(p.d + c.sup_bottom_min * s);
                v = v.max(c.sub_down * s);
                let gap = (u - p.d) - (q.h - v);
                if gap < c.sub_sup_gap * s {
                    v += c.sub_sup_gap * s - gap;
                    let psi = c.sup_bottom_max_with_sub * s - (u - p.d);
                    if psi > 0.0 {
                        u += psi;
                        v -= psi;
                    }
                }
                b.h = b.h.max(u + p.h);
                b.d = b.d.max(v + q.d);
                let (pw, qw) = (p.w, q.w);
                b.put(p, bw + italic, u);
                b.put(q, bw, -v);
                b.w = bw + (italic + pw).max(qw) + after;
            }
            (None, None) => b.w = bw,
        }
        b
    }

    /// A delimiter or radical at least `size` tall, centered on the axis when `center`.
    fn stretch_v(&self, ch: char, size: f32, s: f32, center: bool) -> MBox {
        let f = self.f;
        let Some((_, gid)) = f.glyph(MATH, ch) else {
            return MBox::default();
        };
        let con = f.construction(gid, false);
        let mut b = self.pick(gid, &con, size, s, false);
        if center {
            let axis = f.c.axis * s;
            let lift = axis - (b.h - b.d) / 2.0;
            b.shift(0.0, -lift);
            b.h += lift;
            b.d -= lift;
        }
        b.class = Some(class_of(ch));
        b
    }

    /// A horizontal brace or arrow at least `size` wide, its ink starting at x = 0.
    fn stretch_h(&self, ch: char, size: f32, s: f32) -> MBox {
        let f = self.f;
        let Some((_, gid)) = f.glyph(MATH, ch) else {
            return MBox::default();
        };
        let con = f.construction(gid, true);
        let mut b = self.pick(gid, &con, size, s, true);
        // an assembly is already ink-aligned; a single variant is pulled to its ink box
        if let [
            Prim {
                shape: Shape::Glyph { font, gid, .. },
                ..
            },
        ] = b.items.as_slice()
        {
            let m = f.metrics(*font, *gid);
            b.shift(-m.left * s, 0.0);
            b.w = (m.right - m.left) * s;
        }
        b
    }

    fn pick(&self, gid: u16, con: &Construction, size: f32, s: f32, horizontal: bool) -> MBox {
        for &(g, adv) in &con.variants {
            if adv * s >= size {
                return self.glyph(MATH, g, s);
            }
        }
        if con.parts.is_empty() {
            return match con.variants.last() {
                Some(&(g, _)) => self.glyph(MATH, g, s),
                None => self.glyph(MATH, gid, s),
            };
        }
        self.assemble(&con.parts, size, s, horizontal)
    }

    /// Repeat the extenders of an assembly until it spans `size`, then shrink the overlaps to fit.
    fn assemble(&self, parts: &[Part], size: f32, s: f32, horizontal: bool) -> MBox {
        let f = self.f;
        let o = f.overlap * s;
        let mut seq: Vec<Part> = Vec::new();
        let mut reps = 0;
        loop {
            seq.clear();
            for p in parts {
                let n = if p.extender { reps } else { 1 };
                for _ in 0..n {
                    seq.push(*p);
                }
            }
            if span(&seq, s, o) >= size || reps > 64 {
                break;
            }
            reps += 1;
        }
        let mut excess = (span(&seq, s, o) - size).max(0.0);
        let room: Vec<f32> = seq
            .windows(2)
            .map(|w| (w[0].end.min(w[1].start) * s - o).max(0.0))
            .collect();
        let total_room: f32 = room.iter().sum();
        excess = excess.min(total_room);
        let mut b = MBox::default();
        let mut cursor = 0.0;
        for (i, p) in seq.iter().enumerate() {
            let m = f.metrics(MATH, p.gid);
            let mut g = self.glyph(MATH, p.gid, s);
            if horizontal {
                g.shift(cursor - m.left * s, 0.0);
                b.h = b.h.max(g.h);
                b.d = b.d.max(g.d);
            } else {
                g.shift(0.0, -(cursor + m.depth * s));
                b.w = b.w.max(g.w);
            }
            b.items.extend(g.items);
            if i + 1 < seq.len() {
                let extra = if total_room > 0.0 {
                    excess * room[i] / total_room
                } else {
                    0.0
                };
                cursor += p.advance * s - o - extra;
            } else {
                cursor += p.advance * s;
            }
        }
        if horizontal {
            b.w = cursor;
        } else {
            b.h = cursor;
            b.d = 0.0;
        }
        b
    }
}

/// The length of glyph parts laid end to end with overlap `o`.
fn span(seq: &[Part], s: f32, o: f32) -> f32 {
    seq.iter().map(|p| p.advance * s).sum::<f32>() - o * (seq.len().max(1) - 1) as f32
}

/// A part or colour group, whose children join the parent list.
fn is_transparent(inner: &[Node]) -> bool {
    matches!(inner.first(), Some(Node::PartPush(_) | Node::ColorPush(_)))
}

/// The children of nested transparent groups, in order.
fn flatten(inner: &[Node], out: &mut Vec<Node>) {
    for n in inner {
        match n {
            Node::Group(g) if is_transparent(g) => flatten(g, out),
            other => out.push(other.clone()),
        }
    }
}

/// The formula of a laid-out box.
fn finish(b: MBox, parts: Vec<String>) -> Expr {
    Expr {
        prims: b.items,
        parts,
        w: b.w,
        h: b.h,
        d: b.d,
        align: b.align.or(b.rel),
        unknown: Vec::new(),
    }
}

/// Parse TeX into nodes, with its part keys and unknown commands.
fn parse(src: &str, palette: &HashMap<String, Rgba>) -> (Vec<Node>, Vec<String>, Vec<String>) {
    let mut p = Parser {
        toks: lex(src),
        pos: 0,
        palette,
        parts: Vec::new(),
        unknown: Vec::new(),
    };
    let mut nodes = Vec::new();
    while p.pos < p.toks.len() {
        nodes.extend(p.list(Variant::Italic));
        p.pos += 1;
    }
    (nodes, p.parts, p.unknown)
}

// --8<-- [start:typeset]
/// Lay out a TeX formula, display style unless `inline`.
pub fn typeset(f: &Fonts, src: &str, palette: &HashMap<String, Rgba>, inline: bool) -> Expr {
    let (nodes, parts, unknown) = parse(src, palette);
    let ctx = Ctx { f };
    let st = if inline { Style::TEXT } else { Style::DISPLAY };
    let b = ctx.list(&nodes, st);
    let mut e = finish(b, parts);
    e.unknown = unknown;
    e
}

// --8<-- [end:typeset]
/// Lay out prose with `$math$` and `**bold**`, wrapped at `width` em; `center` centers lines.
pub fn prose(
    f: &Fonts,
    src: &str,
    palette: &HashMap<String, Rgba>,
    width: f32,
    center: bool,
) -> Expr {
    let mut unknown: Vec<String> = Vec::new();
    let ctx = Ctx { f };
    // words: each a box, with whether a space precedes it
    let mut words: Vec<(MBox, bool)> = Vec::new();
    let mut bold = false;
    let mut space = false;
    let mut parts: Vec<String> = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = src.chars().collect();
    let mut word = String::new();
    let flush = |word: &mut String, words: &mut Vec<(MBox, bool)>, space: &mut bool, bold: bool| {
        if !word.is_empty() {
            words.push((ctx.text_box(word, bold, Style::TEXT), *space));
            word.clear();
            *space = false;
        }
    };
    while i < chars.len() {
        let c = chars[i];
        if c == '$' {
            flush(&mut word, &mut words, &mut space, bold);
            let end = chars[i + 1..]
                .iter()
                .position(|&c| c == '$')
                .map_or(chars.len(), |p| i + 1 + p);
            let tex: String = chars[i + 1..end].iter().collect();
            let (nodes, tex_parts, tex_unknown) = parse(&tex, palette);
            let offset = parts.len() as u16;
            unknown.extend(tex_unknown);
            parts.extend(tex_parts);
            let mut b = ctx.list(&nodes, Style::TEXT);
            for prim in &mut b.items {
                prim.part = prim.part.map(|id| id + offset);
            }
            words.push((b, space));
            space = false;
            i = end + 1;
            continue;
        }
        if c == '*' && chars.get(i + 1) == Some(&'*') {
            flush(&mut word, &mut words, &mut space, bold);
            bold = !bold;
            i += 2;
            continue;
        }
        if c.is_whitespace() {
            flush(&mut word, &mut words, &mut space, bold);
            space = true;
            i += 1;
            continue;
        }
        word.push(c);
        i += 1;
    }
    flush(&mut word, &mut words, &mut space, bold);
    let gap = f
        .glyph(TEXT, ' ')
        .map_or(0.333, |(fo, g)| f.metrics(fo, g).advance);
    let mut lines: Vec<Vec<(MBox, f32)>> = vec![Vec::new()];
    let mut x = 0.0;
    for (b, sp) in words {
        let lead = if sp && x > 0.0 { gap } else { 0.0 };
        if x > 0.0 && x + lead + b.w > width {
            lines.push(Vec::new());
            x = 0.0;
        }
        let lead = if x > 0.0 && sp { gap } else { 0.0 };
        let w = b.w;
        lines.last_mut().unwrap().push((b, x + lead));
        x += lead + w;
    }
    let widths: Vec<f32> = lines
        .iter()
        .map(|l| l.last().map_or(0.0, |(b, at)| at + b.w))
        .collect();
    let full = widths.iter().copied().fold(0.0f32, f32::max);
    let mut out = MBox {
        w: full,
        ..MBox::default()
    };
    let mut baseline = 0.0f32;
    let mut prev_d = 0.0f32;
    for (li, line) in lines.into_iter().enumerate() {
        let h = line.iter().map(|(b, _)| b.h).fold(0.70f32, f32::max);
        let d = line.iter().map(|(b, _)| b.d).fold(0.22f32, f32::max);
        if li == 0 {
            out.h = h;
        } else {
            baseline += (prev_d + h + 0.28).max(1.3);
        }
        let dx = if center {
            (full - widths[li]) / 2.0
        } else {
            0.0
        };
        for (b, at) in line {
            out.put(b, dx + at, -baseline);
        }
        out.d = baseline + d;
        prev_d = d;
    }
    let mut e = finish(out, parts);
    e.unknown = unknown;
    e.align = None;
    e
}

/// Lay out program text in the monospace face: lines and indentation kept, `#` comments dimmed.
pub fn code(f: &Fonts, src: &str, palette: &HashMap<String, Rgba>) -> Expr {
    let ctx = Ctx { f };
    let glyph = |c: char| f.glyph(super::font::MONO, c);
    let advance = glyph('m').map_or(0.5, |(font, gid)| f.metrics(font, gid).advance);
    let line_height = 1.3;
    let mut out = MBox {
        h: 0.75,
        ..MBox::default()
    };
    let lines: Vec<&str> = src.trim_end().lines().collect();
    for (row, line) in lines.iter().enumerate() {
        let comment = line.find('#').unwrap_or(line.len());
        for (col, c) in line.chars().enumerate() {
            let Some((font, gid)) = glyph(c).filter(|_| c != ' ') else {
                continue;
            };
            let mut cell = ctx.glyph(font, gid, 1.0);
            if col >= line[..comment].chars().count() {
                cell.tag(palette.get("dim").copied(), None);
            }
            out.put(cell, col as f32 * advance, -(row as f32) * line_height);
        }
        out.w = out.w.max(line.chars().count() as f32 * advance);
    }
    out.d = lines.len().saturating_sub(1) as f32 * line_height + 0.25;
    let mut e = finish(out, Vec::new());
    e.align = None;
    e
}
// --8<-- [end:implementation]
