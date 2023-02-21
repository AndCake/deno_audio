import { hasStopped, init, play } from "./mod.ts";

init();
const idx = await play("music.mp3");
let i = 0;
do {
  Deno.stdout.writeSync(
    new TextEncoder().encode(
      "\r" + "|/-\\"[i % 4],
    ),
  );
  await new Promise((resolve) => setTimeout(resolve, 100));
  i += 1;
} while (!hasStopped(idx));
console.log("Music stopped.");
