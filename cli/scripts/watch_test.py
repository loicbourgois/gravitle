import io
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import watch


class WatchTests(unittest.TestCase):
    def test_start_launches_worker_through_uv_and_publishes_pid(self):
        with tempfile.TemporaryFile(mode="w+") as lock:
            with (
                patch.object(watch.subprocess, "Popen") as popen,
                patch("sys.stdout", new_callable=io.StringIO),
            ):
                popen.return_value.pid = 12345

                result = watch.start(lock)

            self.assertEqual(result, 0)
            popen.assert_called_once_with(
                [
                    "uv",
                    "run",
                    "--no-project",
                    "python",
                    "-u",
                    str(Path(watch.__file__).resolve()),
                    "--worker-fd",
                    str(lock.fileno()),
                ],
                cwd=self.cli_dir,
                stdin=watch.subprocess.DEVNULL,
                stdout=watch.subprocess.DEVNULL,
                stderr=watch.subprocess.DEVNULL,
                start_new_session=True,
                pass_fds=(lock.fileno(),),
            )

            lock.seek(0)
            self.assertEqual(lock.read(), "12345")
            self.assertTrue(watch.LOG_PATH.is_file())
    
    def test_generated_todo_outputs_do_not_trigger_rebuilds(self):
        specifications = self.cli_dir / "specifications"
        specifications.mkdir()
        specification = specifications / "tasks.yml"
        specification.write_text("models: []\n")
        previous = watch.snapshot()

        outputs = [
            specifications / "tasks.yml.t.p.md",
            specifications / "tasks.yml.t.r.gpt6.1sol.md",
            specifications / "tasks.yml.t.r.claudeopus5.5.md",
        ]

        for output in outputs:
            output.write_text("generated")
        self.assertEqual(watch.snapshot(), previous)

        for output in outputs:
            output.write_text("updated generated content")
        self.assertEqual(watch.snapshot(), previous)

        for output in outputs:
            output.unlink()
        self.assertEqual(watch.snapshot(), previous)

        specification.write_text("models: [provider/model]\n")
        self.assertNotEqual(watch.snapshot(), previous)

    def test_generated_todo_output_matching_is_specific(self):
        for name in (
            "tasks.yml.t.p.md",
            "tasks.yml.t.r.gpt6.1sol.md",
            "tasks.yml.t.r.model123.md",
        ):
            with self.subTest(name=name):
                self.assertTrue(watch.is_generated_todo_output(name))

        for name in (
            "tasks.yml",
            "todo_prompt.md",
            "tasks.yml.t.p.md.backup",
            "tasks.yml.t.r.model.md.backup",
            "tasks.yml.t.r..md",
        ):
            with self.subTest(name=name):
                self.assertFalse(watch.is_generated_todo_output(name))

    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.cli_dir = Path(self.directory.name)

        for name, value in (
            ("CLI_DIR", self.cli_dir),
            ("LOG_PATH", self.cli_dir / "logs" / "watch.log"),
        ):
            patcher = patch.object(watch, name, value)
            patcher.start()
            self.addCleanup(patcher.stop)

    def test_logs_keep_latest_100_lines_in_order_with_utc_timestamps(self):
        for index in range(105):
            watch.log_message(f"message {index}")

        lines = watch.LOG_PATH.read_text().splitlines()

        self.assertEqual(len(lines), 100)
        for index, line in enumerate(lines, start=5):
            self.assertRegex(
                line,
                r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ",
            )
            self.assertEqual(line.split(" ", 1)[1], f"message {index}")

    def test_each_line_of_multiline_messages_is_timestamped(self):
        watch.log_message("first\nsecond\n")

        lines = watch.LOG_PATH.read_text().splitlines()

        self.assertEqual(len(lines), 2)
        self.assertTrue(lines[0].endswith("Z first"))
        self.assertTrue(lines[1].endswith("Z second"))

    def test_generated_files_do_not_trigger_rebuilds(self):
        source = self.cli_dir / "main.rs"
        source.write_text("original")
        previous = watch.snapshot()

        watch.log_message("generated output")
        for name in ("target", "__pycache__", ".git"):
            directory = self.cli_dir / name
            directory.mkdir()
            (directory / "generated").write_text("ignored")

        self.assertEqual(watch.snapshot(), previous)

        source.write_text("changed")
        self.assertNotEqual(watch.snapshot(), previous)

    def test_failed_installation_output_is_logged(self):
        with patch.object(watch.subprocess, "Popen") as popen:
            process = popen.return_value.__enter__.return_value
            process.stdout = io.StringIO("compiler diagnostic\n")
            process.wait.return_value = 1

            watch.install()

        content = watch.LOG_PATH.read_text()
        self.assertIn("compiler diagnostic", content)
        self.assertIn(
            "Build/install failed; waiting for another change.",
            content,
        )

    def test_successful_installation_is_logged(self):
        with patch.object(watch.subprocess, "Popen") as popen:
            process = popen.return_value.__enter__.return_value
            process.stdout = io.StringIO("installation output\n")
            process.wait.return_value = 0

            watch.install()

        self.assertIn(
            "Installed g and completions. Watching for changes...",
            watch.LOG_PATH.read_text(),
        )
    
    def test_log_paths_use_tilde_without_rewriting_similar_prefixes(self):
        with patch.object(watch.Path, "home", return_value=Path("/Users/test")):
            watch.log_message(
                "Installed /Users/test/.local/bin/g; "
                "home /Users/test; "
                "unrelated /Users/test-other/file"
            )

        content = watch.LOG_PATH.read_text()
        self.assertIn("Installed ~/.local/bin/g", content)
        self.assertIn("home ~;", content)
        self.assertIn("/Users/test-other/file", content)
    
    def test_standalone_home_path_uses_tilde(self):
        with patch.object(watch.Path, "home", return_value=Path("/Users/test")):
            watch.log_message("Home: /Users/test")

        self.assertTrue(watch.LOG_PATH.read_text().endswith("Home: ~\n"))

    def test_retained_log_paths_are_normalized(self):
        watch.LOG_PATH.parent.mkdir(parents=True)
        watch.LOG_PATH.write_text(
            "2026-11-07T13:39:19Z Installed /Users/test/.local/bin/g\n"
        )

        with patch.object(watch.Path, "home", return_value=Path("/Users/test")):
            watch.log_message("Watching for changes...")

        lines = watch.LOG_PATH.read_text().splitlines()
        self.assertEqual(
            lines[0],
            "2026-11-07T13:39:19Z Installed ~/.local/bin/g",
        )
        self.assertTrue(lines[1].endswith("Z Watching for changes..."))
    
    def test_install_uses_script_under_cli_root(self):
        with patch.object(watch.subprocess, "Popen") as popen:
            process = popen.return_value.__enter__.return_value
            process.stdout = io.StringIO("")
            process.wait.return_value = 0

            watch.install()

        args, kwargs = popen.call_args
        self.assertEqual(
            args,
            (["zsh", str(self.cli_dir / "scripts" / "install.sh")],),
        )
        self.assertEqual(kwargs["cwd"], self.cli_dir)
        self.assertEqual(kwargs["stdout"], watch.subprocess.PIPE)
        self.assertEqual(kwargs["stderr"], watch.subprocess.STDOUT)


class WatchPathsTests(unittest.TestCase):
    def test_cli_root_is_parent_of_scripts_directory(self):
        cli_dir = Path(watch.__file__).resolve().parent.parent

        self.assertEqual(watch.CLI_DIR, cli_dir)
        self.assertEqual(watch.LOG_PATH, cli_dir / "logs" / "watch.log")
        self.assertTrue((cli_dir / "Cargo.toml").is_file())
        self.assertTrue((cli_dir / "scripts" / "install.sh").is_file())

if __name__ == "__main__":
    unittest.main()