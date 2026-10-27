import { generateCSharp } from './csharp';
import { generateDocs } from './docs';
import { generateGo } from './go';
import { generateJava } from './java';
import { generatePython } from './python';
import { generateRust } from './rust';
import { writeSpec } from './shared';
import { generateTypeScript } from './typescript';

/** Regenerates the OpenAPI contract and every per-language SDK from it. */
export function generate(): void {
    writeSpec();
    generateCSharp();
    generateTypeScript();
    generatePython();
    generateGo();
    generateJava();
    generateRust();
    generateDocs();
}
