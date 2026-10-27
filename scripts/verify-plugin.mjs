#!/usr/bin/env node
// Verifies that a built plug-in binary exports the symbols its host requires.
// For an AE/Premiere `.aex`: EffectMain + PluginDataEntryFunction2.
// For an OFX `.ofx`: OfxGetNumberOfPlugins + OfxGetPlugin + OfxSetHost.
// Reads the PE export table directly, so no Visual Studio tooling is needed.
import { readFileSync } from 'node:fs';

const REQUIRED = {
    aex: ['EffectMain', 'PluginDataEntryFunction2'],
    ofx: ['OfxGetNumberOfPlugins', 'OfxGetPlugin', 'OfxSetHost'],
};

function u16(buf, o) {
    return buf.readUInt16LE(o);
}
function u32(buf, o) {
    return buf.readUInt32LE(o);
}

function exportNames(buf) {
    const peOffset = u32(buf, 0x3c);
    if (buf.toString('ascii', peOffset, peOffset + 4) !== 'PE\0\0') {
        throw new Error('not a PE file');
    }
    const coff = peOffset + 4;
    const numberOfSections = u16(buf, coff + 2);
    const optionalSize = u16(buf, coff + 16);
    const optional = coff + 20;
    const magic = u16(buf, optional);
    const dataDir = optional + (magic === 0x20b ? 112 : 96);
    const exportRva = u32(buf, dataDir);
    if (exportRva === 0) return [];

    const sections = [];
    const sectionStart = optional + optionalSize;
    for (let i = 0; i < numberOfSections; i += 1) {
        const s = sectionStart + i * 40;
        sections.push({
            virtualAddress: u32(buf, s + 12),
            sizeOfRawData: u32(buf, s + 16),
            pointerToRawData: u32(buf, s + 20),
        });
    }
    const rvaToOffset = rva => {
        for (const section of sections) {
            if (
                rva >= section.virtualAddress &&
                rva < section.virtualAddress + section.sizeOfRawData
            ) {
                return rva - section.virtualAddress + section.pointerToRawData;
            }
        }
        return -1;
    };

    const exportOffset = rvaToOffset(exportRva);
    const numberOfNames = u32(buf, exportOffset + 24);
    const addressOfNames = rvaToOffset(u32(buf, exportOffset + 32));
    const names = [];
    for (let i = 0; i < numberOfNames; i += 1) {
        const nameRva = u32(buf, addressOfNames + i * 4);
        const nameOffset = rvaToOffset(nameRva);
        let end = nameOffset;
        while (buf[end] !== 0) end += 1;
        names.push(buf.toString('ascii', nameOffset, end));
    }
    return names;
}

const path = process.argv[2];
if (!path) {
    console.error('usage: node scripts/verify-plugin.mjs <plugin.dll|.aex|.ofx>');
    process.exit(1);
}

const names = exportNames(readFileSync(path));
const kinds = Object.entries(REQUIRED).filter(([, symbols]) =>
    symbols.every(symbol => names.includes(symbol))
);

if (kinds.length === 0) {
    const required = path.endsWith('.ofx') ? REQUIRED.ofx : REQUIRED.aex;
    const missing = required.filter(symbol => !names.includes(symbol));
    console.error(`FAIL ${path}: missing exports ${missing.join(', ')}`);
    process.exit(1);
}
console.log(`OK ${path}: exports ${kinds.map(([kind]) => kind).join(' + ')} entry points`);
