# Materugen

Makes use of [Material You Palette (Rust)](https://github.com/dainbrump/material_you_palette) to generate a material you scheme from an image in CLI and output it to a JSON file

## Usage

```bash
# Generate from image
materugen <image path> <output json> [light|dark (default: dark)]

# Generate from hex color
materugen --hex <color> <output json> [light|dark]

# Generate from random color
materugen --random <output json> [light|dark]
```

## Examples

<div style="font-family: -apple-system,BlinkMacSystemFont,Segoe UI,Helvetica,Arial,sans-serif">

### From Hex Color (#052c2c)
```bash
materugen --hex "#052c2c" teal.json light
```

<div style="display: grid; gap: 8px; margin: 20px 0;">
  <div style="display: flex; align-items: center; gap: 8px;">
    <div style="width: 200px; height: 40px; background: #006a6a; border-radius: 4px"></div>
    <code>primary: #006a6a</code>
  </div>
  <div style="display: flex; align-items: center; gap: 8px;">
    <div style="width: 200px; height: 40px; background: #6ff7f6; border-radius: 4px"></div>
    <code>primaryContainer: #6ff7f6</code>
  </div>
</div>

<details>
<summary>View full theme</summary>

```json
{
    "type": "teal",
    "primary": "#006a6a",
    "onPrimary": "#ffffff",
    "primaryContainer": "#6ff7f6",
    "onPrimaryContainer": "#002020",
    "secondary": "#4a6363",
    "onSecondary": "#ffffff",
    "secondaryContainer": "#cce8e7",
    "onSecondaryContainer": "#051f1f",
    "tertiary": "#4b607c",
    "onTertiary": "#ffffff",
    "tertiaryContainer": "#d3e4ff",
    "onTertiaryContainer": "#041c35",
    "error": "#ba1a1a",
    "onError": "#ffffff",
    "errorContainer": "#ffdad6",
    "onErrorContainer": "#410002",
    "background": "#fafdfc",
    "onBackground": "#191c1c",
    "surface": "#fafdfc",
    "onSurface": "#191c1c",
    "surfaceVariant": "#dae5e4",
    "onSurfaceVariant": "#3f4948",
    "outline": "#6f7979",
    "outlineVariant": "#bec9c8",
    "shadow": "#000000",
    "scrim": "#000000",
    "inverseSurface": "#2d3131",
    "inverseOnSurface": "#eff1f0",
    "inversePrimary": "#4cdada"
}
```
</details>

### From Image (1.jpg)
```bash
materugen 1.jpg 1.jpg.json dark
```

<div style="display: grid; gap: 8px; margin: 20px 0;">
  <div style="display: flex; align-items: center; gap: 8px;">
    <div style="width: 200px; height: 40px; background: #c9beff; border-radius: 4px"></div>
    <code>primary: #c9beff</code>
  </div>
  <div style="display: flex; align-items: center; gap: 8px;">
    <div style="width: 200px; height: 40px; background: #48398d; border-radius: 4px"></div>
    <code>primaryContainer: #48398d</code>
  </div>
</div>

<details>
<summary>View full theme</summary>

```json
{
  "type": "dark",
  "primary": "#c9beff",
  "onPrimary": "#312075",
  "primaryContainer": "#48398d",
  "onPrimaryContainer": "#e6deff",
  "secondary": "#c9c3dc",
  "onSecondary": "#312e41",
  "secondaryContainer": "#484459",
  "onSecondaryContainer": "#e6dff9",
  "tertiary": "#edb8cc",
  "onTertiary": "#482535",
  "tertiaryContainer": "#623b4c",
  "onTertiaryContainer": "#ffd8e6",
  "error": "#ffb4ab",
  "onError": "#690005",
  "errorContainer": "#93000a",
  "onErrorContainer": "#ffdad6",
  "background": "#1c1b1f",
  "onBackground": "#e6e1e6",
  "surface": "#1c1b1f",
  "onSurface": "#e6e1e6",
  "surfaceVariant": "#48454e",
  "onSurfaceVariant": "#c9c5d0",
  "outline": "#938f99",
  "outlineVariant": "#48454e",
  "shadow": "#000000",
  "scrim": "#000000",
  "inverseSurface": "#e6e1e6",
  "inverseOnSurface": "#313033",
  "inversePrimary": "#6052a6"
}
```
</details>

### From Random Color (#4D02FC)
```bash
materugen --random random.json dark
```

<div style="display: grid; gap: 8px; margin: 20px 0;">
  <div style="display: flex; align-items: center; gap: 8px;">
    <div style="width: 200px; height: 40px; background: #c8bfff; border-radius: 4px"></div>
    <code>primary: #c8bfff</code>
  </div>
  <div style="display: flex; align-items: center; gap: 8px;">
    <div style="width: 200px; height: 40px; background: #4100db; border-radius: 4px"></div>
    <code>primaryContainer: #4100db</code>
  </div>
</div>

<details>
<summary>View full theme</summary>

```json
{
  "type": "random",
  "primary": "#c8bfff",
  "onPrimary": "#2c009d",
  "primaryContainer": "#4100db",
  "onPrimaryContainer": "#e5deff",
  "secondary": "#c9c3dc",
  "onSecondary": "#312e41",
  "secondaryContainer": "#474459",
  "onSecondaryContainer": "#e5dff9",
  "tertiary": "#ecb8ce",
  "onTertiary": "#482536",
  "tertiaryContainer": "#613b4d",
  "onTertiaryContainer": "#ffd8e7",
  "error": "#ffb4ab",
  "onError": "#690005",
  "errorContainer": "#93000a",
  "onErrorContainer": "#ffdad6",
  "background": "#1c1b1f",
  "onBackground": "#e5e1e6",
  "surface": "#1c1b1f",
  "onSurface": "#e5e1e6",
  "surfaceVariant": "#48454f",
  "onSurfaceVariant": "#c9c5d0",
  "outline": "#928f99",
  "outlineVariant": "#48454f",
  "shadow": "#000000",
  "scrim": "#000000",
  "inverseSurface": "#e5e1e6",
  "inverseOnSurface": "#313033",
  "inversePrimary": "#592dff"
}
```
</details>

</div>

