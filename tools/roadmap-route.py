"""Select an explicit approved-roadmap execution route without executing it."""

import argparse
import json


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Select the source and action for an approved roadmap."
    )
    parser.add_argument("--mode", required=True, choices=("classic", "mission-backed"))
    parser.add_argument("--dossier", required=True)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    route = {
        "mode": args.mode,
        "source": "dossier" if args.mode == "classic" else "mission",
        "dossier": args.dossier,
        "action": "execute-dossier" if args.mode == "classic" else "migrate-dossier",
    }
    print(json.dumps(route))


if __name__ == "__main__":
    main()
