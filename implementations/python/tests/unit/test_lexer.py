"""Tests for JYAML lexer."""

import pytest

from jyaml.lexer import Lexer, LexerError, TokenType


class TestLexer:
    """Test cases for JYAML lexer."""

    def test_empty_input(self):
        """Test empty input."""
        lexer = Lexer("")
        token = lexer.next_token()
        assert token.type == TokenType.EOF

    def test_null_literal(self):
        """Test null literal."""
        lexer = Lexer("null")
        token = lexer.next_token()
        assert token.type == TokenType.NULL
        assert token.value == "null"

    def test_boolean_literals(self):
        """Test boolean literals."""
        lexer = Lexer("true false")

        token = lexer.next_token()
        assert token.type == TokenType.TRUE
        assert token.value == "true"

        token = lexer.next_token()
        assert token.type == TokenType.FALSE
        assert token.value == "false"

    def test_numbers(self):
        """Test number literals."""
        test_cases = [
            ("0", "0"),
            ("42", "42"),
            ("-17", "-17"),
            ("3.14", "3.14"),
            ("-0.5", "-0.5"),
            ("1.23e4", "1.23e4"),
            ("2E-10", "2E-10"),
        ]

        for input_text, expected in test_cases:
            lexer = Lexer(input_text)
            token = lexer.next_token()
            assert token.type == TokenType.NUMBER
            assert token.value == expected

    def test_strings(self):
        """Test string literals."""
        test_cases = [
            ('"hello"', "hello"),
            ("'world'", "world"),
            ('"hello\\nworld"', "hello\nworld"),
            ('"unicode: \\u0041"', "unicode: A"),
            ('""', ""),
        ]

        for input_text, expected in test_cases:
            lexer = Lexer(input_text)
            token = lexer.next_token()
            assert token.type == TokenType.STRING
            assert token.value == expected

    def test_multiline_strings(self):
        """Test multiline string literals."""
        # Literal style
        input_text = """|\n  line1\n  line2"""
        lexer = Lexer(input_text)
        token = lexer.next_token()
        assert token.type == TokenType.STRING
        assert "line1" in token.value and "line2" in token.value

        # Folded style
        input_text = """>\n  line1\n  line2"""
        lexer = Lexer(input_text)
        token = lexer.next_token()
        assert token.type == TokenType.STRING

    def test_punctuation(self):
        """Test punctuation tokens."""
        test_cases = [
            (":", TokenType.COLON),
            (",", TokenType.COMMA),
            ("[", TokenType.LEFT_BRACKET),
            ("]", TokenType.RIGHT_BRACKET),
            ("{", TokenType.LEFT_BRACE),
            ("}", TokenType.RIGHT_BRACE),
            ("-", TokenType.DASH),
        ]

        for input_text, expected_type in test_cases:
            lexer = Lexer(input_text)
            token = lexer.next_token()
            assert token.type == expected_type
            assert token.value == input_text

    def test_comments(self):
        """Test comment tokens."""
        lexer = Lexer("# This is a comment")
        token = lexer.next_token()
        assert token.type == TokenType.COMMENT
        assert token.value == "This is a comment"

    def test_newlines_and_indentation(self):
        """Test newlines and indentation."""
        input_text = "\n  hello"
        lexer = Lexer(input_text)

        # First newline
        token = lexer.next_token()
        assert token.type == TokenType.NEWLINE

        # Indentation
        token = lexer.next_token()
        assert token.type == TokenType.INDENT
        assert token.value == "2"

    def test_bom_error(self):
        """Test BOM detection error."""
        with pytest.raises(LexerError, match="BOM not allowed"):
            Lexer("\ufeffhello")

    def test_tab_error(self):
        """Test tab character error."""
        with pytest.raises(LexerError, match="Tab character"):
            lexer = Lexer("\thello")
            lexer.next_token()

    def test_unterminated_string(self):
        """Test unterminated string error."""
        with pytest.raises(LexerError, match="Unterminated string"):
            lexer = Lexer('"hello')
            lexer.next_token()

    def test_invalid_escape(self):
        """Test invalid escape sequence error."""
        with pytest.raises(LexerError, match="Invalid escape sequence"):
            lexer = Lexer('"\\x"')
            lexer.next_token()

    def test_tokenize_complete(self):
        """Test complete tokenization."""
        input_text = '{"key": "value", "number": 42}'
        lexer = Lexer(input_text)

        tokens = list(lexer.tokenize())
        token_types = [token.type for token in tokens]

        expected_types = [
            TokenType.LEFT_BRACE,
            TokenType.STRING,
            TokenType.COLON,
            TokenType.STRING,
            TokenType.COMMA,
            TokenType.STRING,
            TokenType.COLON,
            TokenType.NUMBER,
            TokenType.RIGHT_BRACE,
            TokenType.EOF,
        ]

        assert token_types == expected_types
