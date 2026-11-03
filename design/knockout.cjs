// Clears the white margin to transparency without touching the artwork:
// flood fill from the four edges only, so white strokes inside the plate stay.
const sharp = require('sharp');

const [input, output] = process.argv.slice(2);

(async () => {
  const { data, info } = await sharp(input).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
  const { width, height } = info;
  const total = width * height;
  const cleared = new Uint8Array(total);

  const isPaper = (index) => {
    const offset = index * 4;
    const r = data[offset];
    const g = data[offset + 1];
    const b = data[offset + 2];
    const spread = Math.max(r, g, b) - Math.min(r, g, b);
    return r > 200 && g > 200 && b > 200 && spread < 22;
  };

  const stack = [];
  for (let x = 0; x < width; x += 1) {
    stack.push(x, (height - 1) * width + x);
  }
  for (let y = 0; y < height; y += 1) {
    stack.push(y * width, y * width + width - 1);
  }

  while (stack.length > 0) {
    const index = stack.pop();
    if (index < 0 || index >= total || cleared[index] === 1 || !isPaper(index)) {
      continue;
    }
    cleared[index] = 1;
    data[index * 4 + 3] = 0;
    const x = index % width;
    const y = Math.floor(index / width);
    if (x > 0) stack.push(index - 1);
    if (x < width - 1) stack.push(index + 1);
    if (y > 0) stack.push(index - width);
    if (y < height - 1) stack.push(index + width);
  }

  // Feather the boundary so the plate has no white halo.
  for (let index = 0; index < total; index += 1) {
    if (cleared[index] === 1) continue;
    const x = index % width;
    const y = Math.floor(index / width);
    const touchesPaper =
      (x > 0 && cleared[index - 1] === 1) ||
      (x < width - 1 && cleared[index + 1] === 1) ||
      (y > 0 && cleared[index - width] === 1) ||
      (y < height - 1 && cleared[index + width] === 1);
    if (!touchesPaper) continue;
    const offset = index * 4;
    const luminance = (data[offset] + data[offset + 1] + data[offset + 2]) / 3;
    if (luminance > 120) {
      const alpha = Math.round((255 * (255 - luminance)) / (255 - 120));
      data[offset + 3] = Math.min(data[offset + 3], Math.max(0, alpha));
    }
  }

  await sharp(data, { raw: { width, height, channels: 4 } })
    .resize(1024, 1024, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
    .png()
    .toFile(output);
  console.log(`knockout done: ${output} (${width}x${height} -> 1024)`);
})();
