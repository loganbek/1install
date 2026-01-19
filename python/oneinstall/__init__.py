import subprocess
import sys
import os

def main():
    # Find the binary in the same directory as this script
    current_dir = os.path.dirname(os.path.realpath(__file__))
    bin_name = "1i"
    if os.name == "nt":
        bin_name += ".exe"
    
    bin_path = os.path.join(current_dir, bin_name)
    
    if os.path.exists(bin_path):
        # Forward all arguments to the Rust binary
        result = subprocess.run([bin_path] + sys.argv[1:])
        sys.exit(result.returncode)
    else:
        # Fallback if binary is not in the same dir (dev mode)
        print(f"Error: 1install binary not found at {bin_path}")
        sys.exit(1)

if __name__ == "__main__":
    main()
