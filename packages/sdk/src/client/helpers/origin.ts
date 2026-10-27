import type { OsbOrigin, Vector2, Vector3 } from '../../lib';
import type { Call } from './types';

/** The shared origin/anchor helpers. */
export interface OriginHelpers {
    /** The anchor offset for an enum origin, in texture pixels. */
    originVector(origin: OsbOrigin, width: number, height: number): Promise<Vector2>;
    /** The effective pivot: the enum origin vector plus an explicit anchor offset. */
    resolveAnchor(
        origin: OsbOrigin,
        anchor: Vector2,
        width: number,
        height: number
    ): Promise<Vector2>;
    /** The 3D effective pivot: a `Vector3` anchor whose `z` anchors depth. */
    resolveAnchor3d(
        origin: OsbOrigin,
        anchor: Vector3,
        width: number,
        height: number
    ): Promise<Vector3>;
    /** Whether a rotated sprite's bounds intersect the screen. */
    inScreenBounds(
        position: Vector2,
        size: Vector2,
        rotation: number,
        origin: Vector2
    ): Promise<boolean>;
}

/** Builds the shared origin/anchor helper client. */
export function createOriginHelpers(call: Call): OriginHelpers {
    return {
        originVector: (origin, width, height) =>
            call('origin.getOriginVector', { origin, width, height }),
        resolveAnchor: (origin, anchor, width, height) =>
            call('origin.resolveAnchor', { origin, anchor, width, height }),
        resolveAnchor3d: (origin, anchor, width, height) =>
            call('origin.resolveAnchor3d', { origin, anchor, width, height }),
        inScreenBounds: (position, size, rotation, origin) =>
            call('origin.inScreenBounds', { position, size, rotation, origin }),
    };
}
