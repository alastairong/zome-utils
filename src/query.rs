//! All helper functions calling `query()`

use hdk::prelude::*;
use crate::*;


/// Return vec of typed entries of given entry type found in local source chain
pub fn get_all_typed_local<R: TryFrom<Entry>>(entry_type: EntryType)
   -> ExternResult<Vec<R>>
{
   /// Query type
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .action_type(ActionType::Create)
      .entry_type(entry_type);
   let records = query(query_args)?;
   /// Get typed for all results
   let mut typeds = Vec::new();
   for record in records {
      let typed: R = get_typed_from_record(record.clone())?;
      typeds.push(typed)
   }
   /// Done
   Ok(typeds)
}


/// Get Record at address using query()
pub fn get_local_from_eh(eh: EntryHash) -> ExternResult<Record> {
   let mut set = HashSet::with_capacity(1);
   set.insert(eh);
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .entry_hashes(set);
   let vec = query(query_args)?;
   if vec.len() != 1 {
      return error("Record not found at given EntryHash");
   }
   Ok(vec[0].clone())
}


/// Get Record at address using query()
pub fn get_local_from_ah(ah: ActionHash) -> ExternResult<Record> {
   let query_args = ChainQueryFilter::default()
      .include_entries(true);
   let maybe_vec = query(query_args);
   if let Err(err) = maybe_vec {
      return error(&format!("{:?}",err));
   }
   let vec = maybe_vec.unwrap();
   for record in vec {
      if record.action_address() == &ah {
         return Ok(record.clone());
      }
   }
   return error("Record not found at given ActionHash");
}
