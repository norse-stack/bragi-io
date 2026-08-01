"""Exception hierarchy for Bragi SDK."""


class BragiError(Exception):
    """Base exception for all Bragi errors."""

    pass


class BragiAuthError(BragiError):
    """Raised on 401 responses (bad or missing API key)."""

    pass


class BragiCreditsError(BragiError):
    """Raised on 402 responses (insufficient credits)."""

    pass


class BragiProcessingError(BragiError):
    """Raised on 500 responses or CLI processing failures."""

    pass


class BragiNotFoundError(BragiError):
    """Raised when the bragi binary is not found (local mode only)."""

    pass
