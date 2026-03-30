import re

def process_file(file_path):
    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()

    # Match `env.panic_with_error` with whitespace/newlines
    content = re.sub(
        r"env\.panic_with_error\(\s*Error::from_contract_error\(\s*ContractError::(\w+)\s*as\s*u32,?\s*\)\s*\)",
        r"return Err(ContractError::\1)",
        content,
    )

    # Convert `.unwrap_or_else(|| { return Err(ContractError::XYZ); })` into `.ok_or(ContractError::XYZ)?`
    content = re.sub(
        r"\.unwrap_or_else\(\|\|\s*\{\s*return Err\(ContractError::(\w+)\);\s*\}\)",
        r".ok_or(ContractError::\1)?",
        content,
    )

    with open(file_path, "w", encoding="utf-8") as f:
        f.write(content)

process_file("contracts/atomic_swap/src/lib.rs")
print("Done")
