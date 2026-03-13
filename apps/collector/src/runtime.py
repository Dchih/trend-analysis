from src.registry import PluginRegistry


class CollectorRuntime:
    def __init__(self, registry: PluginRegistry) -> None:
        self.registry = registry

    def resolve_plugin(self, platform: str) -> object | None:
        return self.registry.get(platform)
