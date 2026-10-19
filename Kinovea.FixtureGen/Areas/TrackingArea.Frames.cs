using System;
using System.Drawing;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;
using Newtonsoft.Json.Linq;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Renders the synthetic frames of a tracking case by an integer formula, written straight into the bitmap
    /// bits (no GDI+ drawing) so the Rust port can produce the same pixels.
    /// Background, per channel c (B, G, R): clamp(base[c] + (x * gx + y * gy) / 16, 0, 255) (integer division).
    /// Object: a square of side "size" or a disc of radius "size", centered at the path point given in
    /// quarter pixels. Each pixel is sampled on a 4x4 grid at eighth-pixel offsets 1, 3, 5, 7; with k samples
    /// inside, the value is (background * (16 - k) + object * k) / 16. A sample (sx, sy) in eighths is inside
    /// the square when |sx - cx| &lt; 4 * size and |sy - cy| &lt; 4 * size, inside the disc when
    /// (sx - cx)^2 + (sy - cy)^2 &lt; (8 * size)^2, with (cx, cy) the center in eighths (2 * quarter pixels).
    /// </summary>
    public partial class TrackingArea
    {
        private int width;
        private int height;
        private int[] background;
        private int gx;
        private int gy;
        private int[] color;
        private bool disc;
        private int size;

        private void SetupFrames(FixtureCase fixtureCase)
        {
            width = fixtureCase.Input("width", 160);
            height = fixtureCase.Input("height", 120);
            background = fixtureCase.Input("background", new[] { 40, 60, 80 });
            gx = fixtureCase.Input("gx", 8);
            gy = fixtureCase.Input("gy", 4);
            color = fixtureCase.Input("color", new[] { 230, 230, 230 });
            disc = fixtureCase.Input("shape", "square") == "disc";
            size = fixtureCase.Input("size", 8);
        }

        private Bitmap Render(JArray center)
        {
            int cx = 2 * (int)center[0];
            int cy = 2 * (int)center[1];
            byte[] bits = new byte[width * height * 4];
            for (int y = 0; y < height; y++)
            {
                for (int x = 0; x < width; x++)
                {
                    int k = Coverage(8 * x, 8 * y, cx, cy);
                    int offset = (y * width + x) * 4;
                    for (int c = 0; c < 3; c++)
                    {
                        int bg = Math.Max(0, Math.Min(255, background[c] + (x * gx + y * gy) / 16));
                        bits[offset + c] = (byte)((bg * (16 - k) + color[c] * k) / 16);
                    }

                    bits[offset + 3] = 255;
                }
            }

            Bitmap bitmap = new Bitmap(width, height, PixelFormat.Format32bppPArgb);
            BitmapData data = bitmap.LockBits(new Rectangle(0, 0, width, height), ImageLockMode.WriteOnly, PixelFormat.Format32bppPArgb);
            for (int y = 0; y < height; y++)
                Marshal.Copy(bits, y * width * 4, data.Scan0 + y * data.Stride, width * 4);
            bitmap.UnlockBits(data);
            return bitmap;
        }

        private int Coverage(int x8, int y8, int cx, int cy)
        {
            int k = 0;
            for (int sy = 1; sy < 8; sy += 2)
            {
                for (int sx = 1; sx < 8; sx += 2)
                {
                    long dx = x8 + sx - cx;
                    long dy = y8 + sy - cy;
                    bool inside = disc
                        ? dx * dx + dy * dy < 64L * size * size
                        : Math.Abs(dx) < 4 * size && Math.Abs(dy) < 4 * size;
                    if (inside)
                        k++;
                }
            }

            return k;
        }
    }
}
