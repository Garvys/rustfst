from rustfst import SymbolTable
from rustfst import VectorFst

EPS_SYMBOL = "<eps>"


def test_symt():
    symt = SymbolTable()
    symt.add_symbol("a")
    symt.add_symbol("b")

    assert symt.num_symbols() == 3

    assert symt.find(EPS_SYMBOL) == 0
    assert symt.find("a") == 1
    assert symt.find("b") == 2

    assert symt.member(EPS_SYMBOL) is True
    assert symt.member("a") is True
    assert symt.member("b") is True
    assert symt.member("c") is False

    assert symt.find(0) == EPS_SYMBOL
    assert symt.find(1) == "a"
    assert symt.find(2) == "b"

    assert symt.member(0) is True
    assert symt.member(1) is True
    assert symt.member(2) is True
    assert symt.member(3) is False


def test_symt_add_twice_symbol():
    symt = SymbolTable()
    symt.add_symbol("a")
    symt.add_symbol("a")

    assert symt.num_symbols() == 2
    assert symt.find("a") == 1


def test_add_table():
    symt1 = SymbolTable()
    symt1.add_symbol("a")
    symt1.add_symbol("b")

    symt2 = SymbolTable()
    symt2.add_symbol("c")
    symt2.add_symbol("b")

    symt1.add_table(symt2)

    assert symt1.num_symbols() == 4
    assert symt1.find(EPS_SYMBOL) == 0
    assert symt1.find("a") == 1
    assert symt1.find("b") == 2
    assert symt1.find("c") == 3


def test_eq_table():
    symt1 = SymbolTable()
    symt1.add_symbol("a")
    symt1.add_symbol("b")

    symt2 = SymbolTable()
    symt2.add_symbol("a")
    symt2.add_symbol("b")

    assert symt1 == symt2


def test_symt_iterator():
    symt = SymbolTable()
    symt.add_symbol("a")
    symt.add_symbol("b")

    assert list(symt) == [(0, "<eps>"), (1, "a"), (2, "b")]


def test_symt_copy_add():
    fst = VectorFst()
    symt = SymbolTable.from_symbols(["a", "b"])
    fst.set_input_symbols(symt)
    fst.set_output_symbols(symt)
    symt2 = fst.input_symbols().copy()
    symt2.add_symbol("c")
    assert symt2.num_symbols() == symt.num_symbols() + 1
