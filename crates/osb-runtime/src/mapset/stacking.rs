use crate::math::Vec2;

use super::hitobject::{HitObject, HitObjectKind};

/// Applies osu!'s stack-height algorithm to the hit objects in place.
///
/// Objects close in time and position stack on top of each other; the resulting
/// `stack_height` is used to offset sprites in per-hitobject effects.
pub fn apply_stacking(objects: &mut [HitObject], stack_leniency: f64, approach_rate: f64) {
    let stack_threshold = preempt_ms(approach_rate) * stack_leniency;
    for i in (0..objects.len()).rev() {
        if matches!(objects[i].kind, HitObjectKind::Spinner) || objects[i].stack_height != 0 {
            continue;
        }
        let start_time = objects[i].start_time;
        for n in (0..i).rev() {
            if matches!(objects[n].kind, HitObjectKind::Spinner) {
                continue;
            }
            if start_time - objects[n].end_time > stack_threshold {
                break;
            }
            if objects[i].position.distance(objects[n].position) < 3.0 {
                objects[i].stack_height = objects[n].stack_height + 1;
                break;
            }
        }
    }
}

/// The visual stack offset in osu! pixels for a given stack height and circle size.
pub fn stack_offset(stack_height: i32, circle_size: f64) -> Vec2 {
    let scale = (1.0 - 0.7 * (circle_size - 5.0) / 5.0) / 2.0;
    let offset = stack_height as f64 * scale * -6.4;
    Vec2::new(offset, offset)
}

fn preempt_ms(approach_rate: f64) -> f64 {
    if approach_rate > 5.0 {
        1200.0 - 150.0 * (approach_rate - 5.0)
    } else {
        1800.0 - 120.0 * approach_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapset::hitobject::HitObjectKind;

    #[test]
    fn overlapping_circles_stack() {
        let mut objects = vec![
            HitObject::new(0.0, 0.0, Vec2::new(100.0, 100.0), HitObjectKind::Circle),
            HitObject::new(10.0, 10.0, Vec2::new(100.0, 100.0), HitObjectKind::Circle),
        ];
        apply_stacking(&mut objects, 0.7, 5.0);
        assert_eq!(objects[1].stack_height, 1);
    }
}
