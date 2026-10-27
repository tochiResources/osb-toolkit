using System;
using System.Linq;
using OpenTK;
using osbTKCommon.Scripting;
using osbTKCommon.Storyboarding;

namespace osbTKScripts
{
    public class Spectrum : StoryboardObjectGenerator
    {
        [Group("Timing")]
        [Configurable]
        public int StartTime = 0;

        [Configurable]
        public int EndTime = 10000;

        [Configurable]
        public int BeatDivisor = 16;

        [Group("Sprite")]
        [Configurable]
        public string SpritePath = "sb/bar.png";

        [Configurable]
        public OsbOrigin SpriteOrigin = OsbOrigin.BottomLeft;

        [Configurable]
        public Vector2 SpriteScale = new Vector2(1, 100);

        [Group("Bars")]
        [Configurable]
        public Vector2 Position = new Vector2(0, 400);

        [Configurable]
        public float Width = 640;

        [Configurable]
        public int BarCount = 96;

        [Configurable]
        public float MinimalHeight = 0.05f;

        public override void Generate()
        {
            if (StartTime == EndTime && Beatmap.HitObjects.FirstOrDefault() != null)
            {
                StartTime = (int)Beatmap.HitObjects.First().StartTime;
                EndTime = (int)Beatmap.HitObjects.Last().EndTime;
            }

            var layer = GetLayer("Spectrum");
            var barWidth = Width / BarCount;
            for (var i = 0; i < BarCount; i++)
            {
                CancellationToken.ThrowIfCancellationRequested();

                var bar = layer.CreateSprite(
                    SpritePath,
                    SpriteOrigin,
                    new Vector2(Position.X + i * barWidth, Position.Y)
                );
                bar.Additive(StartTime, EndTime);
                bar.Fade(StartTime, EndTime, 1, 1);
            }
        }
    }
}
