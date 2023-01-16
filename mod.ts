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
    result: "void",
    nonblocking: true,
  },
  pause: { parameters: ["i32"], result: "void", nonblocking: false },
  set_volume: {
    parameters: ["i32", "f32"],
    result: "void",
    nonblocking: false,
  },
});
export function play(a0: string, volume: number) {
  const a0_buf = new TextEncoder().encode(a0);
  // this is very instable at the moment... needs to be replaced with a better library
  const a0_ptr = Deno.UnsafePointer.of(a0_buf);
  deno_audio.symbols.play(a0_ptr, a0_buf.length, volume);
  return audioCounter++;
}
export function pause(idx: number) {
  deno_audio.symbols.pause(idx);
}
export function setVolume(idx: number, new_volume: number) {
  deno_audio.symbols.set_volume(idx, new_volume);
}
