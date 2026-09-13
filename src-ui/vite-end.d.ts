/// <reference types="vite/client" />
/// <reference types="preact" />

declare module '*.css';
declare module '*.module.css' {
  const classes: { readonly [key: string]: string };
  export default classes;
}