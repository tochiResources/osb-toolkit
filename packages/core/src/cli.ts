#!/usr/bin/env node
import { readdir, readFile } from 'node:fs/promises';
import { basename, join, resolve } from 'node:path';
import { parseArgs } from 'node:util';

import { createLogger, isPluginTarget, type PluginTarget } from '@tochii/osb-toolkit-shared';
import { writeGeneratedFiles } from '@tochii/osb-toolkit-shared/fs';

import { convertEffectBundle } from './convert.js';
import type { ConvertResult, EffectSource } from './types/convert.js';

interface CliOptions {
    target: PluginTarget;
    out: string;
    name?: string;
    id?: string;
    category?: string;
    width?: number;
    height?: number;
    bundle: boolean;
    combineParameters: boolean;
    json: boolean;
    quiet: boolean;
}

async function main(): Promise<void> {
    const { values, positionals } = parseArgs({
        allowPositionals: true,
        options: {
            target: { type: 'string', short: 't' },
            out: { type: 'string', short: 'o', default: 'dist' },
            name: { type: 'string' },
            id: { type: 'string' },
            category: { type: 'string' },
            width: { type: 'string' },
            height: { type: 'string' },
            lib: { type: 'string', multiple: true },
            bundle: { type: 'boolean' },
            separate: { type: 'boolean' },
            'combine-parameters': { type: 'boolean' },
            json: { type: 'boolean' },
            quiet: { type: 'boolean', short: 'q' },
            help: { type: 'boolean', short: 'h' },
        },
    });

    if (values.help || positionals.length === 0) {
        printHelp();
        process.exit(values.help ? 0 : 1);
    }

    const target = values.target ?? 'after-effects';
    if (!isPluginTarget(target)) {
        console.error(`Unknown target: ${target}`);
        process.exit(1);
    }

    const options: CliOptions = {
        target,
        out: values.out ?? 'dist',
        name: values.name,
        id: values.id,
        category: values.category,
        width: values.width ? Number(values.width) : undefined,
        height: values.height ? Number(values.height) : undefined,
        bundle: Boolean(values.bundle) && !values.separate,
        combineParameters: Boolean(values['combine-parameters']),
        json: Boolean(values.json),
        quiet: Boolean(values.quiet),
    };

    const logger = createLogger({
        level: options.quiet ? 'warn' : 'info',
        scope: 'osbtk-fx-convert',
    });

    let failures = 0;
    try {
        const effects = await readSources(positionals);
        const libraries = await readLibraries(values.lib ?? []);
        const result = convertEffectBundle({
            target: options.target,
            effects,
            libraries,
            bundle: options.bundle,
            parameterGrouping: options.combineParameters ? 'combined' : 'effect',
            pluginName: options.name,
            pluginId: options.id,
            category: options.category,
            // Always lint the consumer's C# effect scripts before transpiling.
            lintCSharp: true,
            resolution:
                options.width && options.height
                    ? { width: options.width, height: options.height }
                    : undefined,
        });

        const outDir = resolve(options.out);
        const written: Array<{ plugin: ConvertResult; files: string[] }> = [];
        for (const plugin of result.plugins) {
            const files = await writeGeneratedFiles(plugin.files, { root: outDir });
            written.push({ plugin, files: files.map(file => file.path) });
        }

        if (options.json) {
            console.log(
                JSON.stringify(
                    {
                        target: result.target,
                        bundle: result.bundle,
                        outputDir: outDir,
                        plugins: written.map(entry => ({
                            effectName: entry.plugin.effectName,
                            pluginId: entry.plugin.pluginId,
                            files: entry.files,
                            diagnostics: entry.plugin.diagnostics,
                        })),
                        diagnostics: result.diagnostics,
                    },
                    null,
                    2
                )
            );
        } else {
            logger.info(
                `${effects.length} effect(s) → ${outDir} (${result.bundle ? 'bundled' : 'individual'})`
            );
            for (const entry of written) {
                logger.info(
                    `  ${entry.plugin.effectName}: ${entry.plugin.files.length} file(s), ${entry.plugin.diagnostics.length} diagnostic(s)`
                );
                for (const diagnostic of entry.plugin.diagnostics) {
                    logger.warn(`    [${diagnostic.severity}] ${diagnostic.message}`);
                }
            }
        }
    } catch (error) {
        failures += 1;
        logger.error('Conversion failed', error);
    }

    process.exit(failures > 0 ? 1 : 0);
}

async function readSources(inputs: string[]): Promise<EffectSource[]> {
    const sources: EffectSource[] = [];
    for (const input of inputs) {
        const path = resolve(input);
        sources.push({
            source: await readFile(path, 'utf8'),
            name: basename(path, '.cs'),
            file: input,
        });
    }
    return sources;
}

/** Reads library sources from files and/or directories (recursing for `*.cs`). */
async function readLibraries(inputs: string[]): Promise<EffectSource[]> {
    const sources: EffectSource[] = [];
    for (const input of inputs) {
        const path = resolve(input);
        const files = await collectCsFiles(path);
        for (const file of files) {
            sources.push({
                source: await readFile(file, 'utf8'),
                name: basename(file, '.cs'),
                file,
            });
        }
    }
    return sources;
}

/** Returns `path` if it is a `.cs` file, or every `.cs` file under it if a directory. */
async function collectCsFiles(path: string): Promise<string[]> {
    const entries = await readdir(path, { withFileTypes: true }).catch(() => null);
    if (!entries) return path.endsWith('.cs') ? [path] : [];
    const files: string[] = [];
    for (const entry of entries) {
        const child = join(path, entry.name);
        if (entry.isDirectory()) files.push(...(await collectCsFiles(child)));
        else if (entry.name.endsWith('.cs')) files.push(child);
    }
    return files;
}

function printHelp(): void {
    console.log(`osbtk-fx-convert - effect script converter

Usage:
  osbtk-fx-convert <Effect.cs...> [options]

Options:
  -t, --target <target>   after-effects | davinci-resolve | premiere-pro
  -o, --out <dir>         output directory (default: dist)
      --name <name>       override the plug-in name (bundle name when bundling)
      --id <id>           override the reverse-DNS plug-in id
      --category <name>   override the plug-in category
      --width <px>        composition width
      --height <px>       composition height
      --lib <path...>     external library file(s) or scriptslibrary folder(s)
      --bundle            bundle all effects into a single plug-in
      --separate          convert each effect into its own plug-in (default)
      --combine-parameters  merge bundled parameters by [Group] instead of per effect
      --json              machine-readable output
  -q, --quiet             only log warnings and errors
  -h, --help              show this help
`);
}

void main();
