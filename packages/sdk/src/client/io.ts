import * as native from '@tochii/osb-toolkit-native';

/**
 * The file-operations library (`osb-core`): the toolkit's single filesystem API.
 * Through WebAssembly it probes the host with `environment()`; under
 * Node/Bun/Deno it reads and writes real files, while a browser reports
 * `browser` and throws unless a host is installed via `installGlobalHost()`.
 */
export interface IO {
    /** The detected host environment. */
    environment(): string;
    /** Installs a filesystem host from `globalThis.__osbtk_io`, if present. */
    installGlobalHost(): string;
    /** Reads a file into a byte array. */
    read(path: string): Uint8Array;
    /** Reads a file into a UTF-8 string. */
    readText(path: string): string;
    /** Writes bytes to a file. */
    write(path: string, bytes: Uint8Array): void;
    /** Writes a UTF-8 string to a file. */
    writeText(path: string, contents: string): void;
    /** Whether a path exists. */
    exists(path: string): boolean;
    /** Whether a path is a regular file. */
    isFile(path: string): boolean;
    /** Whether a path is a directory. */
    isDir(path: string): boolean;
    /** The size of a file in bytes. */
    fileSize(path: string): number;
    /** Lists a directory's entry names, sorted. */
    list(path: string): string[];
    /** Creates a directory and any missing parents. */
    createDirAll(path: string): void;
    /** Copies a file. */
    copy(from: string, to: string): void;
    /** Renames (or moves) a file. */
    rename(from: string, to: string): void;
    /** Removes a file. */
    removeFile(path: string): void;
    /** Removes a directory and everything inside it. */
    removeDirAll(path: string): void;
}

/** Builds the file-operations client over the native host bridge. */
export function createIO(): IO {
    return {
        environment: () => native.ioEnvironment(),
        installGlobalHost: () => native.ioInstallGlobalHost(),
        read: path => native.ioRead(path),
        readText: path => native.ioReadString(path),
        write: (path, bytes) => native.ioWrite(path, bytes),
        writeText: (path, contents) => native.ioWriteString(path, contents),
        exists: path => native.ioExists(path),
        isFile: path => native.ioIsFile(path),
        isDir: path => native.ioIsDir(path),
        fileSize: path => native.ioFileSize(path),
        list: path => native.ioList(path),
        createDirAll: path => native.ioCreateDirAll(path),
        copy: (from, to) => native.ioCopy(from, to),
        rename: (from, to) => native.ioRename(from, to),
        removeFile: path => native.ioRemoveFile(path),
        removeDirAll: path => native.ioRemoveDirAll(path),
    };
}
