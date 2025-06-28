#!/usr/bin/env python3
"""Type safety examples using Pydantic models."""

from typing import List, Optional
from pydantic import BaseModel, ValidationError
from jyaml import loads

# Define Pydantic models for type-safe data handling
class ServerConfig(BaseModel):
    host: str
    port: int
    ssl: bool = False

class DatabaseConfig(BaseModel):
    type: str
    host: str
    port: int
    name: str

class AppConfig(BaseModel):
    name: str
    version: str
    debug: bool = False
    server: ServerConfig
    database: Optional[DatabaseConfig] = None
    features: List[str] = []

def demonstrate_type_validation():
    """Show how to use Pydantic for type validation."""
    print("=== Type-Safe JYAML Parsing ===")
    
    # Valid configuration
    valid_config = '''
    {
      "name": "MyApp",
      "version": "1.0.0",
      "debug": true,
      "server": {
        "host": "localhost",
        "port": 8080,
        "ssl": false
      },
      "database": {
        "type": "postgresql",
        "host": "db.example.com",
        "port": 5432,
        "name": "myapp_db"
      },
      "features": [
        "authentication",
        "logging",
        "monitoring"
      ]
    }
    '''
    
    try:
        # Parse JYAML and validate with Pydantic
        raw_data = loads(valid_config)
        config = AppConfig(**raw_data)
        
        print("✓ Configuration loaded successfully:")
        print(f"  App: {config.name} v{config.version}")
        print(f"  Server: {config.server.host}:{config.server.port}")
        print(f"  Database: {config.database.type} at {config.database.host}")
        print(f"  Features: {', '.join(config.features)}")
        print()
        
    except ValidationError as e:
        print(f"✗ Validation error: {e}")
        print()

def demonstrate_type_errors():
    """Show type validation errors."""
    print("=== Type Validation Errors ===")
    
    # Invalid types
    invalid_configs = [
        # Wrong port type
        ('{"name": "App", "version": "1.0", "server": {"host": "localhost", "port": "8080"}}',
         "Port should be integer, not string"),
        
        # Missing required field
        ('{"name": "App", "version": "1.0"}',
         "Missing required server configuration"),
        
        # Invalid boolean
        ('{"name": "App", "version": "1.0", "debug": "yes", "server": {"host": "localhost", "port": 8080}}',
         "Debug should be boolean, not string"),
        
        # Invalid array element
        ('{"name": "App", "version": "1.0", "server": {"host": "localhost", "port": 8080}, "features": ["auth", 123]}',
         "Features should be array of strings"),
    ]
    
    for i, (config_str, description) in enumerate(invalid_configs, 1):
        print(f"{i}. {description}:")
        try:
            raw_data = loads(config_str)
            config = AppConfig(**raw_data)
            print("   ✗ Unexpected success")
        except ValidationError as e:
            print(f"   ✓ Validation caught error: {e.errors()[0]['msg']}")
        except Exception as e:
            print(f"   ✗ Parse error: {e}")
        print()

def demonstrate_partial_validation():
    """Show partial object validation."""
    print("=== Partial Validation ===")
    
    # Server config only
    server_only = '''
    {
      "host": "production.example.com",
      "port": 443,
      "ssl": true
    }
    '''
    
    try:
        raw_data = loads(server_only)
        server_config = ServerConfig(**raw_data)
        print("✓ Server configuration:")
        print(f"  Host: {server_config.host}")
        print(f"  Port: {server_config.port}")
        print(f"  SSL: {server_config.ssl}")
        print()
    except ValidationError as e:
        print(f"✗ Server validation error: {e}")
        print()

def demonstrate_optional_fields():
    """Show handling of optional fields."""
    print("=== Optional Fields ===")
    
    # Minimal configuration (database optional)
    minimal_config = '''
    {
      "name": "MinimalApp",
      "version": "0.1.0",
      "server": {
        "host": "127.0.0.1",
        "port": 3000
      }
    }
    '''
    
    try:
        raw_data = loads(minimal_config)
        config = AppConfig(**raw_data)
        
        print("✓ Minimal configuration loaded:")
        print(f"  App: {config.name} v{config.version}")
        print(f"  Debug: {config.debug} (default)")
        print(f"  Database: {config.database} (optional, not provided)")
        print(f"  Features: {config.features} (default empty list)")
        print()
        
    except ValidationError as e:
        print(f"✗ Validation error: {e}")
        print()

def demonstrate_nested_validation():
    """Show nested object validation with JYAML features."""
    print("=== Nested Validation with JYAML Features ===")
    
    # Configuration with comments and multiline strings
    complex_config = '''
    # Application configuration
    {
      "name": "ComplexApp",
      "version": "2.0.0",
      "description": |
        This is a complex application
        with multiple features and
        detailed configuration
      "server": {
        "host": "0.0.0.0", # Listen on all interfaces
        "port": 8080,
        "ssl": true
      },
      "database": {
        "type": "postgresql",
        "host": "localhost",
        "port": 5432,
        "name": "complex_app"
      },
      "features": [
        "authentication",
        "authorization",
        "logging",
        "monitoring",
        "caching"
      ]
    }
    '''
    
    try:
        raw_data = loads(complex_config)
        config = AppConfig(**raw_data)
        
        print("✓ Complex configuration with JYAML features:")
        print(f"  App: {config.name} v{config.version}")
        print(f"  Server SSL: {config.server.ssl}")
        print(f"  Database: {config.database.type}")
        print(f"  Feature count: {len(config.features)}")
        print()
        
    except ValidationError as e:
        print(f"✗ Validation error: {e}")
        print()

def main():
    print("JYAML Type Safety with Pydantic")
    print("=" * 35)
    print()
    
    demonstrate_type_validation()
    demonstrate_type_errors()
    demonstrate_partial_validation()
    demonstrate_optional_fields()
    demonstrate_nested_validation()

if __name__ == "__main__":
    main()