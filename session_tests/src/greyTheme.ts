// Greyscale Shiki theme, shared by the runtime highlighter and the build-time course renderer.
// Four shades only: text, comments, literals and keywords; the build maps each to a short class.

export const TEXT = '#1a1a1a';
export const COMMENT = '#8a8a8a';
export const LITERAL = '#5c5c5c';
export const KEYWORD = '#000000';
export const THEME_NAME = 'session-grey';

export const greyTheme = {
  name: THEME_NAME,
  type: 'light' as const,
  colors: { 'editor.background': '#f6f6f6', 'editor.foreground': TEXT },
  tokenColors: [
    { settings: { foreground: TEXT } },
    { scope: ['comment', 'punctuation.definition.comment'], settings: { foreground: COMMENT, fontStyle: 'italic' } },
    { scope: ['string', 'constant.character', 'constant.numeric', 'constant.language', 'markup.inline.raw'], settings: { foreground: LITERAL } },
    {
      scope: ['keyword', 'storage', 'storage.type', 'storage.modifier', 'keyword.control', 'keyword.other', 'variable.language.self'],
      settings: { foreground: KEYWORD, fontStyle: 'bold' },
    },
    { scope: ['keyword.operator', 'punctuation'], settings: { foreground: TEXT, fontStyle: '' } },
  ],
};

/** Short class for a token colour: c comment, s literal, k keyword, '' plain text. */
export function tokenClass(color: string | undefined): string {
  switch ((color || '').toLowerCase()) {
    case COMMENT:
      return 'c';
    case LITERAL:
      return 's';
    case KEYWORD:
      return 'k';
    default:
      return '';
  }
}
