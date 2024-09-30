use std::marker::PhantomData;

use common_simple_types::commit_id::CommitId;

#[derive(Clone, Debug)]
pub struct CommitInfo<U> {
    pub commit_id: CommitId,
    pub next_commit_id: CommitId,
    pub _phantom: PhantomData<U>,
}
