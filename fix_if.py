#!/usr/bin/env python3
import re
import sys


def fix_if_statements(filepath):
    with open(filepath, "r") as f:
        content = f.read()

    # 修复 If (FUNCTION(...) { 为 If (FUNCTION(...)) {
    patterns = [
        (r"If \((CONTAINS\([^)]+\)) \{", r"If (\1) {"),
        (r"If \((HAS_KEY\([^)]+\)) \{", r"If (\1) {"),
        (r"If \((STARTS_WITH\([^)]+\)) \{", r"If (\1) {"),
        (r"If \((REGEX_WILDCARD_MATCH\([^)]+\)) \{", r"If (\1) {"),
        (r"If \((REGEX_IS_DIGIT\([^)]+\)) \{", r"If (\1) {"),
        (r"If \((REGEX_IS_ALPHA\([^)]+\)) \{", r"If (\1) {"),
        (r"If \((REGEX_IS_EMAIL\([^)]+\)) \{", r"If (\1) {"),
        (r"If \((REGEX_IS_URL\([^)]+\)) \{", r"If (\1) {"),
    ]

    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content)

    with open(filepath, "w") as f:
        f.write(content)

    print(f"Fixed {filepath}")


if __name__ == "__main__":
    files = [
        "stdlib/cli_utils.aether",
        "stdlib/text_template.aether",
        "stdlib/regex_utils.aether",
    ]
    for f in files:
        fix_if_statements(f)
