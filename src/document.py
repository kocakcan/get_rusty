class Document:
    def __init__(self, words: list[str]):
        """Create a new document"""
        self.words = words

    def add_word(self, word: str):
        """Add a word to the document"""
        self.words.append(word)

    def get_words(self) -> list[str]:
        """Get a list of all the words in the document"""
        return self.words


def main():
    words = ["Hello"]
    d = Document(words)

    d2 = Document(d.get_words())
    d2.add_word("world")


if __name__ == "__main__":
    main()
