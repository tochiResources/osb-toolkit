using System;
using System.Collections.Generic;
using System.Linq;
using OpenTK;
using OpenTK.Graphics;
using osbTKCommon.Mapset;
using osbTKCommon.Scripting;
using osbTKCommon.Storyboarding;
using osbTKCommon.Storyboarding.Util;
using osbTKCommon.Subtitles;
using osbTKCommon.Util;

namespace osbTKScripts
{
    public class ThreeDimRing : StoryboardObjectGenerator
    {
        [Configurable]
        public bool ShowDot = true;

        [Configurable]
        public OsbOrigin DotOrigin = OsbOrigin.Centre;

        [Configurable]
        public string DotSpritePath = "sb/particle.png";

        [Configurable]
        public bool DotRotation = false;

        [Configurable]
        public float MinDotRotation = 0;

        [Configurable]
        public float MaxDotRotation = 0;

        [Configurable]
        public bool ShowTrail = true;

        [Configurable]
        public OsbOrigin TrailOrigin = OsbOrigin.Centre;

        [Configurable]
        public string TrailSpritePath = "sb/particle.png";

        [Configurable]
        public float TrailScale = 0.25f;

        [Configurable]
        public bool RandomTrailRotation = false;

        [Configurable]
        public float MinTrailRotation = 0;

        [Configurable]
        public float MaxTrailRotation = 0;

        [Configurable]
        public bool ShowOriginMarker = false;

        [Configurable]
        public OsbOrigin MarkerOrigin = OsbOrigin.Centre;

        [Configurable]
        public string OriginMarkerPath = "sb/particle.png";

        [Configurable]
        public float OriginMarkerScale = 1;

        [Configurable]
        public bool RandomMarkerRotation = false;

        [Configurable]
        public float MinMarkerRotation = 0;

        [Configurable]
        public float MaxMarkerRotation = 0;

        [Configurable]
        public Color4 OriginMarkerColor = Color4.Cyan;

        [Configurable]
        public int StartTime = 100;

        [Configurable]
        public int EndTime = 10000;

        [Configurable]
        public float CentreX = 320;

        [Configurable]
        public float CentreY = 240;

        [Configurable]
        public float CentreZ = 0;

        [Configurable]
        public float Form = 1;

        [Configurable]
        public float CutSmoothness = 1;

        [Configurable]
        public float yAxisMovement = 50;

        [Configurable]
        public double Speed = 0.015;

        [Configurable]
        public double Diameter = 20;

        [Configurable]
        public float RotationMovementY = 0;

        [Configurable]
        public float MovementZ = 0;

        [Configurable]
        public float DotFadeIn = 1;

        [Configurable]
        public float DotFadeOut = 0;

        [Configurable]
        public bool DotAdditive = true;

        [Configurable]
        public Color4 DotColor = Color4.White;

        [Configurable]
        public float TrailFadeIn = 0.25f;

        [Configurable]
        public float TrailFadeOut = 0;

        [Configurable]
        public bool TrailAdditive = true;

        [Configurable]
        public float TailLength = 600;

        [Configurable]
        public float TailTilt = 0;

        [Configurable]
        public bool RandomTrailColor = false;

        [Configurable]
        public Color4 MinColor = Color4.White;

        [Configurable]
        public Color4 MaxColor = Color4.White;

        public override void Generate()
        {
            WindRing();
        }

        private void WindRing()
        {
            // define some of the useful things
            var BeatDuration = Beatmap.GetTimingPointAt((int)StartTime).BeatDuration;
            var FadeTime = BeatDuration;
            var SpeedTime = BeatDuration * Speed;

            // the 3d x and y axis is the same as sb's coordinate axis here.
            // and about the 3d x axis,
            // The positive direction is the back side of the screen. The negative direction is the front side of the screen.

            // the origin of 3d coordinate on the sb coordinate
            var CenterPos = new Vector3(CentreX, CentreY, CentreZ);

            // the target object's position of 3d coordinate
            var Dot_ThreeDimVertex = new Vector4((float)Diameter, RotationMovementY, MovementZ, 1);

            // this is the target object
            var Dot = GetLayer("Dot")
                .CreateSprite(
                    DotSpritePath,
                    DotOrigin,
                    new Vector2(
                        Dot_ThreeDimVertex.X + CenterPos.X,
                        Dot_ThreeDimVertex.Y + CenterPos.Y
                    )
                );

            // dirty work session
            var SampleAmount = (int)Math.Ceiling((double)(EndTime - StartTime) / (double)SpeedTime);
            var MaxSize = Math.Max(
                Math.Max(Math.Abs(Dot_ThreeDimVertex.X), Math.Abs(Dot_ThreeDimVertex.Y)),
                Math.Abs(Dot_ThreeDimVertex.Z)
            );

            if (ShowOriginMarker)
            {
                // For the origin confirmation
                var OriginMarker = GetLayer("OriginMarker")
                    .CreateSprite(
                        OriginMarkerPath,
                        MarkerOrigin,
                        new Vector2(CenterPos.X, CenterPos.Y)
                    );

                OriginMarker.Color(StartTime, OriginMarkerColor);
                OriginMarker.Scale(StartTime, OriginMarkerScale);
                OriginMarker.Fade(StartTime, StartTime + FadeTime, 0, 1);
                OriginMarker.Fade(EndTime - FadeTime, EndTime, 1, 0);

                if (RandomMarkerRotation)
                {
                    var angle = Random(
                        MathHelper.DegreesToRadians(MinMarkerRotation),
                        MathHelper.DegreesToRadians(MaxMarkerRotation)
                    );
                    OriginMarker.Rotate(StartTime, EndTime, MinMarkerRotation, MaxMarkerRotation);
                }
                else
                {
                    OriginMarker.Rotate(StartTime, EndTime, MinMarkerRotation, MaxMarkerRotation);
                }
            }

            if (ShowDot)
            {
                Dot.Color(StartTime, DotColor);
                Dot.Fade(StartTime, StartTime + FadeTime, DotFadeOut, DotFadeIn);
                Dot.Fade(EndTime - FadeTime, EndTime, DotFadeIn, DotFadeOut);

                if (DotRotation)
                {
                    Dot.Rotate(StartTime, EndTime, MinDotRotation, MaxDotRotation);
                }

                if (DotAdditive)
                {
                    Dot.Additive(StartTime, StartTime);
                }

                for (int i = 0; i < SampleAmount; i++)
                {
                    // Camera's position is rotating around the Y axis.
                    var eyeRadian = dir2Rot(Form) * i;
                    var eye = new Vector3(
                        (float)(50 * Math.Cos(eyeRadian)),
                        CutSmoothness,
                        (float)(yAxisMovement * Math.Sin(eyeRadian))
                    );

                    // Camera's Target is the origin.
                    var target = new Vector3(0, 0, 0);

                    // Camera's Upside is minus side of sb's Y axis.
                    var up = new Vector3(0, -1, 500);

                    // This order is important
                    var Dot_ApparentPos = Matrix4.LookAt(eye, target, up) * Dot_ThreeDimVertex;

                    // dirty work session
                    double Dot_Time1 = StartTime + SpeedTime * i;
                    double Dot_Time2 = Dot_Time1 + SpeedTime;
                    var Dot_Pos1 = Dot.PositionAt(Dot_Time1);
                    var Dot_Pos2 = new Vector2(
                        Dot_ApparentPos.X + CenterPos.X,
                        Dot_ApparentPos.Y + CenterPos.Y
                    );
                    var Dot_Width = Math.Abs(MaxSize * 2 + Dot_ApparentPos.Z) * 0.4;

                    Dot.Move(Dot_Time1, Dot_Time2, Dot_Pos1, Dot_Pos2);
                    Dot.Scale(
                        Dot_Time1,
                        Dot_Time2,
                        Dot.ScaleAt(Dot_Time1).X,
                        Calculator_sbScale(Dot, Dot_Width)
                    );
                }
            }

            // For a locus confirmation
            Blur(
                ref Dot,
                StartTime,
                EndTime,
                SpeedTime * TrailFadeIn,
                FadeTime * 3,
                new Vector2(TailLength, TailTilt)
            );
        }

        // For a locus confirmation
        private void Blur(
            ref OsbSprite Content,
            double StartTime,
            double EndTime,
            double SpeedTime,
            double FadeTime,
            Vector2 MoveLength
        )
        {
            if (ShowTrail)
            {
                for (double sNowTime = StartTime; sNowTime <= EndTime; sNowTime += SpeedTime)
                {
                    var sContentPos = Content.PositionAt(sNowTime);
                    var sContentScale = Content.ScaleAt(sNowTime);
                    var TrailColor = RandomTrailColor
                        ? new Color4(
                            (float)Random(MinColor.R, MaxColor.R),
                            (float)Random(MinColor.G, MaxColor.G),
                            (float)Random(MinColor.B, MaxColor.B),
                            255
                        )
                        : MinColor;

                    var Trail = GetLayer("Trail")
                        .CreateSprite(TrailSpritePath, TrailOrigin, sContentPos);

                    Trail.Color(sNowTime, TrailColor);
                    Trail.Fade(sNowTime, sNowTime + FadeTime, TrailFadeIn, TrailFadeOut);
                    Trail.ScaleVec(sNowTime, sContentScale * TrailScale);
                    Trail.Move(
                        OsbEasing.In,
                        sNowTime,
                        sNowTime + FadeTime,
                        sContentPos.X,
                        sContentPos.Y,
                        sContentPos.X + MoveLength.X,
                        sContentPos.Y + MoveLength.Y
                    );

                    if (RandomMarkerRotation)
                    {
                        var angle = Random(
                            MathHelper.DegreesToRadians(MinTrailRotation),
                            MathHelper.DegreesToRadians(MaxTrailRotation)
                        );
                        Trail.Rotate(
                            sNowTime,
                            sNowTime + FadeTime,
                            MinTrailRotation,
                            MaxTrailRotation
                        );
                    }
                    else
                    {
                        Trail.Rotate(
                            sNowTime,
                            sNowTime + FadeTime,
                            MinTrailRotation,
                            MaxTrailRotation
                        );
                    }

                    if (TrailAdditive)
                    {
                        Trail.Additive(sNowTime, sNowTime + FadeTime);
                    }
                }
            }
        }

        private double Calculator_sbScale(OsbSprite sprite, double Width)
        {
            return (1);
        }

        private static float dir2Rot(double angle)
        {
            return (float)(angle / 180 * Math.PI);
        }
    }
}
