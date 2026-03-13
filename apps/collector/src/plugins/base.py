class BasePlugin:
    platform = ""

    def capabilities(self) -> dict:
        return {"platform": self.platform}

    def search(self, keyword: str, time_range: str) -> list[dict]:
        raise NotImplementedError

    def normalize(self, raw_items: list[dict], keyword_id: int) -> dict:
        raise NotImplementedError
