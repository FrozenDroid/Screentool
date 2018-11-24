# Screentool
CLI tool that lets you make screenshots/recordings (with audio) of any size and at any position.

# Dependencies
* FFmpeg
* (optional) Pulseaudio, alsa, or JACK for audio
* (optional) If you want to use hardware acceleration for screen recordings, you have to have a FFmpeg build compiled with support for NVENC or VA-API.

# Usage
Screenshot saved as a PNG file  
```
screentool -t png -s 1920,1080 screenshot.png
``` 
  
Screenrecording with VA-API hardware acceleration  
```
screentool -t mp4 -a vaapi -s 1920,1080 recording.mp4
```

