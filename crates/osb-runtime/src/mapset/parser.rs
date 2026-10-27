use crate::error::RuntimeResult;
use crate::math::{Color4, Vec2};

use super::beatmap::Beatmap;
use super::control_point::ControlPoint;
use super::hitobject::{HitObject, HitObjectKind};
use super::slider::{CurveType, SliderPath};

/// Parses a `.osu` beatmap from text.
pub fn parse_osu(text: &str) -> RuntimeResult<Beatmap> {
    let mut beatmap = Beatmap::new();
    let mut section = String::new();

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].to_string();
            continue;
        }
        match section.as_str() {
            "General" => parse_general(line, &mut beatmap),
            "Metadata" => parse_metadata(line, &mut beatmap),
            "Difficulty" => parse_difficulty(line, &mut beatmap),
            "Events" => parse_event(line, &mut beatmap),
            "Colours" => parse_colour(line, &mut beatmap),
            "TimingPoints" => parse_timing_point(line, &mut beatmap),
            "HitObjects" => parse_hit_object(line, &mut beatmap),
            _ => {}
        }
    }

    beatmap
        .timing_points
        .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    beatmap
        .hit_objects
        .sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
    beatmap.apply_stacking();
    beatmap.apply_colors();
    Ok(beatmap)
}

fn split_key_value(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(':')?;
    Some((key.trim(), value.trim()))
}

fn parse_general(line: &str, beatmap: &mut Beatmap) {
    if let Some((key, value)) = split_key_value(line) {
        match key {
            "AudioFilename" => beatmap.audio_filename = value.to_string(),
            "Mode" => beatmap.mode = value.parse().unwrap_or(0),
            "StackLeniency" => beatmap.stack_leniency = value.parse().unwrap_or(0.7),
            _ => {}
        }
    }
}

fn parse_metadata(line: &str, beatmap: &mut Beatmap) {
    if let Some((key, value)) = split_key_value(line) {
        match key {
            "Title" => beatmap.title = value.to_string(),
            "Artist" => beatmap.artist = value.to_string(),
            "Creator" => beatmap.creator = value.to_string(),
            "Version" => beatmap.version = value.to_string(),
            _ => {}
        }
    }
}

fn parse_difficulty(line: &str, beatmap: &mut Beatmap) {
    if let Some((key, value)) = split_key_value(line) {
        match key {
            "ApproachRate" => beatmap.approach_rate = value.parse().unwrap_or(5.0),
            "CircleSize" => beatmap.circle_size = value.parse().unwrap_or(5.0),
            "OverallDifficulty" => beatmap.overall_difficulty = value.parse().unwrap_or(5.0),
            "SliderMultiplier" => beatmap.slider_multiplier = value.parse().unwrap_or(1.4),
            "SliderTickRate" => beatmap.slider_tick_rate = value.parse().unwrap_or(1.0),
            _ => {}
        }
    }
}

fn parse_colour(line: &str, beatmap: &mut Beatmap) {
    let Some((key, value)) = split_key_value(line) else {
        return;
    };
    if !key.starts_with("Combo") {
        return;
    }
    let channels: Vec<u8> = value
        .split(',')
        .filter_map(|channel| channel.trim().parse::<u8>().ok())
        .collect();
    if channels.len() >= 3 {
        beatmap.combo_colors.push(Color4::from_rgb(
            channels[0] as f64 / 255.0,
            channels[1] as f64 / 255.0,
            channels[2] as f64 / 255.0,
        ));
    }
}

fn parse_event(line: &str, beatmap: &mut Beatmap) {
    if beatmap.background_path.is_some() {
        return;
    }
    let fields: Vec<&str> = line.split(',').collect();
    if fields.first().map(|value| value.trim()) != Some("0") {
        return;
    }
    if let Some(filename) = fields.get(2) {
        let name = filename.trim().trim_matches('"');
        if !name.is_empty() {
            beatmap.background_path = Some(name.to_string());
        }
    }
}

fn parse_timing_point(line: &str, beatmap: &mut Beatmap) {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() < 2 {
        return;
    }
    let time: f64 = fields[0].trim().parse().unwrap_or(0.0);
    let beat_length: f64 = fields[1].trim().parse().unwrap_or(0.0);
    let uninherited = fields
        .get(6)
        .map(|value| value.trim() != "0")
        .unwrap_or(beat_length > 0.0);
    let mut point = if uninherited {
        ControlPoint::new(time, beat_length, true)
    } else {
        let mut green = ControlPoint::new(time, 0.0, false);
        green.speed_multiplier = if beat_length == 0.0 {
            1.0
        } else {
            -100.0 / beat_length
        };
        green
    };
    if let Some(effects) = fields.get(7) {
        point.effect_flags = effects.trim().parse().unwrap_or(0);
        point.omit_first_bar_line = point.effect_flags & 8 != 0;
    }
    beatmap.timing_points.push(point);
}

fn parse_hit_object(line: &str, beatmap: &mut Beatmap) {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() < 4 {
        return;
    }
    let x: f64 = fields[0].trim().parse().unwrap_or(0.0);
    let y: f64 = fields[1].trim().parse().unwrap_or(0.0);
    let time: f64 = fields[2].trim().parse().unwrap_or(0.0);
    let kind_bits: u32 = fields[3].trim().parse().unwrap_or(1);
    let position = Vec2::new(x, y);

    let mut object = if kind_bits & 128 != 0 {
        let end_time = fields
            .get(5)
            .and_then(|value| value.split(':').next())
            .and_then(|value| value.trim().parse::<f64>().ok())
            .unwrap_or(time);
        HitObject::new(time, end_time, position, HitObjectKind::Hold)
    } else if kind_bits & 8 != 0 {
        let end_time = fields
            .get(5)
            .and_then(|value| value.trim().parse::<f64>().ok())
            .unwrap_or(time);
        HitObject::new(time, end_time, position, HitObjectKind::Spinner)
    } else if kind_bits & 2 != 0 {
        let (path, end_time) = parse_slider(&fields, time, beatmap);
        HitObject::new(time, end_time, position, HitObjectKind::Slider(path))
    } else {
        HitObject::new(time, time, position, HitObjectKind::Circle)
    };

    object.new_combo = kind_bits & 4 != 0;
    beatmap.hit_objects.push(object);
}

fn parse_slider(fields: &[&str], start_time: f64, beatmap: &Beatmap) -> (SliderPath, f64) {
    let definition = fields.get(5).copied().unwrap_or("");
    let mut parts = definition.split('|');
    let curve_type = parts
        .next()
        .and_then(|value| value.chars().next())
        .and_then(CurveType::from_char)
        .unwrap_or(CurveType::Linear);

    let mut control_points = Vec::new();
    for point in parts {
        let mut coords = point.split(':');
        let px: f64 = coords.next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
        let py: f64 = coords.next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
        control_points.push(Vec2::new(px, py));
    }

    let slides: f64 = fields
        .get(6)
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(1.0);
    let length: f64 = fields
        .get(7)
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(0.0);

    let beat_duration = beatmap
        .timing_point_at(start_time)
        .map(|p| p.beat_duration)
        .unwrap_or(500.0);
    let speed = beatmap
        .control_point_at(start_time)
        .map(|p| {
            if p.timing_change {
                1.0
            } else {
                p.speed_multiplier
            }
        })
        .unwrap_or(1.0);
    let duration = if beatmap.slider_multiplier == 0.0 {
        0.0
    } else {
        length / (100.0 * beatmap.slider_multiplier * speed) * beat_duration * slides
    };

    let path = SliderPath::new(control_points, curve_type, length);
    (path, start_time + duration)
}

/// Parses a `.osu` file from disk.
pub fn parse_osu_file(path: impl AsRef<std::path::Path>) -> RuntimeResult<Beatmap> {
    let text = osb_core::io::read_string(path)?;
    parse_osu(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[General]\nAudioFilename: song.mp3\nMode: 0\nStackLeniency: 0.7\n\n[Difficulty]\nApproachRate: 9\nCircleSize: 4\nOverallDifficulty: 8\nSliderMultiplier: 1.8\nSliderTickRate: 1\n\n[TimingPoints]\n1000,500,4,2,0,60,1,0\n5000,-50,4,2,0,60,0,0\n\n[HitObjects]\n100,100,1000,1,0,0:0:0:0:\n200,200,1500,2,0,B|300:300|400:200,1,140\n256,192,3000,8,0,4000,0:0:0:0:\n";

    #[test]
    fn parses_metadata_and_difficulty() {
        let map = parse_osu(SAMPLE).unwrap();
        assert_eq!(map.audio_filename, "song.mp3");
        assert_eq!(map.approach_rate, 9.0);
        assert_eq!(map.circle_size, 4.0);
        assert!((map.slider_multiplier - 1.8).abs() < 1e-9);
    }

    #[test]
    fn parses_timing_points() {
        let map = parse_osu(SAMPLE).unwrap();
        assert_eq!(map.timing_points.len(), 2);
        assert!(map.timing_points[0].timing_change);
        assert!((map.bpm_at(1000.0) - 120.0).abs() < 1e-6);
        assert!(!map.timing_points[1].timing_change);
        assert!((map.timing_points[1].speed_multiplier - 2.0).abs() < 1e-9);
    }

    #[test]
    fn parses_hit_objects() {
        let map = parse_osu(SAMPLE).unwrap();
        assert_eq!(map.hit_objects.len(), 3);
        assert!(map.hit_objects[0].end_time == 1000.0);
        assert!(map.hit_objects[1].is_slider());
        assert!(map.hit_objects[1].end_time > map.hit_objects[1].start_time);
        assert!(map.hit_objects[2].is_spinner());
    }

    #[test]
    fn parses_background_event() {
        let text = "[Events]\n//Background and Video events\n0,0,\"bg.jpg\",0,0\n";
        let map = parse_osu(text).unwrap();
        assert_eq!(map.background_path.as_deref(), Some("bg.jpg"));
    }
}
