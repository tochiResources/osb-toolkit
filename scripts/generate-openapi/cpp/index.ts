import { emitCppClean } from '../clean';

/**
 * Writes the clean C++ header from the OpenAPI contract. The C++ SDK has no
 * OpenAPI generator stage of its own, so `emitCppClean` is the source of truth
 * and `yarn openapi` owns the checked-in copy.
 */
export function generateCpp(): void {
    emitCppClean();
}
