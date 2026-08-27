# Open Media Player

## Introduction

When I was just 6 years old, I used my dad's computer for the first time, back then the **orange and white traffic cone**
logo of VLC media player always made me curious as to what exactly does the app do? Cause let me be honest, a traffic 
cone isn't the first thing which pops up in anybody's mind when they think of a media player.

Anyways, enough dilly dallying. After all those years, now that I am a VLC user myself, I had a thought, _Why not build one myself?_
So to put my skills to test and to enhance them, I will be building my own media player using rust.

## Development

I have decided that while the project is in it's developmental phase, I will be following the following roadmap:

1. **Phase 1 (CLI Phase)**: Make a simple Rust CLI program using ffmpeg-next that opens an .mp4 file and prints its duration, resolution, frame rate, and raw packet counts to the terminal.
2. **Phase 2 (Audio Only)**: Extract audio frames using ffmpeg-next, pass them to cpal or rodio, and get sound playing out of our speakers smoothly.
3. **Phase 3 (Video Only)**: Decode video frames, upload the pixel buffer to a wgpu texture, and display it on a winit window.
4. **Phase 4 (Synchronization)**: Create a shared master clock so the video thread drops or delays frames to stay locked with the audio thread.