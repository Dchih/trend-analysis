from src.runtime import CollectorRuntime


class CollectorWorker:
    def __init__(self, runtime: CollectorRuntime) -> None:
        self.runtime = runtime

    def process(self, task: dict) -> object | None:
        platform = task["platform"]
        return self.runtime.resolve_plugin(platform)
