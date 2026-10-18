    }
    assert!(wheel(10.0, 99, 480.0).is_none());
    assert_eq!(distance(f64::MAX, 2), distance(1_000.0, 0));
    assert_eq!(distance(-f64::MAX, 2), distance(-1_000.0, 0));
}

