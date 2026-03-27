import re


ALERT_START_RE = re.compile(r"^>\s*\[!(NOTE|TIP|IMPORTANT|WARNING|CAUTION)\]\s*(.*)$")
ALERT_MAP = {
    "NOTE": ("note", None),
    "TIP": ("tip", None),
    "IMPORTANT": ("warning", "Important"),
    "WARNING": ("warning", None),
    "CAUTION": ("warning", "Caution"),
}


def _transform_github_alerts(lines):
    output = []
    index = 0

    while index < len(lines):
        match = ALERT_START_RE.match(lines[index])
        if not match:
            output.append(lines[index])
            index += 1
            continue

        alert_type, custom_title = match.groups()
        admonition_type, default_title = ALERT_MAP[alert_type]
        title = custom_title.strip() or default_title

        if title:
            output.append(f'!!! {admonition_type} "{title}"')
        else:
            output.append(f"!!! {admonition_type}")

        index += 1
        while index < len(lines):
            line = lines[index]
            if line.startswith(">"):
                body = line[1:]
                if body.startswith(" "):
                    body = body[1:]
                if body:
                    output.append(f"    {body}")
                else:
                    output.append("")
                index += 1
                continue

            if line == "":
                output.append("")
                index += 1
                break

            break

    return output


def on_page_markdown(markdown, **kwargs):
    return "\n".join(_transform_github_alerts(markdown.splitlines()))