from src.registry import PluginRegistry


class DummyPlugin:
    platform = "dummy"


def test_registry_loads_registered_plugin_and_rejects_unknown_platform():
    registry = PluginRegistry()
    registry.register(DummyPlugin())

    plugin = registry.get("dummy")

    assert plugin.platform == "dummy"
    assert registry.get("unknown") is None
