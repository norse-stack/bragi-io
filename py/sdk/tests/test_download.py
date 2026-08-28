"""Test platform detection and download logic (mocked HTTP)."""

from __future__ import annotations

import os
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

from bragi._download import (
    _PLATFORM_MAP,
    _detect_platform,
    find_or_download_cli,
    get_jre_dir,
)
from bragi.errors import BragiNotFoundError


class TestDetectPlatform:
    """Test platform detection maps to correct release asset names."""

    @patch("bragi._download.platform.system", return_value="Darwin")
    @patch("bragi._download.platform.machine", return_value="arm64")
    def test_macos_arm64(self, _m, _s) -> None:
        assert _detect_platform() == ("aarch64-apple-darwin", ".tar.gz")

    @patch("bragi._download.platform.system", return_value="Darwin")
    @patch("bragi._download.platform.machine", return_value="x86_64")
    def test_macos_x86_unsupported(self, _m, _s) -> None:
        with pytest.raises(BragiNotFoundError, match="Unsupported platform"):
            _detect_platform()

    @patch("bragi._download.platform.system", return_value="Linux")
    @patch("bragi._download.platform.machine", return_value="x86_64")
    def test_linux_x86(self, _m, _s) -> None:
        assert _detect_platform() == ("x86_64-unknown-linux-gnu", ".tar.gz")

    @patch("bragi._download.platform.system", return_value="Windows")
    @patch("bragi._download.platform.machine", return_value="AMD64")
    def test_windows_amd64(self, _m, _s) -> None:
        assert _detect_platform() == ("x86_64-pc-windows-msvc", ".zip")

    @patch("bragi._download.platform.system", return_value="FreeBSD")
    @patch("bragi._download.platform.machine", return_value="x86_64")
    def test_unsupported_platform(self, _m, _s) -> None:
        with pytest.raises(BragiNotFoundError, match="Unsupported platform"):
            _detect_platform()


class TestFindOrDownloadCli:
    """Test CLI discovery with various environment states."""

    @patch.dict(os.environ, {"BRAGI_CLI_PATH": "/usr/local/bin/bragi"})
    @patch("bragi._download.Path.exists", return_value=True)
    def test_env_var_found(self, _exists) -> None:
        result = find_or_download_cli()
        assert str(result) == "/usr/local/bin/bragi"

    @patch.dict(os.environ, {"BRAGI_CLI_PATH": "/nonexistent/bragi"})
    def test_env_var_not_found(self) -> None:
        with pytest.raises(BragiNotFoundError, match="BRAGI_CLI_PATH"):
            find_or_download_cli()

    @patch.dict(os.environ, {}, clear=True)
    @patch("bragi._download.shutil.which", return_value=None)
    def test_falls_through_to_download(self, _which, tmp_path: Path) -> None:
        """When nothing is found locally, it attempts download."""
        # Remove BRAGI_CLI_PATH from env
        os.environ.pop("BRAGI_CLI_PATH", None)

        with patch("bragi._download._BIN_DIR", tmp_path / "bin"):
            with patch(
                "bragi._download._detect_platform",
                side_effect=BragiNotFoundError("test: unsupported"),
            ):
                with pytest.raises(BragiNotFoundError, match="Could not find"):
                    find_or_download_cli()

    @patch.dict(os.environ, {}, clear=True)
    @patch("bragi._download.shutil.which", return_value="/usr/bin/bragi")
    def test_found_on_path(self, _which, tmp_path: Path) -> None:
        os.environ.pop("BRAGI_CLI_PATH", None)
        with patch("bragi._download._BIN_DIR", tmp_path / "bin"):
            result = find_or_download_cli()
            assert str(result) == "/usr/bin/bragi"


class TestGetJreDir:
    """JRE resolution: a real JRE or nothing — never an empty directory."""

    @patch.dict(os.environ, {}, clear=True)
    def test_no_jre_anywhere_is_none(self, tmp_path: Path) -> None:
        # Clear the environment so JAVA_HOME (set on dev machines with a JDK,
        # e.g. via sdkman) doesn't short-circuit the case we're testing. The
        # CLI never downloads into an explicit --jre-path, so an empty dir
        # here would break first use on a JVM-less machine (the 0.6.0 bug).
        jre_dir = tmp_path / "runtime" / "jre"
        with patch("bragi._download._JRE_DIR", jre_dir):
            assert get_jre_dir() is None
            assert not jre_dir.exists()

    @patch.dict(os.environ, {}, clear=True)
    def test_packaged_jre_is_used(self, tmp_path: Path) -> None:
        jre_dir = tmp_path / "runtime" / "jre"
        (jre_dir / "bin").mkdir(parents=True)
        (jre_dir / "bin" / "java").touch()
        with patch("bragi._download._JRE_DIR", jre_dir):
            assert get_jre_dir() == jre_dir

    def test_java_home_wins(self, tmp_path: Path) -> None:
        java_home = tmp_path / "jdk"
        java_home.mkdir()
        with patch.dict(os.environ, {"JAVA_HOME": str(java_home)}, clear=True):
            assert get_jre_dir() == java_home
