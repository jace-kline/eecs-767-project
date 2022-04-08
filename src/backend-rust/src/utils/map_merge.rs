use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MapMergeResult<K,L,R> 
where 
    K: Eq + Ord + Clone + Debug, 
    L: Clone, 
    R: Clone,
{
    Left(K, L),
    Right(K, R),
    Conflict(K, L, R)
}

pub fn merge_maps<K,L,R>(ml: &BTreeMap<K,L>, mr: &BTreeMap<K,R>) -> Vec<MapMergeResult<K,L,R>>
where 
    K: Eq + Ord + Clone + Debug, 
    L: Clone, 
    R: Clone,
{
    let mut v : Vec<MapMergeResult<K,L,R>> = Vec::new();

    let mut iterl = ml.iter();
    let mut iterr = mr.iter();

    let mut optl = iterl.next();
    let mut optr = iterr.next();

    loop {
        let res : Option<MapMergeResult<K,L,R>> = match (optl, optr) {
            (Some((kl, vl)), Some((kr, vr))) => {
                if kl == kr {
                    optl = iterl.next();
                    optr = iterr.next();
                    Some(MapMergeResult::Conflict(kl.clone(), vl.clone(), vr.clone()))
                }
                else if kl < kr {
                    optl = iterl.next();
                    Some(MapMergeResult::Left(kl.clone(), vl.clone()))
                }
                // kl > kr
                else {
                    optr = iterr.next();
                    Some(MapMergeResult::Right(kr.clone(), vr.clone()))
                }
            }
            (Some((kl, vl)), None) => {
                optl = iterl.next();
                Some(MapMergeResult::Left(kl.clone(), vl.clone()))
            }
            (None, Some((kr, vr))) => {
                optr = iterr.next();
                Some(MapMergeResult::Right(kr.clone(), vr.clone()))
            }
            (None, None) => None
        };

        if let Some(res) = res { 
            v.push(res)
        } else {
            break;
        }
    }

    v
}

#[test]
fn test_merge_maps() {
    let ml = BTreeMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("e", 5),
        ("g", 7),
        ("i", 9),
    ]);

    let mr = BTreeMap::from([
        ("b", 2),
        ("d", 3),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
    ]);

    let merge_result = merge_maps(&ml, &mr);

    for res in merge_result {
        println!("{:?}", res);
    }
}


