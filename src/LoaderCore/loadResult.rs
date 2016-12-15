pub use super::Parsing::Parsed;
pub struct LoadResult{
   pub succeded : bool,
   pub uri : String,
   pub parsed : Parsed,
}

//TODO make really Send and Sync
unsafe impl Send for LoadResult{}
unsafe impl Sync for LoadResult{}