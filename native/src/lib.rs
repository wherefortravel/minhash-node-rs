#[macro_use]
extern crate neon;
extern crate twox_hash;

mod minhash;

use neon::prelude::*;
use minhash::{MinHash, LshIndex};

declare_types! {
  pub class JsMinHash for MinHash {
    init(mut cx) {
      Ok(MinHash::new(128))
    }

    method jaccard(mut cx) {
      Ok(cx.string("ok").upcast())
    }

    method update(mut cx) {
      Ok(cx.string("ok").upcast())
    }
  }

  pub class JsLshIndex for LshIndex {
    init(mut cx) {
      Ok(LshIndex::new(4))
    }

    method insert(mut cx) {
      Ok(cx.string("ok").upcast())
    }

    method query(mut cx) {
      Ok(cx.string("ok").upcast())
    }
  }
}

register_module!(mut m, {
  m.export_class::<JsMinHash>("MinHash")
});