# Emergency Instruction Audio Files

This directory contains audio files for emergency instructions. The files are organized by emergency type and instruction step.

## File Naming Convention
- `{emergency_type}_{step_number}_{description}.mp3`
- Example: `drowning_01_check_breathing.mp3`

## Emergency Types
- `drowning` - Drowning emergency instructions
- `fire` - Fire emergency instructions  
- `heart_attack` - Heart attack emergency instructions
- `choking` - Choking emergency instructions
- `bleeding` - Bleeding emergency instructions
- `unconscious` - Unconscious person instructions
- `seizure` - Seizure emergency instructions
- `allergic_reaction` - Allergic reaction instructions
- `poisoning` - Poisoning emergency instructions
- `trauma` - Trauma emergency instructions

## Current Status
- Audio files will be generated using TTS initially
- Pre-recorded MP3 files can be added later for better quality
- Files should be optimized for mobile playback (small file size)

## Usage
The Crisis Companion app will:
1. Detect emergency voice trigger
2. Set volume to 100%
3. Play appropriate emergency instructions
4. Provide step-by-step guidance
5. Auto-dial 911 when needed 