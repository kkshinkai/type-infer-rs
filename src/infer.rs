// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::BTreeMap;

struct InferCtxt;

// NOTE: Don't public this trait, `BTreeMap<_, _>` does not need this ad-hoc
// union method.
trait Union {
    fn union(&self, other: &Self) -> Self;
}

impl<K, V> Union for BTreeMap<K, V>
    where K: Clone + Ord,
          V: Clone,
{
    fn union(&self, other: &Self) -> BTreeMap<K, V> {
        let mut unioned = self.clone();
        for (key, value) in other {
            unioned.entry(key.clone()).or_insert(value.clone());
        }
        unioned
    }
}
