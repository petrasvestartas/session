import type { Tokens } from 'marked';

// Keep SVG blank lines inside HTML; Marked still handles fenced and indented code first.
export function svgHtml(source: string): Tokens.HTML | false {
  const start = source.match(/^ {0,3}(<div\b(?:[^"'<>]|"[^"]*"|'[^']*')*>\s*)?(?=<svg(?:\s|\/?>))/i);
  if (!start) return false;
  const tags = /<!--[\s\S]*?-->|<!\[CDATA\[[\s\S]*?\]\]>|<\/?([a-z][\w:-]*)(?:[^"'<>]|"[^"]*"|'[^']*')*>/gi;
  tags.lastIndex = start[0].length;
  let depth = 0;
  for (let tag; (tag = tags.exec(source));) {
    if (tag[1]?.toLowerCase() !== 'svg') continue;
    if (tag[0].startsWith('</')) depth--;
    else if (!tag[0].endsWith('/>')) depth++;
    if (depth !== 0) continue;
    let end = tags.lastIndex;
    if (start[1]) {
      const close = source.slice(end).match(/^\s*<\/div\s*>/i);
      if (!close) return false;
      end += close[0].length;
    }
    const raw = source.slice(0, end);
    return { type: 'html', raw, text: raw, block: true, pre: false };
  }
  return false;
}
