#[macro_use]
extern crate neon;
extern crate twox_hash;

mod minhash;

use neon::prelude::*;
use minhash::{PermGen, MinHash, LshIndex};

// macro_rules! js_try {
//     ($ctx:expr, $res:expr) => {
//       match $res {
//         Ok(val) => val,
//         Err(err) => return $ctx.throw_error(format!("MinHash {}", err)),
//       }
//     };
// }

declare_types! {

  pub class JsPermGen for PermGen {
    init(mut cx) {
      let num_perm = cx.argument::<JsNumber>(0)?.value() as usize;

      Ok(PermGen::new(num_perm))
    }
  }

  pub class JsMinHash for MinHash {
    init(mut cx) {
      let perm_gen = cx.argument::<JsPermGen>(0)?;
      
      let ret_val = {
        let guard = cx.lock();
        let gen: &PermGen = & *perm_gen.borrow(&guard);

        MinHash::new(gen)
      };

      Ok(ret_val)
    }

    method jaccard(mut cx) {
      let other = cx.argument::<JsMinHash>(0)?;
      let this = cx.this();
      
      let val = {
        let guard = cx.lock();
        let hash = this.borrow(&guard);
        let other_val: &MinHash = & *other.borrow(&guard);
        hash.jaccard(other_val)
      };

      Ok(cx.number(val).upcast())
    }

    method update(mut cx) {
      let perm_gen = cx.argument::<JsPermGen>(0)?;
      let input = cx.argument::<JsString>(1)?.value();
      let mut this = cx.this();

      {
        let guard = cx.lock();
        let mut hash = this.borrow_mut(&guard);
        let gen: &PermGen = & *perm_gen.borrow(&guard);
        hash.update(gen, input);
      }

      Ok(JsUndefined::new().upcast())
    }
  }

  pub class JsLshIndex for LshIndex {
    init(cx) {
      Ok(LshIndex::new(4))
    }

    method insert(mut cx) {
      let key = cx.argument::<JsString>(0)?.value();
      let hash = cx.argument::<JsMinHash>(1)?;
      let mut this = cx.this();

      {
        let guard = cx.lock();
        let mut index = this.borrow_mut(&guard);
        let hash_val: &MinHash = & *hash.borrow(&guard);
        index.insert(key, hash_val);
      }

      Ok(JsUndefined::new().upcast())
    }

    method query(mut cx) {
      let hash = cx.argument::<JsMinHash>(0)?;
      let this = cx.this();

      let results = {
        let guard = cx.lock();
        let index = this.borrow(&guard);
        let hash_val: &MinHash = & *hash.borrow(&guard);
        index.query(hash_val)
      };

      let js_array = JsArray::new(&mut cx, results.len() as u32);

      results.iter().enumerate().for_each(|e| {
          let (i, v) = e;
          let js_string = cx.string(v);
          let _ = js_array.set(&mut cx, i as u32, js_string);
      });

      Ok(js_array.upcast())
    }
  }
}

register_module!(mut m, {
  m.export_class::<JsPermGen>("PermGen");
  m.export_class::<JsMinHash>("MinHash");
  m.export_class::<JsLshIndex>("LshIndex");
  Ok(())
});