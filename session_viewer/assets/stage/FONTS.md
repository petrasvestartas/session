# Stage fonts

`NewCMMath-Book.otf`, `NewCM10-Book.otf`, `NewCM10-Bold.otf` and `NewCMMono10-Book.otf` are New Computer Modern 8.1.1
by Antonis Tsolomitis, from CTAN (`fonts/newcomputermodern/otf`). The Book weights are distributed
under the GUST Font License (https://www.gust.org.pl/projects/e-foundry/licenses). They are used
unchanged: the math font's OpenType MATH table drives the stage TeX layout, the two text fonts set
prose and `\text{}`.
`NewCMMono10-Book.otf` sets `code:` blocks.

The four files are subsets: Latin, Greek, punctuation, arrows, mathematical operators and the
mathematical alphabets, with every layout feature and the MATH table kept. Made with fontTools:

```bash
U="U+0020-007E,U+00A0-00FF,U+0131,U+0141-0142,U+0152-0153,U+0160-0161,U+0178,U+017D-017E,U+02C6-02DC,U+0300-036F,U+0391-03C9,U+03D1,U+03D5,U+03D6,U+03F1,U+03F5,U+2010-2027,U+2030-203A,U+2044,U+20D0-20DC,U+2102,U+210E,U+2113,U+2115,U+2119-211D,U+2124,U+2190-21FF,U+2200-22FF,U+2300-23FF,U+25A0-25FF,U+27E8-27EF,U+27F5-27FF,U+2A00-2AFF,U+1D400-1D7FF"
python -m fontTools.subset <font from CTAN> --unicodes="$U" --layout-features='*' --no-hinting --desubroutinize --output-file=<same name>
```

A character outside these ranges needs the range added and the fonts made again.
