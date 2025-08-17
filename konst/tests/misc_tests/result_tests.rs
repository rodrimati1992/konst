use konst::result;


#[test]
fn unwrap_or_test() {
    // Necessary for type inference reasons.
    type Res = Result<u32, u32>;
    
    const ARR: &[u32] = &[
        result::unwrap_or!(Res::Ok(3), 5),
        result::unwrap_or!(Res::Err(8), 13),
    ];
    
    assert_eq!(ARR, &[3, 13]);
}

#[test]
fn unwrap_or_else_test() {
    // Necessary for type inference reasons.
    type Res = Result<u32, u32>;
    
    const ARR: &[u32] = &[
        result::unwrap_or_else!(Res::Ok(3), |_| loop{}),
        result::unwrap_or_else!(Res::Err(8), |x| x + 5),
    
        result::unwrap_or_else!(Res::Ok(21), add_34),
        result::unwrap_or_else!(Res::Err(55), add_34),
    ];
    
    assert_eq!(ARR, &[3, 13, 21, 89]);
    
    const fn add_34(n: u32) -> u32 {
        n + 34
    }
}

#[test]
fn unwrap_err_or_else_test() {
    // Necessary for type inference reasons.
    type Res = Result<u16, u32>;
    
    const ARR: &[u32] = &[
        result::unwrap_err_or_else!(Res::Ok(3), |x| (x + 2) as u32),
        result::unwrap_err_or_else!(Res::Err(8), |_| loop{}),
    
        result::unwrap_err_or_else!(Res::Ok(16), add_34),
        result::unwrap_err_or_else!(Res::Err(55), add_34),
    ];
    
    assert_eq!(ARR, &[5, 8, 50, 55]);
    
    const fn add_34(n: u16) -> u32 {
        (n + 34) as u32
    }
}

#[test]
fn ok_test() {
    // Necessary for type inference reasons.
    type Res = Result<u32, u32>;
    
    const ARR: &[Option<u32>] = &[
        result::ok!(Res::Ok(3)),
        result::ok!(Res::Err(8)),
    ];
    
    assert_eq!(ARR, &[Some(3), None]);
    
}

#[test]
fn err_test() {
    // Necessary for type inference reasons.
    type Res = Result<u32, u32>;
    
    const ARR: &[Option<u32>] = &[
        result::err!(Res::Ok(3)),
        result::err!(Res::Err(8)),
    ];
    
    assert_eq!(ARR, &[None, Some(8)]);
}

#[test]
fn map_test() {
    // Necessary for type inference reasons.
    type Res = Result<u32, u32>;
    
    const ARR: &[Res] = &[
        result::map!(Res::Ok(3), |x| x + 2),
        result::map!(Res::Err(8), |_| loop{}),
    
        result::map!(Res::Ok(16), add_34),
        result::map!(Res::Err(55), add_34),
    ];
    
    assert_eq!(ARR, &[Ok(5), Err(8), Ok(50), Err(55)]);
    
    const fn add_34(n: u32) -> u32 {
        n + 34
    }
}

#[test]
fn map_err_test() {
    // Necessary for type inference reasons.
    type Res = Result<u32, u32>;
    
    const ARR: &[Res] = &[
        result::map_err!(Res::Ok(3), |_| loop{}),
        result::map_err!(Res::Err(8), |x| x + 5),
    
        result::map_err!(Res::Ok(16), add_34),
        result::map_err!(Res::Err(55), add_34),
    ];
    
    assert_eq!(ARR, &[Ok(3), Err(13), Ok(16), Err(89)]);
    
    const fn add_34(n: u32) -> u32 {
        n + 34
    }
}

#[test]
fn and_then_test() {
    // Necessary for type inference reasons.
    type Res = Result<u32, u32>;
    
    const ARR: &[Res] = &[
        result::and_then!(Res::Ok(1), |x| Ok(x + 2)),
        result::and_then!(Res::Ok(10), |x| Err(x + 4)),
        result::and_then!(Res::Err(20), |_| loop{}),
    
        result::and_then!(Res::Ok(40), add_2),
        result::and_then!(Res::Ok(40), add_5),
        result::and_then!(Res::Err(60), add_5),
    ];
    
    assert_eq!(ARR, &[Ok(3), Err(14), Err(20), Ok(42), Err(45), Err(60)]);
    
    const fn add_2(n: u32) -> Res {
        Ok(n + 2)
    }
    const fn add_5(n: u32) -> Res {
        Err(n + 5)
    }
}

#[test]
fn or_else_test() {
    // Necessary for type inference reasons.
    type Res = Result<u32, u32>;
    
    const ARR: &[Res] = &[
        result::or_else!(Res::Ok(1), |_| loop{}),
        result::or_else!(Res::Err(20), |x| Ok(x + 5)),
        result::or_else!(Res::Err(20), |x| Err(x + 7)),
    
        result::or_else!(Res::Ok(40), add_2),
        result::or_else!(Res::Err(60), add_2),
        result::or_else!(Res::Err(60), add_5),
    ];
    
    assert_eq!(ARR, &[Ok(1), Ok(25), Err(27), Ok(40), Ok(62), Err(65)]);
    
    const fn add_2(n: u32) -> Res {
        Ok(n + 2)
    }
    const fn add_5(n: u32) -> Res {
        Err(n + 5)
    }
}


