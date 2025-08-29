# Crisis Companion Enhanced - Quick Start Guide

## **🚀 For Future Development Sessions**

### **Project Location**
```
/Users/ryanomeara/projects/crisis-companion-enhanced/
```

### **Backup Locations**
```
/Users/ryanomeara/projects/
├── crisis-companion-enhanced/                    # Main working directory
├── crisis-companion-enhanced-backup-20250828-224644/  # Latest backup
├── crisis-companion-backup-20250826/             # Previous backup
└── crisis-companion-original/                    # Original version
```

### **Quick Commands**

#### **Check Project Status**
```bash
cd /Users/ryanomeara/projects/crisis-companion-enhanced
cargo check --lib --features "voice,monitoring,private,rodio"
```

#### **Run Tests**
```bash
cargo test --lib --features "voice,monitoring,private,rodio"
```

#### **Build Project**
```bash
cargo build --lib --features "voice,monitoring,private,rodio"
```

### **Key Files to Know**

#### **Core Architecture**
- `src/lib.rs` - Main library interface
- `src/public/types.rs` - All data structures (15 emergency types)
- `src/public/emergency_interface.rs` - Smart 911 handoff system
- `src/public/audio_interface.rs` - Emergency volume management

#### **Private Implementation**
- `src/private/voice_recognition.rs` - Voice recognition with false positive prevention
- `src/private/emergency_database.rs` - SQLite database with protocols
- `src/private/role_detection.rs` - AI-powered role detection

#### **Documentation**
- `PROJECT_STATUS.md` - Complete project overview
- `README.md` - User-facing documentation
- `QUICK_START.md` - This file

### **Current Features Status**

#### **✅ Fully Implemented**
- **15 Emergency Protocols**: Including Suicide, Drug Overdose, Hypothermia
- **Advanced Audio Management**: Emergency volume override, Android speaker control
- **Smart 911 Handoff**: Intelligent timing, dispatcher context, audio monitoring
- **Voice Recognition**: Multi-layer false positive prevention
- **Database System**: SQLite with emergency protocols
- **AI Role Detection**: Sensor fusion and adaptive learning

#### **🔧 Technical Stack**
- **Language**: Rust
- **Database**: SQLite
- **Voice Recognition**: Vosk + RNNoise filtering
- **Audio Processing**: Advanced volume management
- **Blockchain**: Solana integration for token rewards

### **Development Workflow**

#### **Starting a New Session**
1. **Navigate to project**: `cd /Users/ryanomeara/projects/crisis-companion-enhanced`
2. **Check status**: `cargo check --lib --features "voice,monitoring,private,rodio"`
3. **Read documentation**: Review `PROJECT_STATUS.md` for context
4. **Make changes**: Edit files as needed
5. **Test changes**: Run compilation and tests
6. **Create backup**: Copy directory with timestamp before major changes

#### **Before Making Changes**
```bash
# Create backup
cp -r crisis-companion-enhanced crisis-companion-enhanced-backup-$(date +%Y%m%d-%H%M%S)

# Check current status
cargo check --lib --features "voice,monitoring,private,rodio"
```

#### **After Making Changes**
```bash
# Test compilation
cargo check --lib --features "voice,monitoring,private,rodio"

# Run tests
cargo test --lib --features "voice,monitoring,private,rodio"

# Update documentation
# Edit PROJECT_STATUS.md with new features/changes
```

### **Git Management**

#### **Current Repository Status**
- **Enhanced Version**: Ready to be made public
- **Original Version**: May need restoration from backup
- **Backup Strategy**: Multiple local backups created

#### **Recommended Git Strategy**
1. **Make enhanced version public** for hackathon
2. **Use enhanced version as primary** repository
3. **Keep original as backup** until hackathon judging complete

### **Hackathon Considerations**

#### **Timeline**
- **Judging Deadline**: ~1 week
- **Current State**: World-class implementation complete
- **Risk Mitigation**: Multiple backups created

#### **Submission Strategy**
- **Primary Repository**: Enhanced version (public)
- **Backup**: Multiple local backups
- **Documentation**: Complete project status in `PROJECT_STATUS.md`

### **Troubleshooting**

#### **Common Issues**
1. **Compilation Errors**: Check feature flags are correct
2. **Missing Dependencies**: Ensure all features are enabled
3. **Database Issues**: Check `private/emergency.db` exists

#### **Recovery Steps**
1. **Check backups**: Multiple timestamped backups available
2. **Restore from backup**: Copy backup directory if needed
3. **Review documentation**: `PROJECT_STATUS.md` has complete context

### **Key Insights for Future Development**

#### **Architecture Decisions**
- **Layered Design**: Public interfaces protect proprietary algorithms
- **Modular Structure**: 28 private modules for maintainability
- **Reliability-First**: Multiple fallback systems and error handling

#### **Technical Excellence**
- **Voice Recognition**: Advanced false positive prevention
- **Audio Management**: Complete device control during emergencies
- **911 Coordination**: Intelligent human handoff timing

### **Contact Information**
- **Project**: Crisis Companion Enhanced
- **Location**: `/Users/ryanomeara/projects/crisis-companion-enhanced/`
- **Documentation**: `PROJECT_STATUS.md` for complete context
- **Backups**: Multiple timestamped backups in parent directory

---

**Remember**: This is a world-class, reliability-first emergency response system. Always test thoroughly and maintain backups before making changes.
