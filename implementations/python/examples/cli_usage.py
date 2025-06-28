#!/usr/bin/env python3
"""Command-line interface usage examples."""

import subprocess
import tempfile
import json
from pathlib import Path

def create_sample_files():
    """Create sample JYAML files for CLI demonstration."""
    # Valid JYAML file
    valid_content = '''
    # Sample configuration
    {
      "app": {
        "name": "MyApp",
        "version": "1.0.0"
      },
      "server": {
        "host": "localhost",
        "port": 8080,
        "ssl": false
      },
      "features": [
        "authentication",
        "logging"
      ]
    }
    '''
    
    # Invalid JYAML file
    invalid_content = '''
    {
      "invalid": 
      "missing": value
    }
    '''
    
    # Block-style JYAML
    block_content = '''
    "name": "Block Style Example"
    "description": |
      This is a multiline
      description that preserves
      line breaks
    "config": {
      "debug": true,
      "timeout": 30
    }
    '''
    
    files = {}
    for name, content in [
        ("valid.jyml", valid_content),
        ("invalid.jyml", invalid_content),
        ("block.jyml", block_content)
    ]:
        with tempfile.NamedTemporaryFile(mode='w', suffix='.jyml', delete=False) as f:
            f.write(content.strip())
            files[name] = f.name
    
    return files

def run_cli_command(args, input_data=None):
    """Run CLI command and return result."""
    cmd = ["uv", "run", "python", "-m", "jyaml"] + args
    result = subprocess.run(
        cmd,
        input=input_data,
        capture_output=True,
        text=True
    )
    return result

def demonstrate_parsing():
    """Demonstrate file parsing."""
    print("=== CLI Parsing Examples ===")
    
    files = create_sample_files()
    
    try:
        # Parse valid file
        print("1. Parse valid JYAML file:")
        result = run_cli_command([files["valid.jyml"]])
        if result.returncode == 0:
            output = json.loads(result.stdout)
            print(f"   Success: {output['app']['name']} v{output['app']['version']}")
        else:
            print(f"   Error: {result.stderr}")
        print()
        
        # Parse block-style file
        print("2. Parse block-style JYAML:")
        result = run_cli_command([files["block.jyml"]])
        if result.returncode == 0:
            output = json.loads(result.stdout)
            print(f"   Success: {output['name']}")
            print(f"   Description: {repr(output['description'][:30])}...")
        else:
            print(f"   Error: {result.stderr}")
        print()
        
        # Parse invalid file
        print("3. Parse invalid JYAML file:")
        result = run_cli_command([files["invalid.jyml"]])
        if result.returncode != 0:
            print(f"   Expected error: {result.stderr.strip()}")
        else:
            print("   Unexpected success")
        print()
        
    finally:
        # Clean up
        for temp_file in files.values():
            Path(temp_file).unlink()

def demonstrate_validation():
    """Demonstrate file validation."""
    print("=== CLI Validation Examples ===")
    
    files = create_sample_files()
    
    try:
        # Validate valid file
        print("1. Validate valid file:")
        result = run_cli_command(["--validate", files["valid.jyml"]])
        print(f"   Result: {result.stdout.strip()}")
        print(f"   Exit code: {result.returncode}")
        print()
        
        # Validate invalid file
        print("2. Validate invalid file:")
        result = run_cli_command(["--validate", files["invalid.jyml"]])
        print(f"   Result: {result.stderr.strip()}")
        print(f"   Exit code: {result.returncode}")
        print()
        
    finally:
        # Clean up
        for temp_file in files.values():
            Path(temp_file).unlink()

def demonstrate_stdin():
    """Demonstrate reading from stdin."""
    print("=== CLI Stdin Examples ===")
    
    # Simple JSON input
    print("1. Parse JSON from stdin:")
    json_input = '{"message": "Hello from stdin", "count": 42}'
    result = run_cli_command([], input_data=json_input)
    if result.returncode == 0:
        output = json.loads(result.stdout)
        print(f"   Success: {output['message']}")
    else:
        print(f"   Error: {result.stderr}")
    print()
    
    # JYAML with comments from stdin
    print("2. Parse JYAML with comments from stdin:")
    jyaml_input = '''
    # Configuration from stdin
    {
      "source": "stdin",
      "processed": true
    }
    '''
    result = run_cli_command([], input_data=jyaml_input)
    if result.returncode == 0:
        output = json.loads(result.stdout)
        print(f"   Success: source={output['source']}, processed={output['processed']}")
    else:
        print(f"   Error: {result.stderr}")
    print()

def demonstrate_help():
    """Show help information."""
    print("=== CLI Help ===")
    result = run_cli_command(["--help"])
    print(result.stdout)

def main():
    print("JYAML Command-Line Interface Examples")
    print("=" * 40)
    print()
    
    demonstrate_parsing()
    demonstrate_validation()
    demonstrate_stdin()
    demonstrate_help()

if __name__ == "__main__":
    main()