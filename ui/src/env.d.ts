interface ImportMetaEnv {
  /** Only `.env.mock` sets it: where the browser preview loads catalog pictures from. */
  readonly VITE_IMAGE_ORIGIN?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
