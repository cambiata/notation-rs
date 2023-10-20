use crate::prelude::*;

pub fn matrix_test3() -> RMatrix {
    let col0 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            //
        ],
        None,
        None,
    );

    let col1 = RCol::new(
        vec![
            qitem(0.0, 10.0, NV4),
            qitem(0.0, 10.0, NV4),
            //
        ],
        Some(NV4),
        None,
    );

    let col2 = RCol::new(
        vec![
            qitem(0.0, 10.0, NV4),
            qitem(0.0, 38.0, NV4),
            //
        ],
        Some(NV4),
        None,
    );

    let col3 = RCol::new(
        vec![
            qitem(0.0, 10.0, NV8),
            qitem(0.0, 10.0, NV8),
            //
        ],
        Some(NV8),
        None,
    );
    let col4 = RCol::new(
        vec![
            qitem(0.0, 10.0, NV8),
            qitem(0.0, 10.0, NV8),
            //
        ],
        Some(NV8),
        None,
    );

    let col5 = RCol::new(
        vec![
            xitem(0.0, 10.0, 20.0, 0),
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            //
        ],
        None,
        None,
    );

    let matrix = RMatrix::new(
        vec![
            Rc::new(RefCell::new(col0)),
            Rc::new(RefCell::new(col1)),
            Rc::new(RefCell::new(col2)),
            Rc::new(RefCell::new(col3)),
            Rc::new(RefCell::new(col4)),
            Rc::new(RefCell::new(col5)),
        ],
        None,
    );

    matrix
}

pub fn matrix_test1() -> RMatrix {
    let col0 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            //
        ],
        None,
        None,
    );
    let col1 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV1)))),
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV4)))),
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV2)))),
            //
        ],
        Some(NV4),
        None,
    );
    let col2 = RCol::new(
        vec![
            None, //
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV2DOT)))),
            None, //
        ],
        Some(NV4),
        None,
    );
    let col3 = RCol::new(
        vec![
            None, //
            None, //
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV2)))),
        ],
        Some(NV2),
        None,
    );

    let matrix = RMatrix::new(
        vec![Rc::new(RefCell::new(col0)), Rc::new(RefCell::new(col1)), Rc::new(RefCell::new(col2)), Rc::new(RefCell::new(col3))],
        None,
    );

    matrix
}

pub fn matrix_test2() -> RMatrix {
    let col0 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            //
        ],
        None,
        None,
    );
    let col1 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(vec![NRect::new(0.0, 0.0, 10.0, 10.0)], NV2)))),
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV4)))),
            //
        ],
        Some(NV4),
        None,
    );
    let col2 = RCol::new(
        vec![
            None, //
            Some(Rc::new(RefCell::new(RItem::new(vec![NRect::new(0.0, 0.0, 10.0, 5.0)], NV4)))),
        ],
        Some(NV4),
        None,
    );
    let col3 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(vec![NRect::new(-0.0, 0.0, 20.0, 20.0)], 0)))),
            // Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            None, //
        ],
        None,
        None,
    );

    let col4 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(vec![NRect::new(0.0, 0.0, 10.0, 30.0)], NV2)))),
            Some(Rc::new(RefCell::new(RItem::new(vec![NRect::new(0.0, 0.0, 10.0, 5.0)], NV4)))),
            //
        ],
        Some(NV2),
        None,
    );

    let col5 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(vec![NRect::new(0.0, 0.0, 5.0, 20.0)], 0)))),
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
        ],
        None,
        None,
    );

    let matrix = RMatrix::new(
        vec![
            Rc::new(RefCell::new(col0)),
            Rc::new(RefCell::new(col1)),
            Rc::new(RefCell::new(col2)),
            Rc::new(RefCell::new(col3)),
            Rc::new(RefCell::new(col4)),
            Rc::new(RefCell::new(col5)),
        ],
        None,
    );

    matrix
}

pub fn qitem(x: f32, w: f32, dur: Duration) -> Option<Rc<RefCell<RItem>>> {
    // Some(Rc::new(RefCell::new(RItem::new(
    //     vec![NRect::new(x, 0.0, w, 10.0)],
    //     dur,
    // ))))

    Some(Rc::new(RefCell::new(RItem::new_with_nrectsext(vec![NRect::new(x, 0.0, w, 10.0)], dur))))
}

pub fn xitem(x: f32, w: f32, h: f32, dur: Duration) -> Option<Rc<RefCell<RItem>>> {
    // Some(Rc::new(RefCell::new(RItem::new(
    //     vec![NRect::new(x, 0.0, w, h)],
    //     dur,
    // ))))
    Some(Rc::new(RefCell::new(RItem::new_with_nrectsext(vec![NRect::new(x, 0.0, w, h)], dur))))
}

pub fn r10() -> Vec<NRect> {
    vec![NRect::new(0.0, 0.0, 10.0, 10.0)]
}

pub fn r20() -> Vec<NRect> {
    vec![NRect::new(0.0, 0.0, 10.0, 20.0)]
}
