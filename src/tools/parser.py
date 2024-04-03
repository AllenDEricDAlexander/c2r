import re


def parser_content(resp) -> str:
    if resp.find('```') != -1:
        return re.findall(r"```rust([\s\S]*?)```", resp)[0]
    else:
        return resp.replace('`', '')
