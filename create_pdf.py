#!/usr/bin/env python3
import os
import subprocess
import sys

def create_pdf_from_html():
    """Create PDF from HTML using system tools"""
    
    # Check if we have Chrome/Chromium available
    chrome_paths = [
        '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
        '/Applications/Chromium.app/Contents/MacOS/Chromium',
        'google-chrome',
        'chromium-browser'
    ]
    
    chrome_path = None
    for path in chrome_paths:
        if os.path.exists(path) or subprocess.run(['which', path.split('/')[-1]], capture_output=True).returncode == 0:
            chrome_path = path
            break
    
    if chrome_path:
        print("Using Chrome to convert HTML to PDF...")
        try:
            # Use Chrome headless to convert HTML to PDF
            cmd = [
                chrome_path,
                '--headless',
                '--disable-gpu',
                '--print-to-pdf=Crisis_Companion_Updated_Overview.pdf',
                '--print-to-pdf-no-header',
                'file://' + os.path.abspath('Crisis_Companion_Updated_Overview.html')
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            if result.returncode == 0:
                print("✅ PDF created successfully!")
                return True
            else:
                print(f"❌ Error creating PDF: {result.stderr}")
                return False
                
        except Exception as e:
            print(f"❌ Error: {e}")
            return False
    else:
        print("❌ Chrome/Chromium not found. Trying alternative method...")
        return create_pdf_alternative()

def create_pdf_alternative():
    """Alternative method using pandoc if available"""
    try:
        # Try using pandoc
        result = subprocess.run(['pandoc', 'Crisis_Companion_Updated_Overview.md', '-o', 'Crisis_Companion_Updated_Overview.pdf'], capture_output=True, text=True)
        
        if result.returncode == 0:
            print("✅ PDF created successfully using pandoc!")
            return True
        else:
            print(f"❌ Pandoc error: {result.stderr}")
            return False
            
    except FileNotFoundError:
        print("❌ Pandoc not found. Creating a simple text-based PDF...")
        return create_simple_pdf()

def create_simple_pdf():
    """Create a simple PDF using reportlab"""
    try:
        from reportlab.pdfgen import canvas
        from reportlab.lib.pagesizes import letter
        from reportlab.lib.units import inch
        
        # Read the markdown content
        with open('Crisis_Companion_Updated_Overview.md', 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Create PDF
        c = canvas.Canvas("Crisis_Companion_Updated_Overview.pdf", pagesize=letter)
        width, height = letter
        
        # Set font and size
        c.setFont("Helvetica-Bold", 16)
        c.drawString(1*inch, height-1*inch, "Crisis Companion - Solana Mobile Hackathon Submission")
        
        c.setFont("Helvetica", 12)
        y_position = height - 1.5*inch
        
        # Split content into lines and add to PDF
        lines = content.split('\n')
        for line in lines:
            if line.strip():
                if line.startswith('#'):
                    # Headers
                    c.setFont("Helvetica-Bold", 14)
                    line = line.replace('#', '').strip()
                elif line.startswith('- ') or line.startswith('• '):
                    # Bullet points
                    c.setFont("Helvetica", 11)
                    line = "  " + line
                else:
                    # Regular text
                    c.setFont("Helvetica", 11)
                
                # Check if we need a new page
                if y_position < 1*inch:
                    c.showPage()
                    y_position = height - 1*inch
                
                # Draw the line
                c.drawString(0.5*inch, y_position, line[:80])  # Limit line length
                y_position -= 0.2*inch
        
        c.save()
        print("✅ Simple PDF created successfully!")
        return True
        
    except ImportError:
        print("❌ ReportLab not available. Installing...")
        subprocess.run([sys.executable, '-m', 'pip', 'install', 'reportlab'])
        return create_simple_pdf()

if __name__ == "__main__":
    print("🔄 Creating PDF from HTML...")
    success = create_pdf_from_html()
    
    if success:
        print("🎉 PDF creation completed!")
        print("📄 File: Crisis_Companion_Updated_Overview.pdf")
    else:
        print("❌ Failed to create PDF. Please try manual conversion.") 