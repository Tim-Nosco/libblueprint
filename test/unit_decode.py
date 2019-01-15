#!env python3

def hook(l=None):
    if l!=None:
        locals().update(l)
    import IPython
    IPython.embed(banner1="", confirm_exit=False)
    exit(0)


def test1(libblueprint):
    data = """0eJydWduOmzAQ/Rc/Q4WvJPmV1aoixN1aAoPAVF2t8u8l4bJNdlhm/BQlJGcunnNmxvlg52qwbed8YKcP5srG9+z08sF69+aL6vZZeG8tOzEXbM0S5ov69q7oe1ufK+ff0roofztvU8GuCXP+Yv+yE7++Jsz64IKzE979zftPP9Rn241f+B4pYW3Tjz9u/M2DEdAk7J2dUjGa6Gzp7h7Zypaha7wr09J15eDC6MEXS2K19KvoQ+p8b7swPtgwIa8AhnzECF3h+7bpQnq2VfiKlB4mbzWEpYiRp/kdjNMj1zSvp/jVaOfiRkvTEwHgmkfcYTzy7q1rxtedfNyw52pqhtAOoNf5il7bixvqdAp2DLVtKguA6wmcQ8k+UF3N5yJYHXV+w88jsSbypSYe8msAZJ5hS3aOPds/M85p7uZzMUBQT4zq28qFDfd+aDBqBcFKdNT5ZzntgSo0qFlAIRhNJa38dHFlbdm0re3SsjiPdQxZoRNLbxHrITMZZIzKM/UNzTiZZ9mEJp+OEKEP/Ig9UgWRAyKcyCLdF1vub6qGiGt8EeovRFxQmzFtH4mQcZoN1pIgN0i5+o2mmtBkqnFYdBBHbogBiQjtEDmxZ4j9niHIrJ6TlEWU0BHbVBShp0gysecIOPmUJaeaUl+0ewLfjwo90upveppE99sMrHxISKWKGTmfTxKqRUntwIIuCtLQMoLoLBJNS4OeaeQhqnODYivRzTSDeAGFrLKYSXNjV+K0Xi8Q7qHJsyi+3K9OJankz7cGNwhdkVXsgF5jlKbmA9E5FLXlLSN9xH6vclq9zbkBNVEdiMnA5AJNsWWaRxScxq+Iy0YLoaDptbiGiFfjCWbWKt2RPE0dMLNlWdndg3TkuJlySmvRxLuYdNm/d/VM05dGDqYdIUQ6xw5p6+aP6Gf6QMtNhq6aIzk1GtbN3SoysRvk8xC7L9iGeJOzyDZiXDYiasiiUcGQe6V6PhTojgOMB337o0DlhfhmNJYDCk8Bg548FajDoKPo0VPDhHpNpjv/039/ESTsj+36yaZSwphccHG8Xv8Bty8XdA=="""
    a = libblueprint.decode(data)


if __name__=="__main__":
    from ctypes import cdll
    libblueprint = cdll.LoadLibrary("target/debug/libblueprint.so")
    test1(libblueprint)
    print("done!")