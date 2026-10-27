/**
 * Installer generation for the converter's build pipeline (M17b): re-exports
 * the `@tochii/installer` programmatic API so callers can turn built plug-in
 * artifacts into a distributable installer.
 *
 * Node-only; the browser entry (`@tochii/osb-toolkit-core`) does not import this.
 */
export * from '@tochii/installer';
