import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SELECTOR = Path(__file__).resolve().parents[1] / "tools" / "roadmap-route.py"


class RoadmapRouteTest(unittest.TestCase):
    def run_selector(self, *args, env=None):
        return subprocess.run(
            [sys.executable, str(SELECTOR), *args],
            capture_output=True,
            check=False,
            encoding="utf-8",
            env=env,
        )

    def test_classic_uses_dossier_without_invoking_fake_migrator(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            dossier = root / "approved-dossier"
            dossier.mkdir()
            marker = root / "migrator-invoked"
            fake_migrator = root / "migrator.py"
            fake_migrator.write_text(
                "from pathlib import Path\n"
                f"Path({str(marker)!r}).write_text('invoked')\n"
                "raise SystemExit('migration must not be invoked')\n",
                encoding="utf-8",
            )
            environment = os.environ.copy()
            environment["ROADMAP_MIGRATOR"] = str(fake_migrator)

            result = self.run_selector(
                "--mode", "classic", "--dossier", str(dossier), env=environment
            )

            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(
                json.loads(result.stdout),
                {
                    "mode": "classic",
                    "source": "dossier",
                    "dossier": str(dossier),
                    "action": "execute-dossier",
                },
            )
            self.assertFalse(marker.exists(), "classic mode invoked the fake migrator")

    def test_mission_backed_selects_existing_migration_handoff(self):
        result = self.run_selector(
            "--mode", "mission-backed", "--dossier", "transcripts/approved"
        )

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(
            json.loads(result.stdout),
            {
                "mode": "mission-backed",
                "source": "mission",
                "dossier": "transcripts/approved",
                "action": "migrate-dossier",
            },
        )

    def test_missing_or_invalid_mode_fails_closed(self):
        for arguments in (
            ("--dossier", "transcripts/approved"),
            ("--mode", "automatic", "--dossier", "transcripts/approved"),
        ):
            with self.subTest(arguments=arguments):
                result = self.run_selector(*arguments)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(result.stdout, "")
