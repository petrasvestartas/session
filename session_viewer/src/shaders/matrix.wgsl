// Matrices: one instance per matrix, one quad over all its cells; the fragment stage finds its cell, colours it from its
// value, draws the cell edges and writes the value in white.

// One matrix in the word table: a header of HEADER_WORDS words, then rows * cols values of VALUE_WORDS words, row by row;
// matches matrices.rs. A value is its f32 for the colour, then exactly: its whole part, and its thousandths with the sign
// in the top bit, so a large number keeps the decimals an f32 cannot hold.
const HEADER_WORDS: u32 = 16u;
const VALUE_WORDS: u32 = 3u;
const SIGN_BIT: u32 = 0x80000000u;
const ROWS: u32 = 0u; // row count
const COLS: u32 = 1u; // column count
const INSTANCE: u32 = 2u; // object row
const CELL: u32 = 3u; // edge length of a cell, object units, f32 bits
const LOW: u32 = 4u; // smallest non-zero entry, f32 bits
const HIGH: u32 = 5u; // largest non-zero entry, f32 bits
const LINE: u32 = 6u; // edge colour, packed rgba
const MIN: u32 = 8u; // colour of the smallest entry, packed rgba
const MAX: u32 = 9u; // colour of the largest entry, packed rgba
const ZERO: u32 = 10u; // colour of a zero entry, packed rgba
const TEXT: u32 = 11u; // colour of the values, packed rgba

@group(3) @binding(0) var<storage, read> words: array<u32>; // every matrix, block after block

// The two triangles of the matrix, 0..1 across: x right, y down.
const QUAD = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 0.0), vec2<f32>(1.0, 1.0),
    vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 1.0),
);

const TEXT_EM: f32 = 0.3; // value height over cell size
const DIGIT_W: f32 = 0.42; // seven-segment digit width, em
const DIGIT_H: f32 = 0.7; // digit height, em
const STROKE: f32 = 0.085; // segment thickness, em
const ADVANCE_DIGIT: f32 = 0.6; // advance of a digit, em
const ADVANCE_DOT: f32 = 0.28; // advance of the decimal point, em
const ADVANCE_MINUS: f32 = 0.56; // advance of the minus sign, em
const ADVANCE_ONE: f32 = 0.34; // advance of a 1, only its right strokes, em
const LINE_PX: f32 = 1.0; // edge width shared by two cells, px
const FIT: f32 = 0.9; // widest a value may be, over the cell
const CHARS: u32 = 16u; // characters a value may have
const MINUS: u32 = 10u; // character code of '-'
const DOT: u32 = 11u; // character code of '.'
// Segments a..g of each digit and of '-', bit 0 = a (top) .. bit 6 = g (middle).
const SEGMENTS = array<u32, 11>(63u, 6u, 91u, 79u, 102u, 109u, 125u, 7u, 127u, 111u, 64u);

// What the vertex shader hands the fragment shader.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) uv: vec2<f32>, // position in cells: x = column, y = row, from the top-left corner
    @location(1) @interpolate(flat) block: u32, // the matrix's first word
};

// A word of the block as a float.
fn float_at(block: u32, word: u32) -> f32 {
    return bitcast<f32>(words[block + word]);
}

// A vertex placed off screen, so nothing is drawn.
fn dead_matrix() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    return dead;
}

// Corner `vid` of the matrix whose block starts at word `block`, placed by its object row.
@vertex
fn vs_matrix(@builtin(vertex_index) vid: u32, @builtin(instance_index) block: u32) -> VsOut {
    let rows = words[block + ROWS];
    let cols = words[block + COLS];
    let id = words[block + INSTANCE];
    let inst = instances[id];

    if (rows == 0u || cols == 0u || (inst.flags & FLAG_HIDDEN) != 0u) {
        return dead_matrix();
    }

    let cell = float_at(block, CELL);
    let at = QUAD[vid] * vec2<f32>(f32(cols), f32(rows));
    // cell (0, 0) is centred on the object origin
    let local = vec3<f32>((at.x - 0.5) * cell, (0.5 - at.y) * cell, 0.0);
    let world = place(id, local);

    if (clip_active() && clip_cut(inst.flags, world)) {
        return dead_matrix();
    }

    var o: VsOut;
    o.pos = mvp * vec4<f32>(world, 1.0);
    o.uv = at;
    o.block = block;
    return o;
}

// The characters of a value, left to right: digits 0-9, MINUS, DOT; trailing zeros of the decimals dropped.
fn label(whole: u32, thousandths: u32, negative: bool, chars: ptr<function, array<u32, 16>>) -> u32 {
    // the fewest decimals that write the thousandths
    var decimals = 3u;
    var fraction = thousandths;

    if (fraction == 0u) {
        decimals = 0u;
    } else if (fraction % 100u == 0u) {
        decimals = 1u;
        fraction = fraction / 100u;
    } else if (fraction % 10u == 0u) {
        decimals = 2u;
        fraction = fraction / 10u;
    }

    var reversed: array<u32, 16>;
    var count = 0u;

    // the decimals from the last, then the point
    for (var d = 0u; d < decimals; d++) {
        reversed[count] = fraction % 10u;
        count++;
        fraction = fraction / 10u;
    }

    if (decimals > 0u) {
        reversed[count] = DOT;
        count++;
    }

    // the whole part from the last digit, at least one
    var n = whole;

    loop {
        reversed[count] = n % 10u;
        count++;
        n = n / 10u;

        if (n == 0u || count >= CHARS - 1u) {
            break;
        }
    }

    if (negative && (whole > 0u || thousandths > 0u)) {
        reversed[count] = MINUS;
        count++;
    }

    for (var i = 0u; i < count; i++) {
        (*chars)[i] = reversed[count - 1u - i];
    }

    return count;
}

// Advance of a character, em.
fn advance(code: u32) -> f32 {
    if (code == DOT) {
        return ADVANCE_DOT;
    }

    if (code == 1u) {
        return ADVANCE_ONE;
    }

    return select(ADVANCE_DIGIT, ADVANCE_MINUS, code == MINUS);
}

// Distance from p to the segment a-b.
fn segment_distance(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let ab = b - a;
    let t = clamp(dot(p - a, ab) / dot(ab, ab), 0.0, 1.0);
    return length(p - a - ab * t);
}

// Signed distance to a character drawn from its box's bottom-left corner, em; positive outside.
fn glyph_distance(code: u32, p: vec2<f32>) -> f32 {
    if (code == DOT) {
        return length(p - vec2<f32>(0.1, STROKE * 0.5)) - STROKE * 0.75;
    }

    let w = DIGIT_W;
    let h = DIGIT_H;
    let m = h * 0.5;
    // the seven segments, a (top) .. g (middle)
    let ends = array<vec4<f32>, 7>(
        vec4<f32>(0.0, h, w, h), vec4<f32>(w, m, w, h), vec4<f32>(w, 0.0, w, m), vec4<f32>(0.0, 0.0, w, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, m), vec4<f32>(0.0, m, 0.0, h), vec4<f32>(0.0, m, w, m),
    );
    let lit = SEGMENTS[min(code, 10u)];
    var d = 1e6;

    for (var s = 0u; s < 7u; s++) {
        if ((lit & (1u << s)) != 0u) {
            d = min(d, segment_distance(p, ends[s].xy, ends[s].zw));
        }
    }

    return d - STROKE * 0.5;
}

// How much of this pixel a value written centred in its cell covers, 0..1; `uv` is 0..1 across the cell, `cell_px` the cell in pixels.
fn value_coverage(whole: u32, signed: u32, uv: vec2<f32>, cell_px: f32) -> f32 {
    var chars: array<u32, 16>;
    let count = label(whole, signed & ~SIGN_BIT, (signed & SIGN_BIT) != 0u, &chars);
    var width = 0.0;

    for (var i = 0u; i < count; i++) {
        width += advance(chars[i]);
    }

    // one em over the cell: TEXT_EM, smaller when a long value would not fit
    let em = min(TEXT_EM, FIT / width);
    // the text box, centred on the cell, in em from its bottom-left corner
    let p = vec2<f32>((uv.x - 0.5) / em + width * 0.5, (0.5 - uv.y) / em + DIGIT_H * 0.5);
    let em_px = em * cell_px;
    var x = 0.0;
    var d = 1e6;

    for (var i = 0u; i < count; i++) {
        let step = advance(chars[i]);
        // the glyph centred in its advance; a 1 is only its right strokes, at x = DIGIT_W
        var left = x + (step - select(DIGIT_W, 0.2, chars[i] == DOT)) * 0.5;

        if (chars[i] == 1u) {
            left = x + step * 0.5 - DIGIT_W;
        }

        d = min(d, glyph_distance(chars[i], p - vec2<f32>(left, 0.0)));
        x += step;
    }

    return clamp(0.5 - d * em_px, 0.0, 1.0);
}

// The colour at `uv` (in cells): the cell's fill from its value, its value over it, the edges over both.
fn matrix_color(block: u32, uv: vec2<f32>, px: vec2<f32>, selected: bool) -> vec3<f32> {
    let rows = words[block + ROWS];
    let cols = words[block + COLS];
    let col = min(u32(uv.x), cols - 1u);
    let row = min(u32(uv.y), rows - 1u);
    let inside = uv - vec2<f32>(f32(col), f32(row)); // 0..1 across the cell
    let at = block + HEADER_WORDS + (row * cols + col) * VALUE_WORDS;
    let value = bitcast<f32>(words[at]);
    let low = float_at(block, LOW);
    let high = float_at(block, HIGH);
    let t = select(1.0, clamp((value - low) / (high - low), 0.0, 1.0), high > low);
    var color = mix(unpack4x8unorm(words[block + MIN]).rgb, unpack4x8unorm(words[block + MAX]).rgb, t);

    if (abs(value) < 1e-12) {
        color = unpack4x8unorm(words[block + ZERO]).rgb;
    } else {
        color = mix(color, unpack4x8unorm(words[block + TEXT]).rgb, value_coverage(words[at + 1u], words[at + 2u], inside, px.y));
    }

    // the edges, half of each in the two cells it parts
    let edge = min(min(inside.x, 1.0 - inside.x) * px.x, min(inside.y, 1.0 - inside.y) * px.y);
    let pen = select(unpack4x8unorm(words[block + LINE]).rgb, SELECT_COLOR, selected);
    return mix(color, pen, clamp(LINE_PX * 0.5 + 0.5 - edge, 0.0, 1.0));
}

// True where something nearer than the matrix was drawn: depth is reversed, nearer is larger.
fn matrix_hidden(pos: vec4<f32>, sample: u32) -> bool {
    let z = ink_depth(pos.xy, sample);
    return z > pos.z + ink_tolerance(pos.z, 0.0, 0.0) + 1e-6;
}

@fragment
// Color: the matrix, behind solids that hide it.
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
    // pixels per cell, taken first: fwidth needs uniform control flow
    let px = 1.0 / max(fwidth(in.uv), vec2<f32>(1e-6));

    if (matrix_hidden(in.pos, sample)) {
        discard;
    }

    let selected = (instances[words[in.block + INSTANCE]].flags & FLAG_SELECTED) != 0u;
    return InkColor(vec4<f32>(matrix_color(in.block, in.uv, px, selected), 1.0));
}

@fragment
// Pick id: object row + 1.
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (matrix_hidden(in.pos, 0u)) {
        discard;
    }

    return vec2<u32>(words[in.block + INSTANCE] + 1u, 0u);
}
