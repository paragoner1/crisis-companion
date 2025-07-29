# Crisis Companion - Voice-Activated Emergency Response App

**Solana Mobile Hackathon Submission** | Built for Solana Mobile Seeker

## 🚨 The Problem

Last summer, my 4-year-old son forgot his lifejacket and almost drowned at the lake. I performed CPR without knowing what I was doing. That moment made me realize we need a tool that guides people through emergencies when they're in shock or not thinking clearly.

**The Reality:**
- 236,000 people die from drowning annually
- Average emergency response times: 8-12 minutes
- People panic and forget basic procedures
- Remote areas often lack internet connectivity
- Traditional apps require manual activation and internet

## 💡 Our Solution

**Crisis Companion** uses offline voice recognition to detect emergency phrases like "Drowning help!" and immediately initiates a comprehensive emergency response.

### Key Features:
- **Voice-Activated**: "Drowning help!" triggers immediate response
- **Offline-First**: Works without internet connectivity
- **Auto Volume**: Sets phone to 100% during emergency
- **Step-by-Step**: Clear instructions for any emergency
- **Automatic Actions**: Calls 911, shares location, records audio
- **Multi-Device**: BLE coordination between devices
- **Solana Blockchain**: Audio hash storage for verification
- **Remote Area Focus**: Perfect for beaches, hiking, boating, camping

## 🛠️ Technical Innovation

Built in **Rust** for high performance and memory safety:
- **Vosk Voice Recognition**: Offline speech detection
- **SQLite Database**: Local emergency instructions
- **Bluetooth Low Energy**: Multi-device coordination
- **Solana Blockchain**: Tamper-proof emergency data storage
- **Android JNI**: Mobile integration

## 🚀 Quick Demo

```bash
# Clone the repository
git clone https://github.com/paragoner1/crisis-companion.git
cd crisis-companion

# Install dependencies
cargo build

# Run the demo
cargo run --bin demo_test
```

## 📱 Solana Mobile Integration

Currently developing for Solana Mobile Seeker deployment:
- Android native interface implementation
- Solana Mobile Stack integration
- Mobile Wallet Adapter
- dApp Store compatibility

## 🎯 Market Opportunity

**Target Markets:**
- Remote areas with poor connectivity
- Adventure sports enthusiasts
- Rural communities
- Natural disaster scenarios
- Boating and water activities

**Business Model:**
- Freemium: $5-10/month per user
- Enterprise: $50,000/year per client
- 2027 Target: 15,000 users + 15 enterprise clients

## 📊 Demo Results

The demo shows all core functionality working:
✅ Voice trigger detection  
✅ Emergency response initiation  
✅ Audio management  
✅ Database operations  
✅ UI emergency display  
✅ Blockchain integration  
✅ Device coordination  

## 🔐 Privacy & Security

- Audio recordings encrypted with AES-GCM
- Location data only shared during emergencies
- Blockchain storage for tamper-proof records
- HIPAA/GDPR compliance for medical data

## 🚀 Roadmap

- **August 2024**: Hackathon submission
- **January 2025**: MVP launch
- **April 2025**: Beta testing
- **July 2025**: Public launch
- **2027**: $1.26M revenue target

## 📞 Contact

- **GitHub**: [@paragoner1](https://github.com/paragoner1)
- **Project**: [Crisis Companion](https://github.com/paragoner1/crisis-companion)

---

**⚠️ Emergency Notice**: This software is designed for emergency response. Always call 911 for life-threatening emergencies. This app is a supplement to, not a replacement for, professional emergency services. 