"""
This script fetches all lexicons from the atproto repository and saves them to
the atproto-core/lexicons directory.
"""

import base64
import json
from pathlib import Path
from github import Github

def content_at_path(repo, path):
    contents = repo.get_contents(path)
    files = set()
    for content_file in contents:
        if content_file.type == "dir":
            files.update(content_at_path(repo, content_file.path))
        elif content_file.type == "file":
            files.add((content_file.path, content_file.encoding, content_file.content))
    return files

def main():
    github_client = Github()
    repo = github_client.get_repo("bluesky-social/atproto")
    files = content_at_path(repo, "lexicons")

    for (file_path, content_type, encoded_content) in files:
        if content_type != "base64":
            print(f"Skipping {file_path} because it is not base64 encoded")
            continue

        decoded_content = base64.standard_b64decode(encoded_content).decode("utf-8")
        lexicon = json.loads(decoded_content)
        lexicon_id = lexicon.get("id", "unknown")
        if lexicon_id == "unknown":
            print(f"Skipping {file_path} because it does not have an id")
            continue

        destination = Path('.') / "atproto-core" / "lexicons" / f"{lexicon_id}.json"
        with open(destination, "w") as f:
            f.write(decoded_content)

if __name__ == "__main__":
    main()
