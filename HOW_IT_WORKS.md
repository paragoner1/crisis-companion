# How Solana SOS Works Under the Hood

## Emergency Flow
1. Voice input captured.
2. Processed through Whisper for transcription.
3. AI analyzes (MobileBERT/T5).
4. Generates guidance, triggers actions.

## Technical Details
- Inference: ORT runs models on-device.
- Security: SHA256 verification.
