#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Generate the pumpfun plugin icon.

The icon states the gameplay rather than an abstract motif:

* **the multiblock machine** -- an oak fence standing on a dispenser, the exact
  structure of the Grind Stone, drawn as a Minecraft-style block sprite with
  working particles around it. This is how every machine in the plugin is built:
  vanilla blocks assembled into a structure, then right-clicked.
* **the recipe it runs** -- cobblestone going in, an orange arrow, gravel coming
  out. That is the Grind Stone's first recipe, so the picture is literally what a
  player sees in game: build a structure, feed it an item, get a different item.
* the wordmark **PUMPFUN** -- this project's own name, never "Slimefun", matching
  the NBT-namespace decision documented in the README.

Two outputs come from one geometry definition:

* ``icon.svg``  -- vector source, built with plain string concatenation so it has
  no third-party dependency (the same approach as the reference project `two048`).
* ``icon-64/128/256/512.png`` -- rasterised with Pillow when available, drawn at
  4x and downsampled, which is what keeps the edges clean.
"""

import os

# ---------------------------------------------------------------------------
# Palette
# ---------------------------------------------------------------------------
SLIME = "#55FF55"
ORANGE = "#FF8A3D"

BADGE_TOP = (26, 26, 37)
BADGE_MID = (18, 18, 25)
BADGE_BOTTOM = (10, 10, 16)

DISP_BODY, DISP_BORDER, DISP_BEVEL = "#7D7D7D", "#494949", "#9A9A9A"
DISP_MOUTH, DISP_RING = "#262626", "#A8A8A8"
FENCE_BODY, FENCE_BORDER, FENCE_GRAIN = "#A8763F", "#6E4A22", "#8C5F30"
COBBLE_BODY, COBBLE_BORDER, COBBLE_SPECK = "#8A8A8A", "#5A5A5A", "#6E6E6E"
GRAVEL_BODY, GRAVEL_BORDER = "#A89070", "#6B5C4A"
GRAVEL_SPECK = ["#7A6950", "#C2B49C"]

# ---------------------------------------------------------------------------
# Geometry (512x512 design space)
# ---------------------------------------------------------------------------
W = H = 512
BADGE_RADIUS = 112.0

# fence: a post plus two rails, standing on the dispenser
FENCE_POST = (232.0, 84.0, 48.0, 126.0)
FENCE_RAILS = [(176.0, 100.0, 160.0, 16.0), (176.0, 158.0, 160.0, 16.0)]

# dispenser: a 160x160 front face with a dark mouth and a light ring
DISPENSER = (176.0, 210.0, 160.0, 160.0)
DISP_MOUTH_C = (256.0, 290.0)
DISP_MOUTH_R = 36.0

PARTICLES = [
    (146.0, 128.0, 11.0, ORANGE, 0.75),
    (368.0, 168.0, 9.0, SLIME, 0.60),
    (160.0, 300.0, 8.0, SLIME, 0.50),
    (360.0, 108.0, 10.0, ORANGE, 0.60),
    (378.0, 320.0, 8.0, ORANGE, 0.45),
]

# recipe row: cobblestone -> gravel
BLOCK = 60.0
RECIPE_Y = 392.0
IN_X, OUT_X = 174.0, 288.0
ARROW = (244.0, 278.0, 422.0)   # tail x, head tip x, y
ARROW_HALF = 13.0

COBBLE_SPECKLES = [(10, 12), (34, 8), (46, 30), (18, 38), (38, 46), (8, 28)]
GRAVEL_SPECKLES = [(12, 14), (36, 10), (24, 26), (46, 34), (14, 44), (40, 48)]

WORDMARK_BASELINE, WORDMARK_SIZE = 486.0, 22.0
WORDMARK_TRACKING = 4.0

FONT_UI = "'Segoe UI Semibold', 'Segoe UI', 'DejaVu Sans', Arial, sans-serif"


# ---------------------------------------------------------------------------
# SVG
# ---------------------------------------------------------------------------
def _svg_particles():
    out = []
    for x, y, s, color, alpha in PARTICLES:
        out.append(
            f'<rect x="{x}" y="{y}" width="{s}" height="{s}" rx="2" '
            f'fill="{color}" fill-opacity="{alpha}"/>'
        )
    return "\n".join(out)


def _svg_speckles(x0, y0, speckles, color):
    out = []
    for dx, dy in speckles:
        out.append(
            f'<rect x="{x0 + dx}" y="{y0 + dy}" width="7" height="7" rx="1.5" '
            f'fill="{color}" fill-opacity="0.85"/>'
        )
    return "\n".join(out)


def build_svg():
    a = []
    add = a.append

    add(f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" '
        f'viewBox="0 0 {W} {H}" role="img" aria-label="pumpfun plugin icon">')
    add("<title>pumpfun</title>")
    add("<desc>A grinding machine built from an oak fence on a dispenser, turning "
        "cobblestone into gravel, with working particles around it.</desc>")

    add("<defs>")
    add('<linearGradient id="badge" x1="0" y1="0" x2="1" y2="1">'
        '<stop offset="0" stop-color="#1A1A25"/>'
        '<stop offset="0.55" stop-color="#121219"/>'
        '<stop offset="1" stop-color="#0A0A10"/></linearGradient>')
    add('<radialGradient id="warm" cx="0.20" cy="0.10" r="0.66">'
        f'<stop offset="0" stop-color="{ORANGE}" stop-opacity="0.16"/>'
        f'<stop offset="1" stop-color="{ORANGE}" stop-opacity="0"/></radialGradient>')
    add('<radialGradient id="cool" cx="0.86" cy="0.98" r="0.64">'
        f'<stop offset="0" stop-color="{SLIME}" stop-opacity="0.11"/>'
        f'<stop offset="1" stop-color="{SLIME}" stop-opacity="0"/></radialGradient>')
    add('<filter id="halo" x="-60%" y="-60%" width="220%" height="220%">'
        '<feGaussianBlur stdDeviation="14" result="b"/>'
        '<feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge></filter>')
    add("</defs>")

    add(f'<rect width="{W}" height="{H}" rx="{BADGE_RADIUS}" fill="url(#badge)"/>')
    add(f'<rect width="{W}" height="{H}" rx="{BADGE_RADIUS}" fill="url(#warm)"/>')
    add(f'<rect width="{W}" height="{H}" rx="{BADGE_RADIUS}" fill="url(#cool)"/>')

    # soft halo behind the machine so it reads as "running"
    add('<ellipse cx="256" cy="286" rx="150" ry="150" fill="'
        + ORANGE + '" fill-opacity="0.14" filter="url(#halo)"/>')

    add(_svg_particles())

    # ---- machine: dispenser ----
    dx, dy, dw, dh = DISPENSER
    add(f'<rect x="{dx}" y="{dy}" width="{dw}" height="{dh}" rx="10" '
        f'fill="{DISP_BODY}" stroke="{DISP_BORDER}" stroke-width="7"/>')
    add(f'<rect x="{dx + 7}" y="{dy + 7}" width="{dw - 14}" height="12" rx="4" '
        f'fill="{DISP_BEVEL}" fill-opacity="0.55"/>')
    add(f'<rect x="{dx + 7}" y="{dy + 7}" width="12" height="{dh - 14}" rx="4" '
        f'fill="{DISP_BEVEL}" fill-opacity="0.45"/>')
    add(f'<rect x="{dx + dw - 19}" y="{dy + 7}" width="12" height="{dh - 14}" rx="4" '
        'fill="#000000" fill-opacity="0.18"/>')
    add(f'<circle cx="{DISP_MOUTH_C[0]}" cy="{DISP_MOUTH_C[1]}" r="{DISP_MOUTH_R + 7}" '
        f'fill="none" stroke="{DISP_RING}" stroke-width="6" stroke-opacity="0.85"/>')
    add(f'<circle cx="{DISP_MOUTH_C[0]}" cy="{DISP_MOUTH_C[1]}" r="{DISP_MOUTH_R}" '
        f'fill="{DISP_MOUTH}"/>')

    # ---- machine: fence ----
    for rx, ry, rw, rh in FENCE_RAILS:
        add(f'<rect x="{rx}" y="{ry}" width="{rw}" height="{rh}" rx="4" '
            f'fill="{FENCE_BODY}" stroke="{FENCE_BORDER}" stroke-width="5"/>')
    px, py, pw, ph = FENCE_POST
    add(f'<rect x="{px}" y="{py}" width="{pw}" height="{ph}" rx="5" '
        f'fill="{FENCE_BODY}" stroke="{FENCE_BORDER}" stroke-width="5"/>')
    add(f'<rect x="{px + 12}" y="{py + 12}" width="8" height="{ph - 24}" rx="3" '
        f'fill="{FENCE_GRAIN}"/>')
    add(f'<rect x="{px + 28}" y="{py + 12}" width="8" height="{ph - 24}" rx="3" '
        f'fill="{FENCE_GRAIN}"/>')

    # ---- recipe row ----
    add(f'<rect x="{IN_X}" y="{RECIPE_Y}" width="{BLOCK}" height="{BLOCK}" rx="8" '
        f'fill="{COBBLE_BODY}" stroke="{COBBLE_BORDER}" stroke-width="5"/>')
    add(_svg_speckles(IN_X, RECIPE_Y, COBBLE_SPECKLES, COBBLE_SPECK))
    add(f'<rect x="{OUT_X}" y="{RECIPE_Y}" width="{BLOCK}" height="{BLOCK}" rx="8" '
        f'fill="{GRAVEL_BODY}" stroke="{GRAVEL_BORDER}" stroke-width="5"/>')
    add(_svg_speckles(OUT_X, RECIPE_Y, GRAVEL_SPECKLES, GRAVEL_SPECK[0]))
    add(f'<rect x="{OUT_X + 24}" y="{RECIPE_Y + 30}" width="7" height="7" rx="1.5" '
        f'fill="{GRAVEL_SPECK[1]}" fill-opacity="0.85"/>')

    tx, hx, ty = ARROW
    add(f'<path d="M{tx} {ty} L{hx - ARROW_HALF} {ty}" stroke="{ORANGE}" '
        'stroke-width="10" stroke-linecap="round" fill="none"/>')
    add(f'<path d="M{hx - ARROW_HALF - 2} {ty - ARROW_HALF} L{hx} {ty} '
        f'L{hx - ARROW_HALF - 2} {ty + ARROW_HALF} Z" fill="{ORANGE}"/>')

    add(f'<text x="{W / 2}" y="{WORDMARK_BASELINE}" text-anchor="middle" '
        f'font-family="{FONT_UI}" font-size="{WORDMARK_SIZE}" font-weight="700" '
        f'letter-spacing="{WORDMARK_TRACKING}" fill="{SLIME}" fill-opacity="0.78">'
        "PUMPFUN</text>")

    add(f'<rect x="1.5" y="1.5" width="{W - 3}" height="{H - 3}" rx="{BADGE_RADIUS - 1.5}" '
        'fill="none" stroke="#FFFFFF" stroke-opacity="0.10" stroke-width="3"/>')

    add("</svg>")
    return "\n".join(a)


# ---------------------------------------------------------------------------
# PNG (Pillow)
# ---------------------------------------------------------------------------
S = 4


def _font(paths, size):
    from PIL import ImageFont

    for path in paths:
        if os.path.exists(path):
            return ImageFont.truetype(path, size)
    return ImageFont.load_default(size)


def _diagonal_gradient(size):
    from PIL import Image

    tiny = Image.new("RGB", (2, 2))
    tiny.putpixel((0, 0), (30, 30, 42))
    tiny.putpixel((1, 0), BADGE_TOP)
    tiny.putpixel((0, 1), BADGE_MID)
    tiny.putpixel((1, 1), BADGE_BOTTOM)
    return tiny.resize((size, size), Image.BICUBIC).convert("RGBA")


def _radial_glow(size, cx, cy, radius, color, max_alpha):
    from PIL import Image, ImageDraw

    layer = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(layer, "RGBA")
    steps = 72
    for step in range(steps, 0, -1):
        t = step / steps
        r = radius * t
        alpha = int(max_alpha * (1.0 - t) ** 1.7)
        if alpha > 0:
            d.ellipse([cx - r, cy - r, cx + r, cy + r], fill=(*color, alpha))
    return layer


def _draw_tracking(draw, text, x, y, font, tracking, fill):
    widths = [draw.textlength(ch, font=font) for ch in text]
    total = sum(widths) + tracking * max(0, len(text) - 1)
    cur = x - total / 2
    for ch, w in zip(text, widths):
        draw.text((cur, y), ch, font=font, fill=fill, anchor="ls")
        cur += w + tracking


def _hex(value):
    return tuple(int(value[i : i + 2], 16) for i in (1, 3, 5))


def render_pngs(out_dir):
    from PIL import Image, ImageChops, ImageDraw

    size = W * S

    def sx(v):
        return v * S

    img = _diagonal_gradient(size)
    img = Image.alpha_composite(img, _radial_glow(size, sx(105), sx(60), sx(238), (255, 138, 61), 32))
    img = Image.alpha_composite(img, _radial_glow(size, sx(430), sx(500), sx(228), (85, 255, 85), 24))
    # Soft radial halo behind the machine, so it reads as "running"
    img = Image.alpha_composite(img, _radial_glow(size, sx(256), sx(288), sx(190), (255, 138, 61), 52))

    shapes = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(shapes, "RGBA")

    # particles
    for x, y, s, color, alpha in PARTICLES:
        d.rounded_rectangle(
            [sx(x), sx(y), sx(x + s), sx(y + s)],
            radius=int(sx(2)),
            fill=(*_hex(color), int(alpha * 255)),
        )

    # dispenser
    dx, dy, dw, dh = DISPENSER
    d.rounded_rectangle(
        [sx(dx), sx(dy), sx(dx + dw), sx(dy + dh)],
        radius=int(sx(10)),
        fill=(*_hex(DISP_BODY), 255),
        outline=(*_hex(DISP_BORDER), 255),
        width=int(sx(7)),
    )
    d.rounded_rectangle(
        [sx(dx + 7), sx(dy + 7), sx(dx + dw - 7), sx(dy + 19)],
        radius=int(sx(4)),
        fill=(*_hex(DISP_BEVEL), 150),
    )
    d.rounded_rectangle(
        [sx(dx + 7), sx(dy + 7), sx(dx + 19), sx(dy + dh - 7)],
        radius=int(sx(4)),
        fill=(*_hex(DISP_BEVEL), 120),
    )
    d.rounded_rectangle(
        [sx(dx + dw - 19), sx(dy + 7), sx(dx + dw - 7), sx(dy + dh - 7)],
        radius=int(sx(4)),
        fill=(0, 0, 0, 30),
    )
    mcx, mcy = sx(DISP_MOUTH_C[0]), sx(DISP_MOUTH_C[1])
    rr = sx(DISP_MOUTH_R + 7)
    d.ellipse([mcx - rr, mcy - rr, mcx + rr, mcy + rr], outline=(*_hex(DISP_RING), 217), width=int(sx(6)))
    mr = sx(DISP_MOUTH_R)
    d.ellipse([mcx - mr, mcy - mr, mcx + mr, mcy + mr], fill=(*_hex(DISP_MOUTH), 255))

    # fence
    for rx, ry, rw, rh in FENCE_RAILS:
        d.rounded_rectangle(
            [sx(rx), sx(ry), sx(rx + rw), sx(ry + rh)],
            radius=int(sx(4)),
            fill=(*_hex(FENCE_BODY), 255),
            outline=(*_hex(FENCE_BORDER), 255),
            width=int(sx(5)),
        )
    px, py, pw, ph = FENCE_POST
    d.rounded_rectangle(
        [sx(px), sx(py), sx(px + pw), sx(py + ph)],
        radius=int(sx(5)),
        fill=(*_hex(FENCE_BODY), 255),
        outline=(*_hex(FENCE_BORDER), 255),
        width=int(sx(5)),
    )
    for gx in (12, 28):
        d.rounded_rectangle(
            [sx(px + gx), sx(py + 12), sx(px + gx + 8), sx(py + ph - 12)],
            radius=int(sx(3)),
            fill=(*_hex(FENCE_GRAIN), 255),
        )

    # recipe row
    for x0, body, border, speckles, speck_colors in (
        (IN_X, COBBLE_BODY, COBBLE_BORDER, COBBLE_SPECKLES, [COBBLE_SPECK]),
        (OUT_X, GRAVEL_BODY, GRAVEL_BORDER, GRAVEL_SPECKLES, GRAVEL_SPECK),
    ):
        d.rounded_rectangle(
            [sx(x0), sx(RECIPE_Y), sx(x0 + BLOCK), sx(RECIPE_Y + BLOCK)],
            radius=int(sx(8)),
            fill=(*_hex(body), 255),
            outline=(*_hex(border), 255),
            width=int(sx(5)),
        )
        for i, (dxs, dys) in enumerate(speckles):
            col = speck_colors[i % len(speck_colors)]
            d.rounded_rectangle(
                [sx(x0 + dxs), sx(RECIPE_Y + dys), sx(x0 + dxs + 7), sx(RECIPE_Y + dys + 7)],
                radius=int(sx(1.5)),
                fill=(*_hex(col), 217),
            )

    tx, hx, ty = ARROW
    d.line([sx(tx), sx(ty), sx(hx - ARROW_HALF), sx(ty)], fill=(*_hex(ORANGE), 255), width=int(sx(10)))
    d.polygon(
        [
            (sx(hx - ARROW_HALF - 2), sx(ty - ARROW_HALF)),
            (sx(hx), sx(ty)),
            (sx(hx - ARROW_HALF - 2), sx(ty + ARROW_HALF)),
        ],
        fill=(*_hex(ORANGE), 255),
    )

    # wordmark
    ui = _font(
        [
            "C:/Windows/Fonts/segoeuib.ttf",
            "C:/Windows/Fonts/segoeui.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        ],
        int(sx(WORDMARK_SIZE)),
    )
    _draw_tracking(
        d, "PUMPFUN", sx(W / 2), sx(WORDMARK_BASELINE), ui, sx(WORDMARK_TRACKING), (85, 255, 85, 199)
    )

    # inner hairline
    inset = sx(1.5)
    d.rounded_rectangle(
        [inset, inset, size - 1 - inset, size - 1 - inset],
        radius=int(sx(BADGE_RADIUS - 1.5)),
        outline=(255, 255, 255, 26),
        width=int(sx(3)),
    )

    img = Image.alpha_composite(img, shapes)

    mask = Image.new("L", (size, size), 0)
    ImageDraw.Draw(mask).rounded_rectangle(
        [0, 0, size - 1, size - 1], radius=int(sx(BADGE_RADIUS)), fill=255
    )
    img.putalpha(ImageChops.multiply(img.getchannel("A"), mask))

    written = []
    for px in (512, 256, 128, 64):
        path = os.path.join(out_dir, f"icon-{px}.png")
        img.resize((px, px), Image.LANCZOS).save(path, "PNG", optimize=True)
        written.append(path)
    return written


def main():
    out_dir = os.path.dirname(os.path.abspath(__file__))
    svg_path = os.path.join(out_dir, "icon.svg")
    with open(svg_path, "w", encoding="utf-8") as fh:
        fh.write(build_svg())
    print(svg_path)
    try:
        for path in render_pngs(out_dir):
            print(path)
    except ImportError:
        print("Pillow not installed; skipped PNG rendering (icon.svg is still written).")


if __name__ == "__main__":
    main()
