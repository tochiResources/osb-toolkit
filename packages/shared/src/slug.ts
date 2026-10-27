export function slugify(value: string): string {
    return value
        .normalize('NFKD')
        .replace(/[\u0300-\u036f]/g, '')
        .replace(/([a-z0-9])([A-Z])/g, '$1-$2')
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, '-')
        .replace(/^-+|-+$/g, '');
}

export function pascalCase(value: string): string {
    return value
        .replace(/[^a-zA-Z0-9]+/g, ' ')
        .split(' ')
        .filter(Boolean)
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join('');
}

export function camelCase(value: string): string {
    const pascal = pascalCase(value);
    return pascal.charAt(0).toLowerCase() + pascal.slice(1);
}

export function snakeUpper(value: string): string {
    return value
        .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
        .replace(/[^a-zA-Z0-9]+/g, '_')
        .toUpperCase()
        .replace(/^_+|_+$/g, '');
}

export function reverseDnsId(prefix: string, name: string): string {
    return `${prefix}.${slugify(name).replace(/-/g, '')}`;
}
