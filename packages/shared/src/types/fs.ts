export interface WriteFilesOptions {
    root: string;
    overwrite?: boolean;
}

export interface WrittenFile {
    path: string;
    absolutePath: string;
}
