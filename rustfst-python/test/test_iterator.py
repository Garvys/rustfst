from rustfst import Fst, TrsIterator, StateIterator, Tr
import pytest

def test_trs_iterator():
    
    fst = Fst()
    s1 = fst.add_state()
    s2 = fst.add_state()
    fst.set_start(s1)
    fst.set_final(s2, 0.54)
    tr1 = Tr(1, 18, 2.33, s2)
    fst.add_tr(s1, tr1)

    trs_it = TrsIterator(fst, s1)
    tr = next(trs_it)

    assert tr.ilabel == 1
    assert tr.olabel == 18
    assert tr.weight == pytest.approx(2.33)
    assert tr.next_state == s2

    assert trs_it.done()

    trs_it.reset()

    assert not trs_it.done()

    for tr in trs_it:
        assert tr == tr1

def test_state_iterator():
    
    fst = Fst()
    s1 = fst.add_state()
    s2 = fst.add_state()
    fst.set_start(s1)
    fst.set_final(s2, 0.54)
    tr1 = Tr(1, 18, 2.33, s2)
    fst.add_tr(s1, tr1)

    states_it = StateIterator(fst)

    for idx, state in enumerate(states_it):
        assert idx == state