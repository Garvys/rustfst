from rustfst import VectorFst, Tr
from rustfst.algorithms.compose import ComposeFilter, ComposeConfig

# from rustfst import DrawingConfig


def test_compose_fst():
    # FST 1
    fst1 = VectorFst()

    s1 = fst1.add_state()
    s2 = fst1.add_state()
    s3 = fst1.add_state()

    fst1.set_start(s1)
    fst1.set_final(s2)
    fst1.set_final(s3)

    tr1_1 = Tr(1, 2, 1.0, s2)
    fst1.add_tr(s1, tr1_1)

    tr1_2 = Tr(1, 4, 2.0, s3)
    fst1.add_tr(s1, tr1_2)

    tr1_3 = Tr(3, 5, 2.0, s2)
    fst1.add_tr(s2, tr1_3)

    # d = DrawingConfig()
    # fst1.draw("composition_1.dot", None, None, d)

    # FST 2
    fst2 = VectorFst()

    s1 = fst2.add_state()
    s2 = fst2.add_state()
    s3 = fst2.add_state()

    fst2.set_start(s1)
    fst2.set_final(s3)

    tr2_1 = Tr(2, 6, 1.0, s2)
    fst2.add_tr(s1, tr2_1)

    tr2_2 = Tr(5, 7, 2.5, s3)
    fst2.add_tr(s2, tr2_2)

    tr2_3 = Tr(5, 8, 1.5, s3)
    fst2.add_tr(s3, tr2_3)

    tr2_4 = Tr(4, 9, 3.0, s3)
    fst2.add_tr(s1, tr2_4)

    # fst2.draw("composition_2.dot", None, None, d)

    # Expected FST
    expected_fst = VectorFst()
    s1 = expected_fst.add_state()
    s2 = expected_fst.add_state()
    s3 = expected_fst.add_state()
    s4 = expected_fst.add_state()

    expected_fst.set_start(s1)
    expected_fst.set_final(s3)
    expected_fst.set_final(s4)

    tr3_1 = Tr(1, 6, 2.0, s2)
    expected_fst.add_tr(s1, tr3_1)

    tr3_2 = Tr(1, 9, 5.0, s3)
    expected_fst.add_tr(s1, tr3_2)

    tr3_3 = Tr(3, 7, 4.5, s4)
    expected_fst.add_tr(s2, tr3_3)

    tr3_4 = Tr(3, 8, 3.5, s4)
    expected_fst.add_tr(s4, tr3_4)

    fst3 = fst1.compose(fst2)
    # fst3.draw("composition_res.dot", None, None, d)

    assert fst3 == expected_fst


def test_compose_config():
    compose_filter = ComposeFilter.TRIVIALFILTER
    compose_config = ComposeConfig(compose_filter, True)  # Checked on debug print

    # FST 1
    fst1 = VectorFst()

    s1 = fst1.add_state()
    s2 = fst1.add_state()
    s3 = fst1.add_state()

    fst1.set_start(s1)
    fst1.set_final(s2)
    fst1.set_final(s3)

    tr1_1 = Tr(1, 2, 1.0, s2)
    fst1.add_tr(s1, tr1_1)

    tr1_2 = Tr(1, 4, 2.0, s3)
    fst1.add_tr(s1, tr1_2)

    tr1_3 = Tr(3, 5, 2.0, s2)
    fst1.add_tr(s2, tr1_3)

    # d = DrawingConfig()
    # fst1.draw("composition_1.dot", None, None, d)

    # FST 2
    fst2 = VectorFst()

    s1 = fst2.add_state()
    s2 = fst2.add_state()
    s3 = fst2.add_state()

    fst2.set_start(s1)
    fst2.set_final(s3)

    tr2_1 = Tr(2, 6, 1.0, s2)
    fst2.add_tr(s1, tr2_1)

    tr2_2 = Tr(5, 7, 2.5, s3)
    fst2.add_tr(s2, tr2_2)

    tr2_3 = Tr(5, 8, 1.5, s3)
    fst2.add_tr(s3, tr2_3)

    tr2_4 = Tr(4, 9, 3.0, s3)
    fst2.add_tr(s1, tr2_4)

    # fst2.draw("composition_2.dot", None, None, d)

    # Expected FST
    expected_fst = VectorFst()
    s1 = expected_fst.add_state()
    s2 = expected_fst.add_state()
    s3 = expected_fst.add_state()
    s4 = expected_fst.add_state()

    expected_fst.set_start(s1)
    expected_fst.set_final(s3)
    expected_fst.set_final(s4)

    tr3_1 = Tr(1, 6, 2.0, s2)
    expected_fst.add_tr(s1, tr3_1)

    tr3_2 = Tr(1, 9, 5.0, s3)
    expected_fst.add_tr(s1, tr3_2)

    tr3_3 = Tr(3, 7, 4.5, s4)
    expected_fst.add_tr(s2, tr3_3)

    tr3_4 = Tr(3, 8, 3.5, s4)
    expected_fst.add_tr(s4, tr3_4)

    fst3 = fst1.compose(fst2, compose_config)
    assert fst3 == expected_fst