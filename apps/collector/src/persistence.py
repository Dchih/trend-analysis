class InMemoryStore:
    def __init__(self) -> None:
        self._creators: dict[tuple[str, str], dict] = {}
        self._content_items: dict[tuple[str, str], dict] = {}

    def upsert_creators(self, creators: list[dict]) -> None:
        for creator in creators:
            key = (creator["platform"], creator["platform_creator_id"])
            self._creators[key] = creator

    def upsert_content_items(self, content_items: list[dict]) -> None:
        for content_item in content_items:
            key = (content_item["platform"], content_item["platform_content_id"])
            self._content_items[key] = content_item

    def creators(self) -> list[dict]:
        return list(self._creators.values())

    def content_items(self) -> list[dict]:
        return list(self._content_items.values())
