# Backup Strategy for Crisis Companion Enhanced

## **🔒 Current Backup Locations**

### **Local Backups (Safe)**
- `/Users/ryanomeara/projects/crisis-companion-enhanced-backup-20250828-224644/` - Latest timestamped backup
- `/Users/ryanomeara/projects/crisis-companion-backup-20250826/` - Previous backup
- `/Users/ryanomeara/projects/crisis-companion-original/` - Original version

### **Current Working Directory**
- `/Users/ryanomeara/projects/crisis-companion-enhanced/` - **ACTIVE WORKING VERSION**

## **📋 Before Making Changes**

### **Always Create Backup First**
```bash
# Navigate to projects directory
cd /Users/ryanomeara/projects

# Create timestamped backup
cp -r crisis-companion-enhanced crisis-companion-enhanced-backup-$(date +%Y%m%d-%H%M%S)
```

### **Check Current Status**
```bash
# Verify compilation
cargo check --lib --features "voice,monitoring,private,rodio"

# Check git status
git status
```

## **📁 Project Organization**

### **Key Directories**
- **Working Version**: `crisis-companion-enhanced/` - Always work here
- **Backups**: `crisis-companion-enhanced-backup-YYYYMMDD-HHMMSS/`
- **Original**: `crisis-companion-original/` - Reference only

### **Important Files**
- `PROJECT_STATUS.md` - Complete project documentation
- `QUICK_START.md` - Quick reference guide
- `src/` - All source code
- `private/emergency.db` - SQLite database

## **🚨 Emergency Recovery**

### **If Work is Lost**
1. **Check backups**: Look for timestamped backup folders
2. **Restore from backup**: `cp -r crisis-companion-enhanced-backup-YYYYMMDD-HHMMSS/ crisis-companion-enhanced/`
3. **Verify compilation**: `cargo check`
4. **Check git status**: `git status`

### **If Git Repository is Corrupted**
1. **Use local backup**: Copy from timestamped backup
2. **Reinitialize git**: `git init && git add . && git commit -m "Restored from backup"`
3. **Push to new repository**: Create new GitHub repository

## **💡 Best Practices**

### **Before Each Session**
1. **Create backup**: Always backup before starting work
2. **Check status**: Verify current project state
3. **Read documentation**: Review `PROJECT_STATUS.md`

### **During Development**
1. **Test frequently**: `cargo check` after each change
2. **Commit often**: Small, frequent commits
3. **Document changes**: Update `PROJECT_STATUS.md`

### **After Each Session**
1. **Create final backup**: Timestamped backup
2. **Commit and push**: Save to git repository
3. **Update documentation**: Keep `PROJECT_STATUS.md` current

## **🎯 Quick Commands**

### **Daily Workflow**
```bash
# Start work
cd /Users/ryanomeara/projects/crisis-companion-enhanced

# Create backup
cd .. && cp -r crisis-companion-enhanced crisis-companion-enhanced-backup-$(date +%Y%m%d-%H%M%S)

# Return to work
cd crisis-companion-enhanced

# Check status
cargo check --lib --features "voice,monitoring,private,rodio"
```

### **Emergency Commands**
```bash
# List all backups
ls -la | grep crisis-companion

# Restore from backup
cp -r crisis-companion-enhanced-backup-YYYYMMDD-HHMMSS/ crisis-companion-enhanced/

# Check compilation
cargo check --lib --features "voice,monitoring,private,rodio"
```

## **⚠️ Important Notes**

- **Never work directly in backup folders** - Always work in `crisis-companion-enhanced/`
- **Always backup before major changes** - Timestamped backups are your safety net
- **Keep documentation updated** - `PROJECT_STATUS.md` is your project memory
- **Test frequently** - Compilation errors are easier to fix when caught early

**Remember**: This is a world-class emergency response system. Protect your work like lives depend on it! 🚀
