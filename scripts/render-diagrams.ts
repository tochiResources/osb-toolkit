#!/usr/bin/env -S tsx
import { execFileSync } from 'node:child_process';
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const root = process.cwd();
const dir = join(root, 'docs', 'architecture');
const cli = join(root, 'node_modules', '@mermaid-js', 'mermaid-cli', 'src', 'cli.js');

// mermaid-cli needs a Chromium. The repo ships no absolute browser path: detect
// one (or honour `PUPPETEER_EXECUTABLE_PATH`) and hand mermaid-cli a throwaway
// config in the OS temp dir, so nothing machine-specific is committed.
function browserExecutable(): string | undefined {
    if (process.env.PUPPETEER_EXECUTABLE_PATH) {
        return process.env.PUPPETEER_EXECUTABLE_PATH;
    }
    const candidates = [
        process.env.PROGRAMFILES &&
            join(process.env.PROGRAMFILES, 'Microsoft', 'Edge', 'Application', 'msedge.exe'),
        process.env['PROGRAMFILES(X86)'] &&
            join(
                process.env['PROGRAMFILES(X86)'],
                'Microsoft',
                'Edge',
                'Application',
                'msedge.exe'
            ),
        process.env.PROGRAMFILES &&
            join(process.env.PROGRAMFILES, 'Google', 'Chrome', 'Application', 'chrome.exe'),
        process.env.LOCALAPPDATA &&
            join(process.env.LOCALAPPDATA, 'Google', 'Chrome', 'Application', 'chrome.exe'),
        '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
        '/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge',
        '/usr/bin/google-chrome',
        '/usr/bin/chromium',
        '/usr/bin/chromium-browser',
    ].filter((candidate): candidate is string => typeof candidate === 'string');
    return candidates.find(candidate => existsSync(candidate));
}

let puppeteerConfig: string | undefined;
let puppeteerConfigDir: string | undefined;
const executablePath = browserExecutable();
if (executablePath) {
    puppeteerConfigDir = mkdtempSync(join(tmpdir(), 'osbtk-puppeteer-'));
    puppeteerConfig = join(puppeteerConfigDir, 'puppeteer-config.json');
    writeFileSync(
        puppeteerConfig,
        JSON.stringify({ executablePath, args: ['--no-sandbox', '--disable-gpu'] })
    );
}

// The light background matches the light docs theme; the dark background matches
// VitePress's default dark background.
const LIGHT_BG = 'white';
const DARK_BG = '#1b1b1f';

const files = readdirSync(dir).filter(file => file.endsWith('.mmd') && !file.endsWith('.dark.mmd'));
if (files.length === 0) {
    console.error('no .mmd sources in docs/architecture');
    process.exit(1);
}

function render(input: string, output: string, theme: string, background: string): void {
    execFileSync(
        process.execPath,
        [
            cli,
            ...(puppeteerConfig ? ['-p', puppeteerConfig] : []),
            '-i',
            input,
            '-o',
            output,
            '-t',
            theme,
            '--backgroundColor',
            background,
        ],
        { stdio: 'inherit' }
    );
}

// The `.mmd` sources hard-code a light `classDef` palette (pastel fills, dark
// text). For the dark variant we re-tint that palette - dark fills, brighter
// strokes and light text - keeping each category's hue so the color coding
// survives. Non-`classDef` elements (clusters, edges, labels, class diagrams)
// are handled by mermaid's `dark` theme.
function toDark(source: string): string {
    return source.replace(
        /\b(fill|stroke|color):#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})\b/g,
        (_match, property: string, hex: string) => {
            const [h, s] = hexToHsl(expand(hex));
            const lightness = property === 'fill' ? 0.2 : property === 'stroke' ? 0.6 : 0.93;
            return `${property}:${hslToHex(h, s, lightness)}`;
        }
    );
}

function expand(hex: string): string {
    return hex.length === 3
        ? hex
              .split('')
              .map(char => char + char)
              .join('')
        : hex;
}

function hexToHsl(hex: string): [number, number, number] {
    const r = parseInt(hex.slice(0, 2), 16) / 255;
    const g = parseInt(hex.slice(2, 4), 16) / 255;
    const b = parseInt(hex.slice(4, 6), 16) / 255;
    const max = Math.max(r, g, b);
    const min = Math.min(r, g, b);
    const l = (max + min) / 2;
    const delta = max - min;
    if (delta === 0) {
        return [0, 0, l];
    }
    const s = l > 0.5 ? delta / (2 - max - min) : delta / (max + min);
    let h: number;
    if (max === r) {
        h = ((g - b) / delta + (g < b ? 6 : 0)) / 6;
    } else if (max === g) {
        h = ((b - r) / delta + 2) / 6;
    } else {
        h = ((r - g) / delta + 4) / 6;
    }
    return [h, s, l];
}

function hslToHex(h: number, s: number, l: number): string {
    const channel = (n: number) => {
        const k = (n + h * 12) % 12;
        const a = s * Math.min(l, 1 - l);
        const v = l - a * Math.max(-1, Math.min(k - 3, 9 - k, 1));
        return Math.round(v * 255)
            .toString(16)
            .padStart(2, '0');
    };
    return `#${channel(0)}${channel(8)}${channel(4)}`;
}

for (const file of files) {
    const input = join(dir, file);
    const lightOutput = input.replace(/\.mmd$/, '.svg');
    const darkOutput = input.replace(/\.mmd$/, '.dark.svg');
    const darkSource = input.replace(/\.mmd$/, '.dark.mmd');

    render(input, lightOutput, 'default', LIGHT_BG);

    writeFileSync(darkSource, toDark(readFileSync(input, 'utf8')), 'utf8');
    try {
        render(darkSource, darkOutput, 'dark', DARK_BG);
    } finally {
        rmSync(darkSource, { force: true });
    }

    console.log(`rendered ${file} -> ${lightOutput} (+ ${darkOutput})`);
}

if (puppeteerConfigDir) {
    rmSync(puppeteerConfigDir, { recursive: true, force: true });
}
