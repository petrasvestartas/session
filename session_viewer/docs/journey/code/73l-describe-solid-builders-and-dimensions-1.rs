        wires
    }

    /// A number, or a click's distance from the origin in the plane.
    pub fn size(&self, answer: &Answer) -> f64 {
        match answer {
            Answer::Number(value) => *value,
            Answer::Point(p) => {
                let [u, v, _] = self.local(p);
                u.hypot(v)
            }
        }
    }

    /// A number, or a click's distance from the origin.
    pub fn distance(&self, answer: &Answer) -> f64 {
        match answer {
            Answer::Number(value) => *value,
            Answer::Point(p) => p.distance(&self.origin, None),
        }
    }

    /// A number, or a click's height along the normal.
    pub fn height(&self, answer: &Answer) -> f64 {
        match answer {
            Answer::Number(value) => *value,
            Answer::Point(p) => self.local(p)[2],
        }
    }
}

/// What the answers say so far: the placed frame and the sizes known.
#[derive(Clone, Debug)]
pub struct Part {
    pub frame: Frame,    // where the shape sits
    pub sizes: Vec<f64>, // its dimensions in answer order
}

impl Part {
    /// No sizes yet.
    pub fn new(frame: Frame) -> Self {
        Self {
            frame,
            sizes: Vec::new(),
        }
    }
}

/// A creation command: its questions, preview and kernel object.
pub struct Shape {
    pub name: &'static str,                               // shown name, e.g. `Box`
    pub options: &'static [(&'static str, &'static str)], // buttons: (label, line)
    pub upfront: bool, // the option changes the questions: chosen before any answer
    pub ask: fn(&[Answer], &Part, &str) -> Option<Ask>, // the next question, None when all are answered
    pub read: fn(&Frame, &[Answer], &str) -> Result<Part, String>, // what the answers say; an error refuses the last one
    pub outline: fn(&Part) -> Vec<Vec<Point>>,                     // preview wires, cheap
    pub build: fn(&Part, &str) -> Result<Geometry, String>, // the object for the chosen option
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl Shape {
    /// The option labels a word can pick, without Cancel.
    pub(crate) fn choices(&self) -> impl Iterator<Item = &'static str> {
        options::choices(self.options)
    }

    /// The option `words` start with and how many words spell it, ignoring case and spaces.
    pub(crate) fn option_in(&self, words: &[&str]) -> Option<(usize, usize)> {
        options::find(self.options, words)
    }
}

