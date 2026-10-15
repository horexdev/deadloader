// The product name in UI text is set apart (decision 193): `Text` wraps it in `.dl-brand`. File names, paths and
// addresses that contain it (deadloader.exe, deadloader-data, D:\Games\deadloader, github.com/…/deadloader) stay plain.

export interface BrandPart {
  text: string;
  brand: boolean;
}

const NAME = /(?<![\w\\/.-])deadloader(?![\w-]|\.\w)/g;

/** `text` cut into the product name and the rest, in order; empty parts are left out. */
export function brandParts(text: string): BrandPart[] {
  const parts: BrandPart[] = [];
  let at = 0;
  for (const match of text.matchAll(NAME)) {
    if (match.index > at) {
      parts.push({ text: text.slice(at, match.index), brand: false });
    }
    parts.push({ text: match[0], brand: true });
    at = match.index + match[0].length;
  }
  if (at < text.length) {
    parts.push({ text: text.slice(at), brand: false });
  }
  return parts;
}
