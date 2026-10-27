export function toPosixPath(path: string): string {
    return path.replace(/\\/g, '/');
}

export function ensureTrailingSlash(path: string): string {
    return path.endsWith('/') ? path : `${path}/`;
}

export function joinPosix(...parts: string[]): string {
    return parts
        .filter(part => part.length > 0)
        .map((part, index) =>
            index === 0 ? part.replace(/\/+$/g, '') : part.replace(/^\/+|\/+$/g, '')
        )
        .join('/');
}

export function stripExtension(fileName: string): string {
    const index = fileName.lastIndexOf('.');
    return index > 0 ? fileName.slice(0, index) : fileName;
}

export function baseName(filePath: string): string {
    const normalized = toPosixPath(filePath);
    const index = normalized.lastIndexOf('/');
    return index >= 0 ? normalized.slice(index + 1) : normalized;
}

export function isWindowsAbsolute(path: string): boolean {
    return /^[a-zA-Z]:[\\/]/.test(path) || path.startsWith('\\\\');
}
