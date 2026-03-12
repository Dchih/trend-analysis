class PluginRegistry:
    def __init__(self) -> None:
        self._plugins: dict[str, object] = {}

    def register(self, plugin: object) -> None:
        platform = getattr(plugin, "platform", None)
        if not platform:
            raise ValueError("plugin must declare a platform")
        self._plugins[platform] = plugin

    def get(self, platform: str) -> object | None:
        return self._plugins.get(platform)
