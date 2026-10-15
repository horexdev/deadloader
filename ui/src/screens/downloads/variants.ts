// How D4 names a variant: by the folder it sits in inside the archive (authors usually name them so), else by its
// file name.

export function variantName(innerPath: string): string {
  const parts = innerPath.split(/[\\/]/).filter((p) => p !== '');
  const folder = parts.length > 1 ? parts.at(-2) : undefined;
  return folder ?? (parts.at(-1) ?? innerPath).replace(/\.vpk$/i, '');
}

/** The last part of a path inside an archive. */
export function fileName(innerPath: string): string {
  return innerPath.split(/[\\/]/).filter((p) => p !== '').at(-1) ?? innerPath;
}
