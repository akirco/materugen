#!/usr/bin/env -S node --experimental-specifier-resolution=node
import {
  hexFromArgb,
  sourceColorFromImage,
  themeFromSourceColor,
} from "@material/material-color-utilities";
import { existsSync } from "fs";
import { writeFile } from "fs/promises";
import { JSDOM } from "jsdom";
import { basename, dirname, resolve } from "path";
import { hasUncaughtExceptionCaptureCallback } from "process";

// Remove node and file name + --experimental-specifier-resolution
// When using node with the option, it's not passed, but when running after global install, it is
const args = process.argv.slice(process.argv.includes('--experimental-specifier-resolution=node') ? 3 : 2);

if (args.length < 2) {
  console.error(`Not enough arguments: ${args.length}`);
  console.error("Usage: materugen <source image> <output json> (light|dark)");
  process.exit(1);
}

let wallpaper = args[0];
const outPut = args[1];
const schemeType = args.length >= 3 ? args[2].toLowerCase() : "dark";

if (!existsSync(wallpaper)) {
  console.error(`${wallpaper} not found!`);
  process.exit(1);
}

// Convert to absolute path
wallpaper = resolve(wallpaper);

if (!["light", "dark"].includes(schemeType)) {
  console.error(`Invalid scheme: ${schemeType}`);
  process.exit(1);
}

const jsdom = new JSDOM(
  `<!DOCTYPE html><img id="wallpaper" src="${basename(wallpaper)}" />`,
  {
    resources: "usable",
    url: `file:///${dirname(wallpaper)}/`,
  }
);

// Fix for sourceColorFromImage (which was meant to run in a browser)
global.document = jsdom.window.document;

// Get source color
const color = await sourceColorFromImage(
  jsdom.window.document.getElementById("wallpaper") as HTMLImageElement
);

// Generate a color scheme
const scheme = themeFromSourceColor(color).schemes[schemeType];

// Convert colorscheme colors to hex
let hexScheme = scheme["props"];

for (const color of Object.keys(hexScheme)) {
  hexScheme[color] = hexFromArgb(hexScheme[color]);
}

// Add a type property which can me useful
hexScheme.type = schemeType;

try {
  await writeFile(outPut, JSON.stringify(hexScheme));
} catch (e) {
  console.error(`Can't write ${outPut}: ${e}`);
  process.exit(1);
}

console.log(`Wrote ${outPut}`);
