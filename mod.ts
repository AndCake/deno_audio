const OS_PREFIX = Deno.build.os === "windows" ? "" : "lib";
const OS_SUFFIX = Deno.build.os === "windows"
  ? ".dll"
  : Deno.build.os === "darwin"
  ? ".dylib"
  : ".so";

function getLibraryPath(lib: string): string {
  lib = `${OS_PREFIX}${lib}${OS_SUFFIX}`;
  const libPath = "./lib";
  if (libPath) {
    return `${libPath}/${lib}`;
  } else {
    return lib;
  }
}

let audioCounter = 0;
const deno_audio = Deno.dlopen(getLibraryPath("deno_audio"), {
  play: {
    parameters: ["pointer", "u32", "f32"],
    result: "i32",
    nonblocking: true,
  },
  pause: { parameters: ["i32"], result: "void", nonblocking: false },
  set_volume: {
    parameters: ["i32", "f32"],
    result: "void",
    nonblocking: false,
  },
});

/**
 * plays the given file with the given volume. 
 * volume is a value between 0 (muted) and 1 (full volume). If no volume is given, 1 is assumed.
 *
 * this function returns the audio index with which to pause it or change the volume.
 */
export function play(a0: string, volume: number): number {
  const a0_buf = new TextEncoder().encode(a0);
  const a0_ptr = Deno.UnsafePointer.of(a0_buf);
  // explicitly a pointer - not a buffer!
  deno_audio.symbols.play(a0_ptr, a0_buf.length, volume || 1);
  return audioCounter++;
}

/**
 * pauses the audio based on the given audio index (can be retrieved using `play()`.
 */
export function pause(idx: number) {
  deno_audio.symbols.pause(idx);
}

/**
 * sets the volume of the sound (based on the audio index). The 
 * new_volume parameter can assume values between 0 (muted) and 1 (full volume).
 */
export function setVolume(idx: number, new_volume: number) {
  deno_audio.symbols.set_volume(idx, new_volume);
}
